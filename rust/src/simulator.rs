use crate::{
    dag::{Dag, DagError},
    decision::evaluate_all,
    model::{Authority, AuthorityId, Block, Committee, CommitteeError},
    trace::{Event, EventStatus, TRACE_SCHEMA_VERSION, Trace, invariant_report},
    wave::wave_rounds,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimulationConfig {
    pub seed: u64,
    pub stakes: Vec<u64>,
    pub slots: u32,
    pub latency_min_ms: u64,
    pub latency_max_ms: u64,
    pub round_duration_ms: u64,
    pub packet_loss: f64,
    pub crash_authorities: Vec<u32>,
    pub byzantine_authorities: Vec<u32>,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            seed: 7,
            stakes: vec![1, 1, 1, 1],
            slots: 4,
            latency_min_ms: 5,
            latency_max_ms: 40,
            round_duration_ms: 50,
            packet_loss: 0.0,
            crash_authorities: vec![],
            byzantine_authorities: vec![],
        }
    }
}

#[derive(Debug, Error)]
pub enum SimulationError {
    #[error("invalid simulation configuration: {0}")]
    Config(String),
    #[error(transparent)]
    Committee(#[from] CommitteeError),
    #[error(transparent)]
    Dag(#[from] DagError),
}

pub fn simulate(config: &SimulationConfig) -> Result<Trace, SimulationError> {
    validate_config(config)?;
    let authorities: Vec<_> = config
        .stakes
        .iter()
        .enumerate()
        .map(|(i, stake)| Authority {
            id: AuthorityId(format!("authority-{i}")),
            stake: *stake,
        })
        .collect();
    let committee = Committee::new(authorities.clone())?;
    let crashes: BTreeSet<_> = config.crash_authorities.iter().copied().collect();
    let byzantine: BTreeSet<_> = config.byzantine_authorities.iter().copied().collect();
    if crashes
        .iter()
        .chain(&byzantine)
        .any(|i| *i as usize >= authorities.len())
    {
        return Err(SimulationError::Config(
            "fault authority index is outside committee".into(),
        ));
    }
    let mut rng = ChaCha8Rng::seed_from_u64(config.seed);
    let mut blocks = Vec::new();
    let mut events = Vec::new();
    let mut local_views = vec![BTreeSet::new(); authorities.len()];
    let mut known = BTreeMap::new();
    for slot in 0..config.slots {
        let rounds = wave_rounds(slot).map_err(|e| SimulationError::Config(e.to_string()))?;
        let base = rounds.proposal;
        let wave_start = u64::from(base)
            .checked_mul(config.round_duration_ms)
            .ok_or_else(|| SimulationError::Config("wave schedule timestamp overflow".into()))?;
        let support_time = wave_start
            .checked_add(config.round_duration_ms)
            .ok_or_else(|| SimulationError::Config("support schedule timestamp overflow".into()))?;
        let certificate_time = support_time
            .checked_add(config.round_duration_ms)
            .ok_or_else(|| {
                SimulationError::Config("certificate schedule timestamp overflow".into())
            })?;
        let leader = slot as usize % authorities.len();
        if !crashes.contains(&(leader as u32)) {
            let variants = if byzantine.contains(&(leader as u32)) {
                2
            } else {
                1
            };
            for variant in 0..variants {
                let block = Block::new(authorities[leader].id.clone(), base, slot, variant, vec![]);
                schedule_broadcast(
                    &mut rng,
                    config,
                    &block,
                    wave_start,
                    &authorities,
                    &mut events,
                )?;
                known.insert(block.id.clone(), block.clone());
                blocks.push(block);
            }
        }
        deliver_before(support_time, &events, &mut local_views);
        for (i, a) in authorities.iter().enumerate() {
            if crashes.contains(&(i as u32)) {
                continue;
            }
            let available: Vec<_> = local_views[i]
                .iter()
                .filter(|id| {
                    known
                        .get(*id)
                        .is_some_and(|b| b.slot == slot && b.round == base)
                })
                .cloned()
                .collect();
            if available.is_empty() {
                continue;
            }
            let parent_sets: Vec<Vec<_>> = if byzantine.contains(&(i as u32)) && available.len() > 1
            {
                available.iter().map(|id| vec![id.clone()]).collect()
            } else {
                vec![available]
            };
            for (variant, parents) in parent_sets.into_iter().enumerate() {
                let block = Block::new(a.id.clone(), rounds.support, slot, variant as u32, parents);
                schedule_broadcast(
                    &mut rng,
                    config,
                    &block,
                    support_time,
                    &authorities,
                    &mut events,
                )?;
                known.insert(block.id.clone(), block.clone());
                blocks.push(block);
            }
        }
        deliver_before(certificate_time, &events, &mut local_views);
        for (i, a) in authorities.iter().enumerate() {
            if crashes.contains(&(i as u32)) {
                continue;
            }
            let support_parents: Vec<_> = local_views[i]
                .iter()
                .filter(|id| {
                    known
                        .get(*id)
                        .is_some_and(|b| b.slot == slot && b.round == rounds.support)
                })
                .cloned()
                .collect();
            if support_parents.is_empty() {
                continue;
            }
            let variants = if byzantine.contains(&(i as u32)) {
                2
            } else {
                1
            };
            for variant in 0..variants {
                let block = Block::new(
                    a.id.clone(),
                    rounds.certificate,
                    slot,
                    variant,
                    support_parents.clone(),
                );
                schedule_broadcast(
                    &mut rng,
                    config,
                    &block,
                    certificate_time,
                    &authorities,
                    &mut events,
                )?;
                known.insert(block.id.clone(), block.clone());
                blocks.push(block);
            }
        }
    }
    events.sort_by(|a, b| {
        (a.outcome_at_ms, &a.block_id, &a.receiver).cmp(&(
            b.outcome_at_ms,
            &b.block_id,
            &b.receiver,
        ))
    });
    for (sequence, event) in events.iter_mut().enumerate() {
        event.sequence = sequence as u64;
    }
    let dag = Dag::build(&committee, &blocks)?;
    let decisions = evaluate_all(&committee, &dag, config.slots)
        .map_err(|e| SimulationError::Config(e.to_string()))?;
    let invariants = invariant_report(&committee, &dag, &decisions, config);
    Ok(Trace {
        schema_version: TRACE_SCHEMA_VERSION.into(),
        scope: "research/engineering Mysticeti-style digital twin; not a production Sui validator"
            .into(),
        config: config.clone(),
        seed: config.seed,
        committee: authorities,
        blocks,
        events,
        decisions,
        invariants,
        evidence_labels: vec![
            "SIMULATED".into(),
            "REPLAYABLE".into(),
            "NOT_PRODUCTION_CONSENSUS".into(),
        ],
    })
}

pub(crate) fn validate_config(config: &SimulationConfig) -> Result<(), SimulationError> {
    if config.slots == 0 {
        return Err(SimulationError::Config("slots must be positive".into()));
    }
    if !(0.0..=1.0).contains(&config.packet_loss) {
        return Err(SimulationError::Config(
            "packet loss must be in [0,1]".into(),
        ));
    }
    if config.latency_min_ms > config.latency_max_ms {
        return Err(SimulationError::Config(
            "minimum latency exceeds maximum".into(),
        ));
    }
    if config.round_duration_ms == 0 {
        return Err(SimulationError::Config(
            "round duration must be positive".into(),
        ));
    }
    let rounds =
        wave_rounds(config.slots - 1).map_err(|e| SimulationError::Config(e.to_string()))?;
    u64::from(rounds.certificate)
        .checked_mul(config.round_duration_ms)
        .and_then(|time| time.checked_add(config.latency_max_ms))
        .ok_or_else(|| {
            SimulationError::Config("complete simulation time horizon overflows".into())
        })?;
    Ok(())
}

fn schedule_broadcast(
    rng: &mut ChaCha8Rng,
    config: &SimulationConfig,
    block: &Block,
    scheduled_at_ms: u64,
    authorities: &[Authority],
    events: &mut Vec<Event>,
) -> Result<(), SimulationError> {
    for receiver in authorities {
        let local = receiver.id == block.author;
        let latency = if local {
            0
        } else {
            rng.random_range(config.latency_min_ms..=config.latency_max_ms)
        };
        let dropped = !local && rng.random_bool(config.packet_loss);
        events.push(Event {
            sequence: 0,
            scheduled_at_ms,
            outcome_at_ms: scheduled_at_ms.checked_add(latency).ok_or_else(|| {
                SimulationError::Config("network outcome timestamp overflow".into())
            })?,
            status: if dropped {
                EventStatus::Dropped
            } else {
                EventStatus::Delivered
            },
            block_id: block.id.clone(),
            sender: block.author.clone(),
            receiver: receiver.id.clone(),
        });
    }
    Ok(())
}

fn deliver_before(deadline: u64, events: &[Event], local_views: &mut [BTreeSet<String>]) {
    for event in events
        .iter()
        .filter(|e| e.status == EventStatus::Delivered && e.outcome_at_ms <= deadline)
    {
        if let Some(index) = event
            .receiver
            .0
            .strip_prefix("authority-")
            .and_then(|v| v.parse::<usize>().ok())
        {
            local_views[index].insert(event.block_id.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decision::DecisionKind;

    #[test]
    fn crashed_slot_zero_leader_produces_explicit_complete_skip_coverage() {
        let config = SimulationConfig {
            slots: 4,
            crash_authorities: vec![0],
            ..Default::default()
        };
        let trace = simulate(&config).unwrap();
        for slot in 0..config.slots {
            let proposals = trace.blocks.iter().filter(|b| {
                b.slot == slot && wave_rounds(slot).is_ok_and(|r| b.round == r.proposal)
            });
            let proposal_count = proposals.count();
            assert_eq!(
                trace.decisions.iter().filter(|d| d.slot == slot).count(),
                proposal_count.max(1)
            );
        }
        let slot_zero = trace.decisions.iter().find(|d| d.slot == 0).unwrap();
        assert_eq!(slot_zero.kind, DecisionKind::Skip);
        assert!(slot_zero.proposal_id.is_none());
    }

    #[test]
    fn overflowing_round_and_time_horizons_are_rejected() {
        assert!(
            simulate(&SimulationConfig {
                slots: u32::MAX,
                ..Default::default()
            })
            .is_err()
        );
        assert!(
            simulate(&SimulationConfig {
                round_duration_ms: u64::MAX,
                ..Default::default()
            })
            .is_err()
        );
    }
}

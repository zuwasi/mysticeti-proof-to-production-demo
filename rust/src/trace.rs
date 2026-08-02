use crate::{
    dag::{Dag, DagError},
    decision::{Decision, DecisionKind, evaluate_all},
    model::{Authority, AuthorityId, BlockId, Committee, CommitteeError},
    simulator::{SimulationConfig, simulate, validate_config},
    wave::wave_rounds,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};
use thiserror::Error;

pub const TRACE_SCHEMA_VERSION: &str = "mysticeti-twin.trace.v1";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventStatus {
    Delivered,
    Dropped,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Event {
    pub sequence: u64,
    pub scheduled_at_ms: u64,
    pub outcome_at_ms: u64,
    pub status: EventStatus,
    pub block_id: String,
    pub sender: crate::model::AuthorityId,
    pub receiver: crate::model::AuthorityId,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InvariantCheck {
    pub name: String,
    pub passed: bool,
    pub detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Assumption {
    pub name: String,
    pub satisfied: bool,
    pub detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InvariantReport {
    pub assumptions: Vec<Assumption>,
    pub checks: Vec<InvariantCheck>,
    pub all_passed: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Trace {
    pub schema_version: String,
    pub scope: String,
    pub config: SimulationConfig,
    pub seed: u64,
    pub committee: Vec<Authority>,
    pub blocks: Vec<crate::model::Block>,
    pub events: Vec<Event>,
    pub decisions: Vec<Decision>,
    pub invariants: InvariantReport,
    pub evidence_labels: Vec<String>,
}

#[derive(Debug, Error)]
pub enum VerifyError {
    #[error("trace I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("malformed trace JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("unsupported trace schema {0}")]
    Schema(String),
    #[error("invalid committee: {0}")]
    Committee(#[from] CommitteeError),
    #[error("invalid DAG: {0}")]
    Dag(#[from] DagError),
    #[error("recorded decisions differ from replay")]
    DecisionMismatch,
    #[error("trace invariant violation: {0}")]
    Invariant(String),
    #[error("trace differs from deterministic simulation: {0}")]
    Provenance(&'static str),
}

/// Diverse-assurance implementation: this intentionally duplicates the small
/// decision derivation instead of calling `evaluate_slot`/`evaluate_all`, so a
/// defect in the production evaluator is not automatically repeated by audit.
pub fn audit_decisions(
    committee: &Committee,
    dag: &Dag,
    decisions: &[Decision],
    configured_slots: u32,
) -> bool {
    let mut expected = Vec::new();
    for slot in 0..configured_slots {
        let Ok(rounds) = wave_rounds(slot) else {
            return false;
        };
        let proposals: Vec<_> = dag
            .blocks()
            .filter(|block| block.slot == slot && block.round == rounds.proposal)
            .collect();
        if proposals.is_empty() {
            expected.push((
                slot,
                None,
                rounds.proposal,
                DecisionKind::Skip,
                Vec::new(),
                Vec::new(),
                0,
                0,
            ));
            continue;
        }
        for proposal in proposals {
            let selected = |support: &&crate::model::Block| -> Option<BlockId> {
                support
                    .parents
                    .iter()
                    .filter_map(|id| dag.get(id))
                    .filter(|parent| parent.slot == slot && parent.round == rounds.proposal)
                    .map(|parent| parent.id.clone())
                    .min()
            };
            let support_authors: BTreeSet<AuthorityId> = dag
                .blocks()
                .filter(|block| block.slot == slot && block.round == rounds.support)
                .filter(|support| selected(support).as_ref() == Some(&proposal.id))
                .map(|support| support.author.clone())
                .collect();
            let certificate_authors: BTreeSet<AuthorityId> = dag
                .blocks()
                .filter(|block| block.slot == slot && block.round == rounds.certificate)
                .filter(|certificate| {
                    let parent_authors: BTreeSet<AuthorityId> = certificate
                        .parents
                        .iter()
                        .filter_map(|id| dag.get(id))
                        .filter(|support| {
                            support.slot == slot
                                && support.round == rounds.support
                                && selected(support).as_ref() == Some(&proposal.id)
                        })
                        .map(|support| support.author.clone())
                        .collect();
                    committee.distinct_stake(parent_authors.iter()) >= committee.quorum_threshold()
                })
                .map(|certificate| certificate.author.clone())
                .collect();
            let support_stake = committee.distinct_stake(support_authors.iter());
            let certificate_stake = committee.distinct_stake(certificate_authors.iter());
            let kind = if certificate_stake >= committee.quorum_threshold() {
                DecisionKind::Commit
            } else {
                DecisionKind::NoDecision
            };
            expected.push((
                slot,
                Some(proposal.id.clone()),
                rounds.proposal,
                kind,
                support_authors.into_iter().collect(),
                certificate_authors.into_iter().collect(),
                support_stake,
                certificate_stake,
            ));
        }
    }
    decisions.len() == expected.len()
        && decisions.iter().zip(expected).all(|(actual, expected)| {
            actual.slot == expected.0
                && actual.proposal_id == expected.1
                && actual.proposal_round == expected.2
                && actual.kind == expected.3
                && actual.support_authors == expected.4
                && actual.certificate_authors == expected.5
                && actual.support_stake == expected.6
                && actual.certificate_author_stake == expected.7
        })
}

pub fn invariant_report(
    committee: &Committee,
    dag: &Dag,
    decisions: &[Decision],
    config: &SimulationConfig,
) -> InvariantReport {
    let commits: Vec<_> = decisions
        .iter()
        .filter(|d| d.kind == DecisionKind::Commit)
        .collect();
    let mut per_slot: BTreeMap<u32, BTreeSet<_>> = BTreeMap::new();
    for d in &commits {
        if let Some(id) = &d.proposal_id {
            per_slot.entry(d.slot).or_default().insert(id);
        }
    }
    let no_conflicting_commits = per_slot.values().all(|ids| ids.len() <= 1);
    let evidence_valid = audit_decisions(committee, dag, decisions, config.slots);
    let commit_quorum = commits
        .iter()
        .all(|d| d.certificate_author_stake >= committee.quorum_threshold() && evidence_valid);
    let certified: Vec<_> = decisions
        .iter()
        .filter(|d| d.support_stake >= committee.quorum_threshold())
        .collect();
    let mut certified_slots: BTreeMap<u32, BTreeSet<_>> = BTreeMap::new();
    for d in certified {
        if let Some(id) = &d.proposal_id {
            certified_slots.entry(d.slot).or_default().insert(id);
        }
    }
    let no_conflicting_certificates = certified_slots.values().all(|ids| ids.len() <= 1);
    let byzantine_ids: BTreeSet<_> = config
        .byzantine_authorities
        .iter()
        .map(|i| crate::model::AuthorityId(format!("authority-{i}")))
        .collect();
    let byzantine_stake = committee.distinct_stake(byzantine_ids.iter());
    let byzantine_safe = byzantine_stake <= committee.byzantine_safety_bound();
    let honest_single_support = dag
        .blocks()
        .filter(|b| wave_rounds(b.slot).is_ok_and(|rounds| b.round == rounds.support))
        .filter(|b| !byzantine_ids.contains(&b.author))
        .map(|b| (&b.author, b.slot))
        .collect::<Vec<_>>();
    let honest_single_support =
        honest_single_support.iter().collect::<BTreeSet<_>>().len() == honest_single_support.len();
    let checks = vec![
        InvariantCheck {
            name: "no_conflicting_certified_proposals".into(),
            passed: no_conflicting_certificates,
            detail: "under honest single-support assumption".into(),
        },
        InvariantCheck {
            name: "no_conflicting_direct_commits".into(),
            passed: no_conflicting_commits,
            detail: "at most one committed proposal per slot".into(),
        },
        InvariantCheck {
            name: "commit_has_certificate_author_quorum".into(),
            passed: commit_quorum,
            detail: format!("threshold={}", committee.quorum_threshold()),
        },
        InvariantCheck {
            name: "equivocation_stake_deduplicated".into(),
            passed: evidence_valid,
            detail: "explicit author lists are sorted, unique, and stake sums recompute".into(),
        },
        InvariantCheck {
            name: "dag_validity".into(),
            passed: true,
            detail: "full Dag::build validation succeeded".into(),
        },
    ];
    let assumptions = vec![
        Assumption {
            name: "honest_single_support".into(),
            satisfied: honest_single_support,
            detail: "generated honest authorities have at most one support block per slot".into(),
        },
        Assumption {
            name: "byzantine_stake_below_one_third".into(),
            satisfied: byzantine_safe,
            detail: format!(
                "configured_byzantine_stake={byzantine_stake}, total_stake={}",
                committee.total_stake()
            ),
        },
        Assumption {
            name: "educational_skip_scope".into(),
            satisfied: true,
            detail: "skip does not model production indirect decisions".into(),
        },
    ];
    let all_passed = checks.iter().all(|c| c.passed)
        && assumptions.iter().all(|assumption| assumption.satisfied);
    InvariantReport {
        assumptions,
        checks,
        all_passed,
    }
}

pub fn verify_trace(trace: &Trace) -> Result<Vec<Decision>, VerifyError> {
    if trace.schema_version != TRACE_SCHEMA_VERSION {
        return Err(VerifyError::Schema(trace.schema_version.clone()));
    }
    if trace.seed != trace.config.seed {
        return Err(VerifyError::Invariant(
            "top-level seed differs from config seed".into(),
        ));
    }
    validate_config(&trace.config).map_err(|error| VerifyError::Invariant(error.to_string()))?;
    let committee = Committee::new(trace.committee.clone())?;
    let expected_committee: Vec<_> = trace
        .config
        .stakes
        .iter()
        .enumerate()
        .map(|(i, stake)| Authority {
            id: crate::model::AuthorityId(format!("authority-{i}")),
            stake: *stake,
        })
        .collect();
    if trace.committee != expected_committee {
        return Err(VerifyError::Invariant(
            "config stakes differ from committee".into(),
        ));
    }
    let ids: BTreeSet<_> = trace.committee.iter().map(|a| &a.id).collect();
    let block_ids: BTreeSet<_> = trace.blocks.iter().map(|b| &b.id).collect();
    let block_authors: BTreeMap<_, _> = trace.blocks.iter().map(|b| (&b.id, &b.author)).collect();
    for (index, event) in trace.events.iter().enumerate() {
        if event.sequence != index as u64
            || event.outcome_at_ms < event.scheduled_at_ms
            || index > 0 && trace.events[index - 1].outcome_at_ms > event.outcome_at_ms
            || !ids.contains(&event.sender)
            || !ids.contains(&event.receiver)
            || !block_ids.contains(&event.block_id)
            || block_authors.get(&event.block_id) != Some(&&event.sender)
        {
            return Err(VerifyError::Invariant(
                "invalid event reference, ordering, or fields".into(),
            ));
        }
    }
    let dag = Dag::build(&committee, &trace.blocks)?;
    let decisions = evaluate_all(&committee, &dag, trace.config.slots)
        .map_err(|e| VerifyError::Invariant(e.to_string()))?;
    if decisions != trace.decisions {
        return Err(VerifyError::DecisionMismatch);
    }
    let report = invariant_report(&committee, &dag, &decisions, &trace.config);
    if !report.all_passed || report.assumptions.iter().any(|a| !a.satisfied) {
        return Err(VerifyError::Invariant(
            "recomputed safety checks failed".into(),
        ));
    }
    if report != trace.invariants {
        return Err(VerifyError::Invariant(
            "recorded invariant report differs from replay".into(),
        ));
    }
    let generated =
        simulate(&trace.config).map_err(|_| VerifyError::Provenance("simulation failed"))?;
    if generated.committee != trace.committee {
        return Err(VerifyError::Provenance("committee"));
    }
    if generated.blocks != trace.blocks {
        return Err(VerifyError::Provenance("blocks"));
    }
    if generated.events != trace.events {
        return Err(VerifyError::Provenance("events"));
    }
    if generated.decisions != trace.decisions {
        return Err(VerifyError::Provenance("decisions"));
    }
    if generated.scope != trace.scope {
        return Err(VerifyError::Provenance("scope"));
    }
    if generated.evidence_labels != trace.evidence_labels {
        return Err(VerifyError::Provenance("labels"));
    }
    if generated.invariants != trace.invariants {
        return Err(VerifyError::Provenance("invariants"));
    }
    Ok(decisions)
}

pub fn load_and_verify(path: impl AsRef<Path>) -> Result<Trace, VerifyError> {
    let trace: Trace = serde_json::from_slice(&fs::read(path)?)?;
    verify_trace(&trace)?;
    Ok(trace)
}

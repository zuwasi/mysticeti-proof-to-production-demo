use crate::{SimulationConfig, SimulationError, decision::DecisionKind, simulate};
use rayon::prelude::*;
use thiserror::Error;

const PACKET_LOSS_VALUES: [f64; 4] = [0.0, 0.05, 0.15, 0.30];
const CSV_HEADER: &str = "seed,packet_loss,blocks,commits,all_invariants_passed\n";

#[derive(Clone, Debug, PartialEq)]
pub struct CampaignRow {
    pub seed: u64,
    pub packet_loss: f64,
    pub blocks: usize,
    pub commits: usize,
    pub all_invariants_passed: bool,
}

#[derive(Debug, Error)]
pub enum CampaignError {
    #[error("campaign worker count must be nonzero")]
    ZeroWorkers,
    #[error("failed to build campaign thread pool: {0}")]
    ThreadPool(#[from] rayon::ThreadPoolBuildError),
    #[error(transparent)]
    Simulation(#[from] SimulationError),
}

/// Runs the canonical seed × packet-loss campaign.
///
/// Every row is produced by an independent deterministic `simulate` call. A
/// dedicated Rayon pool is used only when `workers > 1`; the global pool is
/// never configured or mutated. Returned rows are sorted by seed, then loss.
pub fn run_fault_campaign(seeds: u32, workers: usize) -> Result<Vec<CampaignRow>, CampaignError> {
    if workers == 0 {
        return Err(CampaignError::ZeroWorkers);
    }

    let scenarios: Vec<_> = (0..seeds)
        .flat_map(|seed| {
            PACKET_LOSS_VALUES
                .into_iter()
                .map(move |loss| (u64::from(seed), loss))
        })
        .collect();
    let run = || {
        scenarios
            .par_iter()
            .map(|&(seed, packet_loss)| run_scenario(seed, packet_loss))
            .collect::<Result<Vec<_>, _>>()
    };
    let mut rows = if workers == 1 {
        scenarios
            .iter()
            .map(|&(seed, packet_loss)| run_scenario(seed, packet_loss))
            .collect::<Result<Vec<_>, _>>()?
    } else {
        rayon::ThreadPoolBuilder::new()
            .num_threads(workers)
            .build()?
            .install(run)?
    };
    rows.sort_by(|a, b| {
        a.seed
            .cmp(&b.seed)
            .then_with(|| a.packet_loss.total_cmp(&b.packet_loss))
    });
    Ok(rows)
}

pub fn campaign_csv(rows: &[CampaignRow]) -> String {
    let mut csv = String::from(CSV_HEADER);
    for row in rows {
        csv.push_str(&format!(
            "{},{:.2},{},{},{}\n",
            row.seed, row.packet_loss, row.blocks, row.commits, row.all_invariants_passed
        ));
    }
    csv
}

fn run_scenario(seed: u64, packet_loss: f64) -> Result<CampaignRow, SimulationError> {
    let trace = simulate(&SimulationConfig {
        seed,
        packet_loss,
        slots: 8,
        ..Default::default()
    })?;
    Ok(CampaignRow {
        seed,
        packet_loss,
        blocks: trace.blocks.len(),
        commits: trace
            .decisions
            .iter()
            .filter(|decision| decision.kind == DecisionKind::Commit)
            .count(),
        all_invariants_passed: trace.invariants.all_passed,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn csv(seeds: u32, workers: usize) -> String {
        campaign_csv(&run_fault_campaign(seeds, workers).unwrap())
    }

    #[test]
    fn sequential_and_parallel_csv_are_byte_identical() {
        assert_eq!(csv(4, 1).as_bytes(), csv(4, 8).as_bytes());
    }

    #[test]
    fn repeated_parallel_csv_is_deterministic() {
        assert_eq!(csv(4, 8).as_bytes(), csv(4, 8).as_bytes());
    }

    #[test]
    fn zero_workers_is_rejected() {
        assert!(matches!(
            run_fault_campaign(1, 0),
            Err(CampaignError::ZeroWorkers)
        ));
    }

    #[test]
    fn rows_are_canonically_ordered() {
        let rows = run_fault_campaign(3, 8).unwrap();
        assert!(rows.windows(2).all(|pair| {
            (pair[0].seed, pair[0].packet_loss) < (pair[1].seed, pair[1].packet_loss)
        }));
    }

    #[test]
    fn all_invariant_statuses_are_preserved() {
        let rows = run_fault_campaign(4, 8).unwrap();
        assert!(rows.iter().all(|row| row.all_invariants_passed));
    }
}

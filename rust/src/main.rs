#![forbid(unsafe_code)]
use clap::{Parser, Subcommand};
use mysticeti_twin::{
    SimulationConfig, campaign_csv, load_and_verify, run_fault_campaign, simulate, verify_trace,
};
use std::{
    fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

#[derive(Parser)]
#[command(
    name = "mysticeti-twin",
    about = "Deterministic stake-weighted Mysticeti-style research twin (not a production Sui validator)"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run a seeded simulation and write a replayable JSON trace.
    Simulate {
        #[arg(long, default_value_t = 7)]
        seed: u64,
        #[arg(long, value_delimiter = ',', default_value = "1,1,1,1")]
        stakes: Vec<u64>,
        #[arg(long, default_value_t = 4)]
        slots: u32,
        #[arg(long, default_value_t = 0.0)]
        packet_loss: f64,
        #[arg(long, default_value_t = 50)]
        round_duration_ms: u64,
        #[arg(long, value_delimiter = ',')]
        crash: Vec<u32>,
        #[arg(long, value_delimiter = ',')]
        byzantine: Vec<u32>,
        #[arg(long)]
        output: PathBuf,
    },
    /// Replay blocks and print the recomputed decisions.
    Replay { trace: PathBuf },
    /// Strictly validate schema, DAG, decisions, and invariants.
    Verify { trace: PathBuf },
    /// Run a deterministic packet-loss/seed campaign and emit CSV.
    Sweep {
        #[arg(long)]
        output: PathBuf,
        #[arg(long, default_value_t = 8)]
        seeds: u32,
        /// Number of independent campaign scenarios to run concurrently.
        #[arg(long, default_value_t = default_jobs(), value_parser = parse_jobs)]
        jobs: usize,
    },
    /// Generate the documented deterministic demonstration trace.
    Demo {
        #[arg(long)]
        output: PathBuf,
    },
}

fn write_trace(path: &Path, config: SimulationConfig) -> Result<(), Box<dyn std::error::Error>> {
    let trace = simulate(&config)?;
    verify_trace(&trace)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, serde_json::to_vec_pretty(&trace)?)?;
    println!(
        "wrote {} blocks and {} decisions to {}",
        trace.blocks.len(),
        trace.decisions.len(),
        path.display()
    );
    Ok(())
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    match Cli::parse().command {
        Command::Simulate {
            seed,
            stakes,
            slots,
            packet_loss,
            round_duration_ms,
            crash,
            byzantine,
            output,
        } => write_trace(
            &output,
            SimulationConfig {
                seed,
                stakes,
                slots,
                packet_loss,
                round_duration_ms,
                crash_authorities: crash,
                byzantine_authorities: byzantine,
                ..Default::default()
            },
        ),
        Command::Demo { output } => write_trace(
            &output,
            SimulationConfig {
                seed: 2026,
                stakes: vec![4, 3, 2, 1],
                slots: 6,
                byzantine_authorities: vec![3],
                ..Default::default()
            },
        ),
        Command::Replay { trace } => {
            let t = load_and_verify(trace)?;
            println!("{}", serde_json::to_string_pretty(&t.decisions)?);
            Ok(())
        }
        Command::Verify { trace } => {
            let t = load_and_verify(&trace)?;
            println!(
                "verified {}: {} blocks; all invariants passed",
                trace.display(),
                t.blocks.len()
            );
            Ok(())
        }
        Command::Sweep {
            output,
            seeds,
            jobs,
        } => {
            if let Some(parent) = output.parent() {
                fs::create_dir_all(parent)?;
            }
            let rows = run_fault_campaign(seeds, jobs)?;
            fs::write(&output, campaign_csv(&rows))?;
            println!(
                "wrote deterministic sweep using {jobs} job(s) to {}",
                output.display()
            );
            Ok(())
        }
    }
}

fn default_jobs() -> usize {
    std::thread::available_parallelism().map_or(1, usize::from)
}

fn parse_jobs(value: &str) -> Result<usize, String> {
    match value.parse::<usize>() {
        Ok(0) => Err("jobs must be nonzero".into()),
        Ok(jobs) => Ok(jobs),
        Err(error) => Err(error.to_string()),
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

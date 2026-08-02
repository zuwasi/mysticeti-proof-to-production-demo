#![forbid(unsafe_code)]
//! Deterministic, stake-weighted Mysticeti-style research digital twin.
//!
//! This crate is an educational engineering reference, not production Sui consensus.

pub mod campaign;
pub mod dag;
pub mod decision;
pub mod model;
pub mod simulator;
pub mod trace;
pub mod wave;

pub use campaign::{CampaignError, CampaignRow, campaign_csv, run_fault_campaign};
pub use simulator::{SimulationConfig, SimulationError, simulate};
pub use trace::{Trace, load_and_verify, verify_trace};

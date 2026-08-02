# Fifteen-minute developer demo script

## 0:00–1:00 — Why this protocol

Mysticeti is not a hypothetical consensus paper: Sui Mainnet validators switched to Mysticeti-C on July 25, 2024. The question is how to examine a production-relevant protocol without confusing proof, simulation, and benchmark claims.

## 1:00-4:00 - Live Rust simulation, replay, and tamper resistance

Run `cargo run --release -- demo --output ..\exports\rust_demo_trace.json`, strict `verify`, and `replay`. Show schema version, weighted committee, events, blocks, and decisions. Copy the trace, alter a block ID, and show strict verification reject it; do not replace the release trace.

Then show `exports/wsl_rust_build_report.txt`: Windows and Ubuntu WSL generate the same trace SHA-256, while one-worker and eight-worker campaigns generate the same canonical CSV SHA-256. The concurrency is across independent scenarios; the consensus state machine itself remains deterministic and single-scenario.

- validator lanes and rounds;
- one Byzantine authority and visible equivocations;
- authority-distinct parent selection;
- explicit evidence labels.

Message: **more blocks do not mean more voting power**.

## 4:00-7:00 - Packet-loss campaign and Mathematica plot

Run the 20-seed sweep with `--jobs 1` and `--jobs 8`, then byte-compare `exports/rust_fault_sweep.csv` and `exports/rust_fault_sweep_parallel.csv`. Show the equal SHA-256 values in `rust_build_report.txt`, then evaluate `RustFaultSweepPlot["exports/rust_fault_sweep.csv"]`. The dedicated Rayon pool executes independent scenarios on real worker threads; canonical seed/loss ordering makes both CSV files byte-identical. Each scenario's consensus state machine remains deterministic and single-scenario event-driven, so this is concurrency/reproducibility evidence rather than a production Sui concurrency claim.

- supporter authorities at \(r+1\);
- certificate block IDs and authorities at \(r+2\);
- final `Commit`, `Skip`, or `Undecided` decision.

Message: **a certificate pattern is one layer; direct commit needs a quorum of certificate blocks**.

## 7:00-10:00 - Inspect certificate versus commit

Use the existing Mathematica DAG/fixtures and vary the Safety Microscope. Contrast one certificate with quorum certificate-author evidence. Show `ValidateRustTrace` passing its independent schema/reference/stake checks.

- fault bound;
- crashed validators;
- rounds;
- equivocation;
- random seed.

Ask the audience to predict whether the slot commits, skips, or remains undecided before changing a control.

## 10:00-12:00 - Show the Lean proof boundary

Open `docs/plain_english_proof.md`, then show the theorem names in `lean/MysticetiProofs/Safety.lean`.

Run:

```powershell
Set-Location .\lean
lake build
```

Message: Lean proves the general threshold statements with no `sorry`; it does not certify the entire notebook or Sui implementation.

## 12:00-13:00 - Practical Sui adapter path

Show the Table I CSV comparison and explain:

- those bars are paper-reported values;
- they are not simulator output;
- synthetic timing is clearly labeled and uncalibrated;
- production Sui uses stake-weighted voting.

Explain that a future adapter would translate Sui-compatible committee/block/event data into the versioned trace boundary while preserving provenance. It would not make this twin a validator.

## 13:00-15:00 - Run the three-lane evidence gate

```powershell
.\build_all.ps1
```

Open `exports/combined_validation_report.md` and `docs/formalization_map.md`.

Closing message:

> Amp preserves traceability across Rust execution, Mathematica conformance, Lean theorem checking, and production interpretation-with the evidence boundaries visible rather than hidden.

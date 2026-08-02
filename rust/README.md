# Mysticeti Consensus Digital Twin — Rust reference

This crate is a deterministic, stake-weighted **research and engineering digital twin** inspired by Mysticeti's three-round direct-decision structure. It is intended for protocol exploration, reproducible fault campaigns, evidence generation, and as a practical bridge from the project's Mathematica model and Lean properties toward Sui-oriented engineering.

It is **not a production Sui validator**, not wire-compatible with Sui, not a replacement for Mysticeti/Sui consensus code, and not a claim to implement production indirect decisions, epoch changes, persistence, cryptography, networking, transaction execution, or validator operations.

## Architecture and semantics

- `model`: authority IDs and positive stake, validated committees, total stake, strict Byzantine safety bound, quorum `floor(2T/3)+1`, canonical SHA-256 block IDs.
- `dag`: validates committee membership, canonical IDs and sorted/unique parents, known parents, the exact proposal/support/certificate rounds `3s/3s+1/3s+2`, same-slot adjacent-phase parent links, and unique `(author, round, slot, variant)` identities. Insert validation is transactional.
- `decision`: each support block selects exactly one directly referenced leader proposal using the smallest canonical block ID. Certificate qualification uses only same-slot canonical supports selected for that proposal. Support and certificate author lists are explicit, sorted, deduplicated audit evidence. **Direct commit requires quorum stake among distinct authors of qualifying `r+2` blocks.**
- `simulator`: a seeded, deterministic, receiver-specific event queue. Every broadcast records sender, receiver, schedule/outcome time, and delivered/dropped status. At each configurable round deadline an authority builds from only its local delivered view. Honest authorities produce one support block; Byzantine authorities may emit separate opposing variants. Latency and loss therefore alter parents and outcomes rather than serving as labels.
- `trace`: versioned JSON, strict deserialization, replay from recorded blocks, decision comparison, and invariant recomputation.
- `campaign`: a seed × packet-loss scenario API backed by a dedicated Rayon thread pool. Campaign scenarios run concurrently, but each scenario remains an independent deterministic `simulate` call and results are sorted by seed then packet loss before CSV serialization.

The educational skip rule is: if no round-`r` proposal exists for a slot when the three-round wave is evaluated, emit `skip`; an observed proposal lacking direct-commit evidence emits `no_decision`. This is useful and explicit, but **does not claim production Mysticeti indirect-decision semantics**.

## Commands

```powershell
cargo run --release -- demo --output artifacts\rust_demo_trace.json
cargo run --release -- simulate --seed 99 --stakes 4,3,2,1 --slots 10 --round-duration-ms 50 --packet-loss 0.1 --byzantine 3 --output artifacts\trace.json
cargo run --release -- replay artifacts\trace.json
cargo run --release -- verify artifacts\trace.json
cargo run --release -- sweep --seeds 20 --jobs 8 --output artifacts\rust_fault_sweep.csv
```

Use `cargo run -- --help` and each subcommand's `--help` for the complete interface.

### Concurrency boundary

`sweep --jobs N` parallelizes independent campaign scenarios, not the consensus state machine. It uses a campaign-owned Rayon `ThreadPool` rather than mutating Rayon's global pool; `--jobs 0` is rejected. Since each scenario is seeded and event-driven and rows are canonically sorted before serialization, the same inputs produce byte-identical CSV with `--jobs 1` and `--jobs 8`. This is concrete host-concurrency and reproducibility evidence for the research twin. It does **not** claim concurrency, scheduling, networking, builder-relay, or production equivalence with Sui/Mysticeti.

## Trace schema

`mysticeti-twin.trace.v1` contains: scope statement; complete simulation config and seed; committee; canonical blocks; ordered per-receiver events; decisions with author evidence; assumptions with status; invariant checks; and evidence labels. Strict verification both independently rebuilds the DAG/recomputes evidence and reruns `simulate(config)`, requiring exact equality of all evidence-bearing fields. Config stakes must exactly reconstruct the committee. Unknown fields at every nested domain type, malformed JSON, invalid events/blocks, tampering, and unsupported schemas are rejected.

## Evidence boundaries and invariants

Results report assumptions separately with machine-checked status. Configured Byzantine stake is computed from the committee and must be strictly below one third; honest single-support is checked from generated blocks. Strict verification rejects traces when required premises fail rather than presenting a passed safety claim. The canonical global generated DAG is campaign-level evidence evaluated after production; it is **not node-local finalization**. Output is labelled `SIMULATED`, `REPLAYABLE`, and `NOT_PRODUCTION_CONSENSUS` and makes no equivalence claim to production Sui or Mysticeti.

## Practical Sui integration path

1. Map Sui epoch committee data into `Authority` records while preserving exact stake units.
2. Add an adapter from captured/fixture consensus blocks to this canonical `Block` model; keep adapter evidence distinct from simulated evidence.
3. Differentially compare wave-level support/certificate outcomes against the matching Sui/Mysticeti implementation revision.
4. Version protocol assumptions and trace schema per upstream revision, and validate against Sui test vectors.
5. Only then consider real network timing feeds. Production cryptographic verification, storage, epoch lifecycle, recovery, and indirect decisions remain upstream responsibilities.

## Development

```powershell
cargo fmt --check
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo bench
```

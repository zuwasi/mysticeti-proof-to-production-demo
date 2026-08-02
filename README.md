# Mysticeti Proof-to-Production Demo

[![CI](https://github.com/zuwasi/mysticeti-proof-to-production-demo/actions/workflows/ci.yml/badge.svg)](https://github.com/zuwasi/mysticeti-proof-to-production-demo/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Paper DOI](https://img.shields.io/badge/DOI-10.14722%2Fndss.2025.240929-24D5FF.svg)](https://doi.org/10.14722/ndss.2025.240929)

> **What if every blockchain consensus decision came with a proof, a replay,
> and a reproducible hash?**

This repository demonstrates an Amp-orchestrated evidence workflow in which:

- **Rust** executes and deterministically replays a stake-weighted,
  event-driven Mysticeti-style research digital twin;
- **Wolfram Mathematica** explores the DAG, runs bounded fixtures, visualizes
  sensitivity, and independently audits selected Rust evidence;
- **Lean 4** kernel-checks explicitly mapped quorum and safety mathematics;
- **Amp** coordinates the lanes and fails the release gate when their evidence
  disagrees.

**Interactive presentation:**
https://zuwasi.github.io/Public-html-pages/mysticeti-consensus-digital-twin/

## Independent project and proof boundary

This is an independent, bounded research and educational demonstration. It is
not the production Sui implementation and is not affiliated with or endorsed
by Mysten Labs or Sui.

Lean proves the exact statements identified in
[`docs/formalization_map.md`](docs/formalization_map.md). It does **not** prove
the Rust binary, networking, cryptography, storage, liveness, epoch changes, or
production equivalence. Rust/Wolfram conformance is executable evidence, not a
formal refinement proof.

## Original research

The project is inspired by:

> Kushal Babel et al., "Mysticeti: Reaching the Latency Limits with
> Uncertified DAGs," NDSS 2025.

Authoritative DOI: https://doi.org/10.14722/ndss.2025.240929

See [`NOTICE.md`](NOTICE.md), [`CITATION.cff`](CITATION.cff), and
[`docs/references.md`](docs/references.md) for attribution and related sources.
The paper PDF is linked, not redistributed.

## Five-minute Rust demonstration

Prerequisite: Rust 1.97.1 or newer.

```bash
git clone https://github.com/zuwasi/mysticeti-proof-to-production-demo.git
cd mysticeti-proof-to-production-demo/rust
cargo run --release -- demo --output ../exports/rust_demo_trace.json
cargo run --release -- verify ../exports/rust_demo_trace.json
cargo run --release -- replay ../exports/rust_demo_trace.json
cargo run --release -- sweep --seeds 20 --jobs 1 --output ../exports/jobs1.csv
cargo run --release -- sweep --seeds 20 --jobs 8 --output ../exports/jobs8.csv
```

The two campaign files must be byte-identical. On Linux/macOS:

```bash
cmp ../exports/jobs1.csv ../exports/jobs8.csv
sha256sum ../exports/jobs1.csv ../exports/jobs8.csv
```

Expected campaign SHA-256:

```text
5fdc665f5c35cee5c63143860789a9a2a4db831ec65c9e814610d1f6b1764a6a
```

## Full local evidence gate

### Required tools

- Rust 1.97.1+
- Lean 4.32.2 and Lake
- Wolfram Language runtime compatible with `wolframscript`
- PowerShell 7 or Windows PowerShell 5.1 for `build_all.ps1`

From PowerShell:

```powershell
.\build_all.ps1
```

The command runs Rust, Wolfram, and Lean, verifies required artifacts, and
emits `exports/combined_validation_report.md`. The Criterion benchmark is
intentionally separate:

```bash
cd rust
cargo bench --bench simulation
```

Public CI runs Rust on Windows and Ubuntu, Lean, deterministic campaign/replay
checks, and repository-integrity checks. Mathematica is not executed on hosted
CI because a suitable Wolfram runtime/license is not assumed; its checked-in
validation snapshot is structurally checked, and the full lane is run locally.

## What the Rust twin models

- exact stake-weighted quorum arithmetic;
- validated three-round DAG construction;
- deterministic receiver-specific delivery and local views;
- latency, packet loss, crashes, and Byzantine equivocation;
- direct commit versus certificate separation;
- strict trace parsing, replay, and tamper rejection;
- independent invariant auditing;
- deterministic parallel fault campaigns using a dedicated Rayon pool and
  canonical output ordering.

It deliberately omits production networking, signatures, persistence, epoch
changes, indirect decisions, Sui object execution, and validator integration.

## Evidence hierarchy

| Evidence | Meaning |
|---|---|
| Lean theorem | Kernel-checked exact statement under explicit assumptions. |
| Rust trace | Deterministic research-twin execution, not production telemetry. |
| Wolfram fixture | Executable bounded scenario or independent evidence check. |
| Cross-language conformance | Selected structural/semantic checks, not equivalence proof. |
| Synthetic sensitivity | Explanatory uncalibrated experiment, not WAN prediction. |
| Paper transcription | Cited values copied for comparison, not reproduced measurements. |

## Reproducibility snapshot

Windows and Ubuntu WSL independently produced the same trace:

```text
6272fa854de66bc42512f38d095d7ccf6f75bb85f581dc4acaabf9fbe8ede71d
```

Windows/Ubuntu and one/eight-worker campaigns produced the same CSV:

```text
5fdc665f5c35cee5c63143860789a9a2a4db831ec65c9e814610d1f6b1764a6a
```

These hashes demonstrate deterministic evidence for the bounded twin. They do
not establish production correctness or performance.

## Repository map

```text
rust/                 Rust digital twin, CLI, tests, and benchmark
lean/                 Lean definitions and mapped proofs
src/                  Wolfram package
MysticetiLab.nb        interactive Mathematica notebook
build_project.wls      Wolfram validation/export pipeline
build_all.ps1          combined local evidence gate
data/                  cited paper-value fixtures
exports/               selected reproducible evidence snapshot
docs/                  formalization map, implications, demo, security
presentation/          PPTX, PDF, offline HTML, and portable generator
scripts/               public-repository integrity checks
```

## Presentations

- PowerPoint: `presentation/Mysticeti_Consensus_Digital_Twin_ESL.pptx`
- PDF: `presentation/rendered/Mysticeti_Consensus_Digital_Twin_ESL.pdf`
- Offline HTML: `presentation/html/index.html`
- Live HTML: https://zuwasi.github.io/Public-html-pages/mysticeti-consensus-digital-twin/

## Security, contribution, and license

- Security policy: [`SECURITY.md`](SECURITY.md)
- Public assessment: [`docs/SECURITY_ASSESSMENT.md`](docs/SECURITY_ASSESSMENT.md)
- Contributions: [`CONTRIBUTING.md`](CONTRIBUTING.md)
- License: [MIT](LICENSE)

## Commercial adaptation

The repository is an open demonstration. Protocol-specific adapters,
production-trace ingestion, formalization, CI integration, and release evidence
remain engineering engagements.

Engineering Software Lab: https://eswlab.com/

Daniel Liezrowice: https://il.linkedin.com/in/liezrowice

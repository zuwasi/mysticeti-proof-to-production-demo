# Mysticeti Combined Validation Report

- Rust lane exit code: 0
- Rust required artifacts nonempty: True
- Wolfram lane exit code: 0
- Lean build exit code: 0
- Lean placeholder scan: PASSED
- Combined release gate: PASSED

## Evidence boundaries

- Rust executes a deterministic, stake-weighted, event-driven research twin with strict replay/tamper checks; it is not production Sui.
- Wolfram independently audits the recorded Rust schema, references, stake arithmetic, and evidence, alongside its paper-specific fixtures.
- Lean kernel-checks only the exact equal-authority quorum statements mapped in docs/formalization_map.md; Lean does not prove Rust or Wolfram.
- The project does not claim complete Mysticeti safety, liveness, cryptography, epoch-change, or production-performance verification.

## Wolfram output

BUILD OK: 11 non-empty exports; Wolfram validation 12/12; Rust conformance 11/11; Rust sweep 80/80 rows valid


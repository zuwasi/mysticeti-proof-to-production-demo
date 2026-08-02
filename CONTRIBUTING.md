# Contributing

Contributions that improve correctness, reproducibility, documentation, or
the explicit evidence boundary are welcome.

## Before opening a pull request

Run:

```powershell
Set-Location rust
cargo fmt --check
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
Set-Location ..\lean
lake build
```

If you change serialized evidence, also run one-worker and eight-worker fault
campaigns and confirm that their CSV files are byte-identical. Changes to
Wolfram code should include regenerated validation evidence and state the
Wolfram version used.

## Evidence and claim discipline

- State every new assumption.
- Distinguish theorem, executable test, synthetic experiment, transcription,
  and production measurement.
- Do not describe Lean as proving the Rust implementation unless a checked
  refinement relation is added.
- Do not describe this project as the production Sui implementation.
- Add tests for behavior changes and update `docs/formalization_map.md` when a
  claim's status changes.

By contributing, you agree that your contribution is licensed under MIT.

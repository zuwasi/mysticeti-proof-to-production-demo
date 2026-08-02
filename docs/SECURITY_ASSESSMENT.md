# Public-release security assessment

Assessment date: 2026-08-02  
Assessed revision: `3f27e09efa33566cface5888097a6f6912cb6257`  
Scope: curated public repository, Rust crate, Python integrity checker, GitHub
Actions, dependency manifests, release scripts, Wolfram/Lean source, and static
presentation package.

## Executive summary

No confirmed security vulnerabilities or exposed secrets were identified in
the curated public-release tree. Endor Labs reported no findings for
vulnerabilities, secrets, dependencies, SAST, or GitHub Actions. Manual checks
confirmed that build caches and machine-specific paths are excluded, GitHub
Actions are pinned to full commit hashes, the Rust crate forbids unsafe code,
and untrusted trace JSON uses strict schemas with unknown-field rejection.

Security level for the stated research/demo scope: **Low risk**.

This rating does not make the project production consensus software. The twin
does not implement production cryptography, networking, storage, identity,
authorization, validator operations, or deployment controls, so those systems
were not assessed.

## Checks performed

| Area | Method | Result |
|---|---|---|
| Dependency vulnerabilities | Endor Labs vulnerability/dependency scan | No problems found |
| Secrets | Endor Labs secret scan plus scoped pattern review | No problems found |
| Rust/Python/JavaScript SAST | Endor Labs SAST plus manual trust-boundary review | No problems found |
| GitHub Actions | Endor Labs GitHub Actions scan and manual permissions review | No problems found |
| Rust safety posture | `#![forbid(unsafe_code)]`, tests, Clippy warnings-as-errors | Passed |
| Trace parser | Strict Serde schemas, tamper and malformed-input tests | Passed |
| Supply-chain pinning | Cargo lockfile, Lean manifest/toolchain, full-SHA CI actions | Present |
| License metadata | `cargo metadata` across 112 registry packages | All declare license metadata |
| Public-tree hygiene | `scripts/check_public_repo.py` | Passed |
| Presentation HTML | Local assets, CSP, safe external-link attributes | Passed |

## Findings

No reportable Critical, High, Medium, Low, or Informational security findings
remain open at this revision.

## Positive controls

- Rust library and CLI use `#![forbid(unsafe_code)]`.
- Domain and trace structures use `#[serde(deny_unknown_fields)]`.
- Strict verification rebuilds and recomputes evidence rather than trusting
  reported invariant flags.
- Tests cover malformed JSON, unknown fields, tampering, invalid references,
  failed Byzantine assumptions, deterministic replay, and independent audit.
- CI has read-only `contents` permission and pins external actions to complete
  commit hashes.
- Rust and Lean dependency resolutions are committed.
- Generated build directories, Lean packages, Python caches, and rendering
  profiles are excluded from version control.
- The public integrity checker rejects machine-specific Windows/WSL paths and
  validates the checked-in trace hash, Wolfram result summary, HTML assets, and
  public repository links.

## Limitations

- Endor SAST supports the Rust, Python, and JavaScript portions; Wolfram
  Language and Lean source received manual and build-oriented review rather
  than language-specific SAST.
- Binary PPTX/PDF/JPEG/PNG assets were inspected for presentation quality and
  repository provenance, not reverse-engineered as executable content.
- Build caches (`rust/target`, `lean/.lake`) were deliberately excluded because
  they are not published artifacts.
- No production deployment, remote service, validator, wallet, key store, or
  blockchain network was penetration-tested.
- This assessment is point-in-time. Dependabot and CI should be kept enabled,
  and dependency/security scans should be repeated for releases.

## Release recommendation

**Approved for public demonstration release** within the documented bounded
scope. Do not represent this assessment as certification of production Sui,
complete Mysticeti safety/liveness, or formal verification of the Rust binary.

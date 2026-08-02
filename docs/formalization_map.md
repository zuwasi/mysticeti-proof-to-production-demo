# Paper-to-code formalization map

| Paper item | Normalized claim | Wolfram implementation | Lean theorem | Rust executable / conformance | Added assumptions | Status |
|---|---|---|---|---|---|---|
| Section II-C quorum threshold | Two \(2f+1\) quorums in \(3f+1\) intersect in at least \(f+1\). | `HonestIntersectionLowerBound` and safety envelope | `quorum_intersection_at_least_f_add_one` | Weighted threshold executed; trace arithmetic audited | Equal voting power in theorem | Lean theorem plus Rust conformance |
| Section II-C honest overlap | If at most \(f\) validators are Byzantine, quorum intersection contains an honest validator. | Boundary fixtures | `quorum_intersection_contains_honest` | Assumption recorded and required by strict verification | Byzantine stake below one third | Formally proved in Lean; runtime assumption checked |
| Lemma 4 support discipline | An honest validator supports at most one proposal in one proposer slot. | `SupportedProposal` deterministically returns one proposal at most | Used as an explicit premise | Generated honest single-support assumption checked | Production DFS/order equivalence is not proved | Partially formalized and executable |
| Lemma 5 certificate uniqueness | Distinct proposals in the same slot cannot both gather support quorums. | Duplicate-author and support fixtures | `lemma5_at_most_one_certified_block_per_slot` | Conflicting certificates checked in trace | `slotOf` equality; honest single-support premise | Formally proved for theorem; runtime checked |
| Algorithms 1-2 direct certificate | An \(r+2\) certificate block contains quorum stake from distinct \(r+1\) supporters. | `CertificateEvidence`; Rust trace audit | N/A | Executed and independently stake-recomputed | Bounded direct path | Cross-language conformance |
| Algorithm 2 direct commit | Direct commit requires quorum stake from distinct certificate authors. | `DirectSlotDecision`; Rust trace audit | N/A | Executed, replayed, tamper checked | Direct path only | Cross-language conformance |
| Section II-C direct skip | Every proposal has \(q\) round-\(r+1\) non-support authorities. | Conservative direct skip in `DirectSlotDecision` | N/A | Bounded no-proposal skip only | Round-level local view | Numerically validated only |
| Lemma 6 / Theorem 1 | Correct validators produce consistent slot states and total order. | Not claimed | N/A | Not implemented | Requires indirect-decision and causal-history formalization | Partially formalized |
| Theorem 6 quorum core | Conflicting transactions cannot both gather vote quorums. | Quorum boundary examples | `conflicting_transactions_cannot_both_gather_quorums` | No-conflicting-direct-commit trace check | Honest validators do not double-vote conflicts | Formally proved only for Lean statement |
| Theorem 6 full FPC safety | Honest validator never finalizes conflicting transactions. | Not claimed | Quorum core only | Not implemented | Finalization and consensus-path composition omitted | Partially formalized |
| Section VII timing | Loss changes event delivery and commit outcomes. | `RustFaultSweepPlot` | N/A | Event-driven 20-seed campaign | Deterministic research model, not WAN calibration | Executable campaign |
| Section VIII Table I | Bullshark 2890/4600 ms; Mysticeti-C 650/975 ms under reported setup. | CSV-backed comparison | N/A | No reproduction claim | Values transcribed locally | Numerically validated only |

## Status interpretation

“Formally proved” applies only to the exact Lean statement named in that row. It does not propagate to adjacent protocol claims. “Numerically validated only” includes executable fixtures and synthetic analysis; it is not a mathematical proof or production benchmark reproduction.

Rows not expanded above retain their prior status. Rust execution and Mathematica conformance are tests, not proof that Rust implements production Mysticeti; Lean does not prove Rust.

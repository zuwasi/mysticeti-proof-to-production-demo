import Mathlib.Data.Finset.Card

/-!
# Finite definitions for the Mysticeti quorum kernel

This file captures only a Lean-checked, equal-voting-power quorum-counting
abstraction used by Mysticeti (NDSS 2025), Section II-C: `n = 3f + 1` and
quorum threshold `2f + 1`.  DAG support traversal, the commit/skip decision
state machine, networking, cryptography, liveness, epoch change, and full
transaction finalization semantics are deliberately outside this finite kernel.
-/

namespace Mysticeti

/-- The validator type for fault bound `f`, with exactly `3f + 1` members. -/
abbrev Validator (f : Nat) := Fin (3 * f + 1)

/-- The equal-power quorum threshold used in Section II-C. -/
def quorumSize (f : Nat) : Nat := 2 * f + 1

/-- A finite voter set reaches the equal-power quorum threshold. -/
def IsQuorum {α : Type*} [DecidableEq α] (f : Nat) (voters : Finset α) : Prop :=
  quorumSize f ≤ voters.card

/-- Certification is abstracted to the set of validators supporting a proposal. -/
def Certified {α Proposal : Type*} [DecidableEq α]
    (f : Nat) (supports : Proposal → Finset α) (proposal : Proposal) : Prop :=
  IsQuorum f (supports proposal)

end Mysticeti

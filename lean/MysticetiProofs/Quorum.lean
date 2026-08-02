import MysticetiProofs.Definitions
import Mathlib.Tactic

/-!
# Quorum intersection

These Lean-checked lemmas formalize only the quorum-counting step used throughout
Mysticeti (NDSS 2025), Section II-C, Lemma 4, Corollary 1, Lemma 5, and Theorem 6.
Voting power is equal, so cardinality is the relevant weight.  They do not model
DAG support traversal, the commit/skip decision state machine, networking,
cryptography, liveness, epoch change, or full transaction finalization.
-/

namespace Mysticeti

/-- Two `2f + 1` quorums inside a `3f + 1` universe overlap in at least `f + 1`
validators.  This is the finite counting core behind Section II-C's quorum
intersection arguments. -/
theorem quorum_intersection_at_least_f_add_one
    {α : Type*} [DecidableEq α]
    (f : Nat) (validators first second : Finset α)
    (hUniverse : validators.card = 3 * f + 1)
    (hFirstSub : first ⊆ validators)
    (hSecondSub : second ⊆ validators)
    (hFirst : 2 * f + 1 ≤ first.card)
    (hSecond : 2 * f + 1 ≤ second.card) :
    f + 1 ≤ (first ∩ second).card := by
  have hUnionSub : first ∪ second ⊆ validators := Finset.union_subset hFirstSub hSecondSub
  have hUnionCard : (first ∪ second).card ≤ validators.card := Finset.card_le_card hUnionSub
  have hCount := Finset.card_union_add_card_inter first second
  omega

/-- If at most `f` validators are Byzantine, the intersection of two quorums
contains a non-Byzantine validator (Section II-C, Lemma 4 and Corollary 1). -/
theorem quorum_intersection_contains_honest
    {α : Type*} [DecidableEq α]
    (f : Nat) (validators byzantine first second : Finset α)
    (hUniverse : validators.card = 3 * f + 1)
    (hByzantine : byzantine.card ≤ f)
    (hFirstSub : first ⊆ validators)
    (hSecondSub : second ⊆ validators)
    (hFirst : 2 * f + 1 ≤ first.card)
    (hSecond : 2 * f + 1 ≤ second.card) :
    ∃ validator ∈ first ∩ second, validator ∉ byzantine := by
  have hIntersection := quorum_intersection_at_least_f_add_one f validators first second
    hUniverse hFirstSub hSecondSub hFirst hSecond
  by_contra hNoHonest
  push Not at hNoHonest
  have hSub : first ∩ second ⊆ byzantine := by
    intro validator hValidator
    exact hNoHonest validator hValidator
  have hCard := Finset.card_le_card hSub
  omega

example : quorumSize 1 = 3 := by decide
example : (3 * 1 + 1 : Nat) = 4 := by decide
example : quorumSize 2 = 5 := by decide
example : (3 * 2 + 1 : Nat) = 7 := by decide

end Mysticeti

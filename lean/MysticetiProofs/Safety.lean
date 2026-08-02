import MysticetiProofs.Quorum

/-!
# Safety consequences of quorum intersection

The assumptions below expose the protocol obligation explicitly: an honest
validator does not support two distinct proposals in one slot and does not vote
for conflicting fast-path transactions.  Combined with finite quorum
intersection, these give Lean-checked quorum-counting abstractions used by
Mysticeti (NDSS 2025) Lemma 5 and Theorem 6.  Lemma 4 and Corollary 1 supply the
honest-intersection principle proved in `Quorum.lean`.

These results do not model DAG support traversal, the commit/skip decision state
machine, networking, cryptography, liveness, epoch change, or full transaction
finalization, and therefore are not full protocol safety or finalization proofs.
-/

namespace Mysticeti

/-- Paper Lemma 5 abstraction: at most one proposal can be certified for a slot,
provided every honest validator supports at most one proposal in that slot. -/
theorem lemma5_at_most_one_certified_block_per_slot
    {α Proposal Slot : Type*} [DecidableEq α] [DecidableEq Proposal]
    (f : Nat) (validators byzantine : Finset α)
    (supports : Proposal → Finset α)
    (slotOf : Proposal → Slot)
    (hUniverse : validators.card = 3 * f + 1)
    (hByzantine : byzantine.card ≤ f)
    (hSupportsSub : ∀ proposal, supports proposal ⊆ validators)
    (hHonestSingleSupport : ∀ validator, validator ∉ byzantine →
      ∀ first second, slotOf first = slotOf second →
        validator ∈ supports first → validator ∈ supports second → first = second)
    {first second : Proposal}
    (hSameSlot : slotOf first = slotOf second)
    (hFirstCertified : Certified f supports first)
    (hSecondCertified : Certified f supports second) :
    first = second := by
  obtain ⟨validator, hBoth, hHonest⟩ := quorum_intersection_contains_honest
    f validators byzantine (supports first) (supports second)
    hUniverse hByzantine (hSupportsSub first) (hSupportsSub second)
    hFirstCertified hSecondCertified
  have ⟨hFirstSupport, hSecondSupport⟩ := Finset.mem_inter.mp hBoth
  exact hHonestSingleSupport validator hHonest first second hSameSlot hFirstSupport hSecondSupport

/-- Quorum-counting core used by paper Theorem 6: two conflicting transactions
cannot both gather quorums if honest validators never vote for a conflicting
pair.  This theorem does not define or prove transaction finalization. -/
theorem conflicting_transactions_cannot_both_gather_quorums
    {α Transaction : Type*} [DecidableEq α]
    (f : Nat) (validators byzantine : Finset α)
    (votes : Transaction → Finset α)
    (Conflicts : Transaction → Transaction → Prop)
    (hUniverse : validators.card = 3 * f + 1)
    (hByzantine : byzantine.card ≤ f)
    (hVotesSub : ∀ transaction, votes transaction ⊆ validators)
    (hHonestNoConflictVote : ∀ validator, validator ∉ byzantine →
      ∀ first second, Conflicts first second →
        validator ∈ votes first → validator ∉ votes second)
    {first second : Transaction}
    (hConflict : Conflicts first second)
    (hFirstQuorum : IsQuorum f (votes first))
    (hSecondQuorum : IsQuorum f (votes second)) :
    False := by
  obtain ⟨validator, hBoth, hHonest⟩ := quorum_intersection_contains_honest
    f validators byzantine (votes first) (votes second)
    hUniverse hByzantine (hVotesSub first) (hVotesSub second)
    hFirstQuorum hSecondQuorum
  have ⟨hFirstVote, hSecondVote⟩ := Finset.mem_inter.mp hBoth
  exact (hHonestNoConflictVote validator hHonest first second hConflict hFirstVote) hSecondVote

end Mysticeti

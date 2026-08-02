# Plain-English proof companion

## Claim being proved

In the equal-voting-power abstraction used by the Lean kernel, let:

- the committee contain exactly \(n = 3f+1\) validators;
- at most \(f\) validators be Byzantine;
- a quorum contain at least \(q = 2f+1\) validators.

Then two quorums share at least \(f+1\) validators, so at least one validator in their intersection is honest.

This is the counting core used in the paper’s certificate-uniqueness argument (Lemma 5) and conflicting-fast-path-vote argument (Theorem 6). It is not the complete proof of Mysticeti consensus.

## 1. Quorum intersection

Let \(Q_1\) and \(Q_2\) be two quorums. Inclusion-exclusion gives

\[
|Q_1 \cap Q_2| = |Q_1| + |Q_2| - |Q_1 \cup Q_2|.
\tag{1}
\]

Both quorums lie inside the committee, so \(|Q_1 \cup Q_2| \le 3f+1\). Because each quorum has at least \(2f+1\) members,

\[
|Q_1 \cap Q_2|
\ge (2f+1)+(2f+1)-(3f+1)
= f+1.
\tag{2}
\]

## 2. Honest intersection

There are at most \(f\) Byzantine validators, but Equation (2) places at least \(f+1\) validators in the intersection. Therefore the intersection cannot consist entirely of Byzantine validators. At least one shared validator is honest.

## 3. Why two proposals in one slot cannot both be certified

Assume two different proposals \(P_1\) and \(P_2\) occupy the same proposer slot and both gather support quorums.

The honest-intersection result produces an honest validator that belongs to both support sets. But the paper’s support rule—and the explicit premise of the Lean theorem—says an honest validator supports at most one proposal in a given slot. Therefore \(P_1=P_2\), contradicting the assumption that they differ.

Lean theorem:

```text
lemma5_at_most_one_certified_block_per_slot
```

## 4. Why conflicting transactions cannot both gather vote quorums

Assume conflicting transactions \(T_1\) and \(T_2\) both gather quorums. Again, quorum intersection identifies an honest validator appearing in both vote sets. The explicit honest-validator premise forbids voting for both members of a conflicting pair, producing a contradiction.

Lean theorem:

```text
conflicting_transactions_cannot_both_gather_quorums
```

This proves the quorum-counting core used by paper Theorem 6. It does **not** formalize Mysticeti-FPC transaction finalization, epoch transitions, or persistence.

## 5. Concrete examples

| Fault bound | Committee | Quorum | Minimum overlap | Honest overlap guaranteed |
|---:|---:|---:|---:|---:|
| 1 | 4 | 3 | 2 | Yes—at least 1 |
| 2 | 7 | 5 | 3 | Yes—at least 1 |
| 3 | 10 | 7 | 4 | Yes—at least 1 |

## Trust boundary

Lean checks that the theorem follows from the formal assumptions. It does not establish that:

- the paper was transcribed perfectly;
- the Wolfram support traversal is equivalent to production Sui;
- signatures or network delivery behave correctly;
- the complete commit/skip algorithm is safe and live;
- production voting power is equal rather than stake weighted.

Those distinctions are recorded in `formalization_map.md` and tested or documented in the appropriate evidence lane.

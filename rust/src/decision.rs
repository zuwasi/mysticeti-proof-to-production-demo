use crate::{
    dag::Dag,
    model::{AuthorityId, Block, BlockId, Committee},
    wave::{WaveRoundError, wave_rounds},
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionKind {
    Commit,
    Skip,
    NoDecision,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Decision {
    pub slot: u32,
    pub proposal_id: Option<BlockId>,
    pub proposal_round: u32,
    pub kind: DecisionKind,
    pub support_stake: u64,
    pub certificate_author_stake: u64,
    pub support_authors: Vec<AuthorityId>,
    pub certificate_authors: Vec<AuthorityId>,
    pub evidence: Vec<String>,
}

/// Canonical vote selection: the lexicographically smallest directly referenced
/// proposal ID in the slot. Thus even malformed ancestry containing equivocations
/// gives an honest support block exactly one vote.
pub fn selected_proposal(dag: &Dag, support: &Block) -> Option<BlockId> {
    let proposal_round = wave_rounds(support.slot).ok()?.proposal;
    support
        .parents
        .iter()
        .filter_map(|id| dag.get(id))
        .filter(|p| p.slot == support.slot && p.round == proposal_round)
        .map(|p| p.id.clone())
        .min()
}

pub fn evaluate_slot(
    committee: &Committee,
    dag: &Dag,
    slot: u32,
) -> Result<Vec<Decision>, WaveRoundError> {
    let rounds = wave_rounds(slot)?;
    let proposal_round = rounds.proposal;
    let proposals: Vec<_> = dag
        .blocks()
        .filter(|b| b.slot == slot && b.round == proposal_round)
        .collect();
    let supports: Vec<_> = dag
        .blocks()
        .filter(|b| b.slot == slot && b.round == rounds.support)
        .collect();
    let cert_blocks: Vec<_> = dag
        .blocks()
        .filter(|b| b.slot == slot && b.round == rounds.certificate)
        .collect();
    let mut out = Vec::new();
    for proposal in proposals {
        let support_authors: BTreeSet<AuthorityId> = supports
            .iter()
            .filter(|s| selected_proposal(dag, s).as_ref() == Some(&proposal.id))
            .map(|s| s.author.clone())
            .collect();
        let support_stake = committee.distinct_stake(support_authors.iter());
        let qualifying: BTreeSet<AuthorityId> = cert_blocks
            .iter()
            .filter(|c| {
                let parent_authors: BTreeSet<AuthorityId> = c
                    .parents
                    .iter()
                    .filter_map(|id| dag.get(id))
                    .filter(|b| {
                        b.round == rounds.support
                            && b.slot == slot
                            && selected_proposal(dag, b).as_ref() == Some(&proposal.id)
                    })
                    .map(|b| b.author.clone())
                    .collect();
                committee.distinct_stake(parent_authors.iter()) >= committee.quorum_threshold()
            })
            .map(|c| c.author.clone())
            .collect();
        let certificate_author_stake = committee.distinct_stake(qualifying.iter());
        let kind = if certificate_author_stake >= committee.quorum_threshold() {
            DecisionKind::Commit
        } else {
            DecisionKind::NoDecision
        };
        out.push(Decision {
            slot,
            proposal_id: Some(proposal.id.clone()),
            proposal_round,
            kind,
            support_stake,
            certificate_author_stake,
            support_authors: support_authors.into_iter().collect(),
            certificate_authors: qualifying.into_iter().collect(),
            evidence: vec![
                "distinct-author support stake".into(),
                "r+2 certificate-block author stake".into(),
            ],
        });
    }
    if out.is_empty() {
        out.push(Decision {
            slot,
            proposal_id: None,
            proposal_round,
            kind: DecisionKind::Skip,
            support_stake: 0,
            certificate_author_stake: 0,
            support_authors: vec![],
            certificate_authors: vec![],
            evidence: vec!["educational skip: no round-r proposal observed by r+2".into()],
        });
    }
    Ok(out)
}

pub fn evaluate_all(
    committee: &Committee,
    dag: &Dag,
    configured_slots: u32,
) -> Result<Vec<Decision>, WaveRoundError> {
    (0..configured_slots).try_fold(Vec::new(), |mut decisions, slot| {
        decisions.extend(evaluate_slot(committee, dag, slot)?);
        Ok(decisions)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Authority, AuthorityId};

    fn fixture(cert_authors: usize) -> (Committee, Dag) {
        let auth: Vec<_> = (0..4)
            .map(|i| Authority {
                id: AuthorityId(format!("a{i}")),
                stake: 1,
            })
            .collect();
        let c = Committee::new(auth).unwrap();
        let p = Block::new(AuthorityId("a0".into()), 0, 0, 0, vec![]);
        let supports: Vec<_> = (0..4)
            .map(|i| Block::new(AuthorityId(format!("a{i}")), 1, 0, 0, vec![p.id.clone()]))
            .collect();
        let parents: Vec<_> = supports.iter().map(|b| b.id.clone()).collect();
        let certs: Vec<_> = (0..cert_authors)
            .map(|i| Block::new(AuthorityId(format!("a{i}")), 2, 0, 0, parents.clone()))
            .collect();
        let blocks = std::iter::once(p)
            .chain(supports)
            .chain(certs)
            .collect::<Vec<_>>();
        (c.clone(), Dag::build(&c, &blocks).unwrap())
    }

    #[test]
    fn certificate_is_not_commit() {
        let (c, d) = fixture(1);
        assert_eq!(
            evaluate_slot(&c, &d, 0).unwrap()[0].kind,
            DecisionKind::NoDecision
        );
    }
    #[test]
    fn happy_direct_commit() {
        let (c, d) = fixture(3);
        assert_eq!(
            evaluate_slot(&c, &d, 0).unwrap()[0].kind,
            DecisionKind::Commit
        );
    }
    #[test]
    fn equivocation_does_not_multiply_stake() {
        let (c, mut d) = fixture(1);
        let p = d.blocks().find(|b| b.round == 0).unwrap().id.clone();
        let e = Block::new(AuthorityId("a0".into()), 1, 0, 1, vec![p]);
        d.insert(&c, e).unwrap();
        assert_eq!(evaluate_slot(&c, &d, 0).unwrap()[0].support_stake, 4);
    }

    #[test]
    fn malformed_dual_ancestry_selects_only_one_equivocation() {
        let authorities: Vec<_> = (0..4)
            .map(|i| Authority {
                id: AuthorityId(format!("a{i}")),
                stake: 1,
            })
            .collect();
        let c = Committee::new(authorities).unwrap();
        let p0 = Block::new(AuthorityId("a0".into()), 0, 0, 0, vec![]);
        let p1 = Block::new(AuthorityId("a0".into()), 0, 0, 1, vec![]);
        let both = vec![p0.id.clone(), p1.id.clone()];
        let mut supports: Vec<_> = (0..3)
            .map(|i| Block::new(AuthorityId(format!("a{i}")), 1, 0, 0, both.clone()))
            .collect();
        supports.push(Block::new(
            AuthorityId("a3".into()),
            1,
            0,
            0,
            vec![p0.id.clone()],
        ));
        supports.push(Block::new(
            AuthorityId("a3".into()),
            1,
            0,
            1,
            vec![p1.id.clone()],
        ));
        let support_ids = supports.iter().map(|b| b.id.clone()).collect::<Vec<_>>();
        let certs = (0..4)
            .map(|i| Block::new(AuthorityId(format!("a{i}")), 2, 0, 0, support_ids.clone()))
            .collect::<Vec<_>>();
        let blocks = vec![p0, p1]
            .into_iter()
            .chain(supports)
            .chain(certs)
            .collect::<Vec<_>>();
        let dag = Dag::build(&c, &blocks).unwrap();
        let decisions = evaluate_slot(&c, &dag, 0).unwrap();
        assert_eq!(
            decisions
                .iter()
                .filter(|d| d.kind == DecisionKind::Commit)
                .count(),
            1
        );
        assert!(decisions.iter().all(|d| d.support_authors.len() <= 4));
    }
}

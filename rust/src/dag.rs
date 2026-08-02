use crate::{
    model::{Block, BlockId, Committee},
    wave::wave_rounds,
};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

#[derive(Debug, Error, Clone, Eq, PartialEq)]
pub enum DagError {
    #[error("block {block} has unknown author {author}")]
    UnknownAuthor { block: BlockId, author: String },
    #[error("block {block} has unknown parent {parent}")]
    UnknownParent { block: BlockId, parent: BlockId },
    #[error("parent {parent} is not earlier than block {block}")]
    ParentNotEarlier { block: BlockId, parent: BlockId },
    #[error("duplicate author/round/slot/variant identity for block {0}")]
    IdentityCollision(BlockId),
    #[error("block {0} has a non-canonical deterministic id")]
    InvalidBlockId(BlockId),
    #[error("block {0} has non-canonical parents (must be strictly sorted and unique)")]
    NonCanonicalParents(BlockId),
    #[error("block {block} round {round} does not match slot {slot} phase")]
    InvalidWave {
        block: BlockId,
        round: u32,
        slot: u32,
    },
    #[error("block {block} has phase/slot-invalid parent {parent}")]
    InvalidParentPhase { block: BlockId, parent: BlockId },
}

#[derive(Clone, Debug)]
pub struct Dag {
    blocks: BTreeMap<BlockId, Block>,
    identities: BTreeSet<(crate::model::AuthorityId, u32, u32, u32)>,
}

impl Dag {
    pub fn build(committee: &Committee, blocks: &[Block]) -> Result<Self, DagError> {
        let mut ordered = blocks.to_vec();
        ordered.sort_by(|a, b| (a.round, &a.id).cmp(&(b.round, &b.id)));
        let mut dag = Self {
            blocks: BTreeMap::new(),
            identities: BTreeSet::new(),
        };
        for block in ordered {
            dag.insert(committee, block)?;
        }
        Ok(dag)
    }

    pub fn insert(&mut self, committee: &Committee, block: Block) -> Result<(), DagError> {
        if committee.stake(&block.author).is_none() {
            return Err(DagError::UnknownAuthor {
                block: block.id,
                author: block.author.0,
            });
        }
        if !block.has_valid_id() {
            return Err(DagError::InvalidBlockId(block.id));
        }
        if !block.parents.windows(2).all(|w| w[0] < w[1]) {
            return Err(DagError::NonCanonicalParents(block.id));
        }
        let rounds = wave_rounds(block.slot).map_err(|_| DagError::InvalidWave {
            block: block.id.clone(),
            round: block.round,
            slot: block.slot,
        })?;
        if !(rounds.proposal..=rounds.certificate).contains(&block.round) {
            return Err(DagError::InvalidWave {
                block: block.id.clone(),
                round: block.round,
                slot: block.slot,
            });
        }
        let identity = (block.author.clone(), block.round, block.slot, block.variant);
        if self.identities.contains(&identity) {
            return Err(DagError::IdentityCollision(block.id));
        }
        for parent_id in &block.parents {
            let parent = self
                .blocks
                .get(parent_id)
                .ok_or_else(|| DagError::UnknownParent {
                    block: block.id.clone(),
                    parent: parent_id.clone(),
                })?;
            if parent.round >= block.round {
                return Err(DagError::ParentNotEarlier {
                    block: block.id.clone(),
                    parent: parent_id.clone(),
                });
            }
            if parent.slot != block.slot || parent.round.checked_add(1) != Some(block.round) {
                return Err(DagError::InvalidParentPhase {
                    block: block.id.clone(),
                    parent: parent_id.clone(),
                });
            }
        }
        self.identities.insert(identity);
        self.blocks.insert(block.id.clone(), block);
        Ok(())
    }

    pub fn get(&self, id: &BlockId) -> Option<&Block> {
        self.blocks.get(id)
    }
    pub fn blocks(&self) -> impl Iterator<Item = &Block> {
        self.blocks.values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Authority, AuthorityId};

    #[test]
    fn invalid_parent_is_rejected() {
        let c = Committee::new(vec![Authority {
            id: AuthorityId("a".into()),
            stake: 1,
        }])
        .unwrap();
        let b = Block::new(AuthorityId("a".into()), 1, 0, 0, vec!["missing".into()]);
        assert!(matches!(
            Dag::build(&c, &[b]),
            Err(DagError::UnknownParent { .. })
        ));
    }

    #[test]
    fn failed_insert_is_transactional_and_retry_succeeds() {
        let c = Committee::new(vec![Authority {
            id: AuthorityId("a".into()),
            stake: 1,
        }])
        .unwrap();
        let mut dag = Dag::build(&c, &[]).unwrap();
        let bad = Block::new(AuthorityId("a".into()), 1, 0, 0, vec!["missing".into()]);
        assert!(dag.insert(&c, bad).is_err());
        let proposal = Block::new(AuthorityId("a".into()), 0, 0, 0, vec![]);
        dag.insert(&c, proposal.clone()).unwrap();
        let retry = Block::new(AuthorityId("a".into()), 1, 0, 0, vec![proposal.id]);
        dag.insert(&c, retry).unwrap();
    }

    #[test]
    fn wave_and_parent_canonicality_are_enforced() {
        let c = Committee::new(vec![Authority {
            id: AuthorityId("a".into()),
            stake: 1,
        }])
        .unwrap();
        let p = Block::new(AuthorityId("a".into()), 0, 0, 0, vec![]);
        let wrong_slot = Block::new(AuthorityId("a".into()), 4, 0, 0, vec![p.id.clone()]);
        assert!(matches!(
            Dag::build(&c, &[p.clone(), wrong_slot]),
            Err(DagError::InvalidWave { .. })
        ));
        let mut duplicate = Block::new(AuthorityId("a".into()), 1, 0, 0, vec![p.id.clone()]);
        duplicate.parents.push(p.id);
        duplicate.id = crate::model::deterministic_block_id(
            &duplicate.author,
            duplicate.round,
            duplicate.slot,
            duplicate.variant,
            &duplicate.parents,
        );
        assert!(matches!(
            Dag::build(&c, &[duplicate]),
            Err(DagError::NonCanonicalParents(_))
        ));
    }

    #[test]
    fn unrepresentable_high_slot_is_rejected() {
        let c = Committee::new(vec![Authority {
            id: AuthorityId("a".into()),
            stake: 1,
        }])
        .unwrap();
        let block = Block::new(AuthorityId("a".into()), u32::MAX, u32::MAX, 0, vec![]);
        assert!(matches!(
            Dag::build(&c, &[block]),
            Err(DagError::InvalidWave { .. })
        ));
    }
}

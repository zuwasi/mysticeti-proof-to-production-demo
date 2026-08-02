use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

pub type Round = u32;
pub type Stake = u64;
pub type BlockId = String;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AuthorityId(pub String);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Authority {
    pub id: AuthorityId,
    pub stake: Stake,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Committee {
    authorities: BTreeMap<AuthorityId, Stake>,
    total_stake: Stake,
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum CommitteeError {
    #[error("committee must not be empty")]
    Empty,
    #[error("authority {0} has zero stake")]
    ZeroStake(String),
    #[error("duplicate authority {0}")]
    Duplicate(String),
    #[error("total stake overflow")]
    Overflow,
}

impl Committee {
    pub fn new(authorities: Vec<Authority>) -> Result<Self, CommitteeError> {
        if authorities.is_empty() {
            return Err(CommitteeError::Empty);
        }
        let mut map = BTreeMap::new();
        let mut total = 0_u64;
        for authority in authorities {
            if authority.stake == 0 {
                return Err(CommitteeError::ZeroStake(authority.id.0));
            }
            let id = authority.id;
            if map.insert(id.clone(), authority.stake).is_some() {
                return Err(CommitteeError::Duplicate(id.0));
            }
            total = total
                .checked_add(authority.stake)
                .ok_or(CommitteeError::Overflow)?;
        }
        Ok(Self {
            authorities: map,
            total_stake: total,
        })
    }

    pub fn authorities(&self) -> impl Iterator<Item = (&AuthorityId, &Stake)> {
        self.authorities.iter()
    }

    pub fn stake(&self, id: &AuthorityId) -> Option<Stake> {
        self.authorities.get(id).copied()
    }

    pub const fn total_stake(&self) -> Stake {
        self.total_stake
    }

    pub fn quorum_threshold(&self) -> Stake {
        self.total_stake / 3 * 2 + (self.total_stake % 3 * 2) / 3 + 1
    }

    /// Maximum Byzantine stake satisfying the strict `< total/3` safety assumption.
    pub fn byzantine_safety_bound(&self) -> Stake {
        self.total_stake.saturating_sub(1) / 3
    }

    pub fn distinct_stake<'a>(&self, ids: impl IntoIterator<Item = &'a AuthorityId>) -> Stake {
        ids.into_iter()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .filter_map(|id| self.stake(id))
            .sum()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Block {
    pub id: BlockId,
    pub author: AuthorityId,
    pub round: Round,
    pub slot: u32,
    pub variant: u32,
    pub parents: Vec<BlockId>,
}

impl Block {
    pub fn new(
        author: AuthorityId,
        round: Round,
        slot: u32,
        variant: u32,
        mut parents: Vec<BlockId>,
    ) -> Self {
        parents.sort();
        parents.dedup();
        let id = deterministic_block_id(&author, round, slot, variant, &parents);
        Self {
            id,
            author,
            round,
            slot,
            variant,
            parents,
        }
    }

    pub fn has_valid_id(&self) -> bool {
        self.id
            == deterministic_block_id(
                &self.author,
                self.round,
                self.slot,
                self.variant,
                &self.parents,
            )
    }
}

pub fn deterministic_block_id(
    author: &AuthorityId,
    round: Round,
    slot: u32,
    variant: u32,
    parents: &[BlockId],
) -> BlockId {
    let mut hasher = Sha256::new();
    hasher.update(b"mysticeti-twin:block:v1\0");
    hasher.update((author.0.len() as u64).to_be_bytes());
    hasher.update(author.0.as_bytes());
    hasher.update(round.to_be_bytes());
    hasher.update(slot.to_be_bytes());
    hasher.update(variant.to_be_bytes());
    for parent in parents {
        hasher.update((parent.len() as u64).to_be_bytes());
        hasher.update(parent.as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thresholds_are_exact() {
        let c = Committee::new(vec![Authority {
            id: AuthorityId("a".into()),
            stake: 10,
        }])
        .unwrap();
        assert_eq!(c.quorum_threshold(), 7);
        assert_eq!(c.byzantine_safety_bound(), 3);
    }

    #[test]
    fn weighted_quorum_deduplicates() {
        let c = Committee::new(vec![
            Authority {
                id: AuthorityId("a".into()),
                stake: 7,
            },
            Authority {
                id: AuthorityId("b".into()),
                stake: 3,
            },
        ])
        .unwrap();
        let a = AuthorityId("a".into());
        assert_eq!(c.distinct_stake([&a, &a]), 7);
        assert!(c.distinct_stake([&a]) >= c.quorum_threshold());
    }
}

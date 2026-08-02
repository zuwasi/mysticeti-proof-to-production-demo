use mysticeti_twin::model::{Authority, AuthorityId, Committee};
use proptest::prelude::*;

proptest! {
    #[test]
    fn weighted_quorums_intersect_in_more_than_byzantine_stake(
        weights in prop::collection::vec(1_u16..100, 4..16),
        left_bits in any::<u16>(), right_bits in any::<u16>()
    ) {
        let committee = Committee::new(weights.iter().enumerate().map(|(i, w)| Authority {
            id: AuthorityId(format!("a{i}")), stake: u64::from(*w)
        }).collect()).unwrap();
        let total=committee.total_stake();
        let quorum=committee.quorum_threshold();
        let removal_budget=total-quorum;
        let quorum_set = |bits:u16| {
            let mut removed=0;
            weights.iter().enumerate().map(|(i,w)| {
                let stake=u64::from(*w);
                let keep=bits&(1<<i)==0 || removed+stake>removal_budget;
                if !keep { removed+=stake; }
                keep
            }).collect::<Vec<_>>()
        };
        let left=quorum_set(left_bits);
        let right=quorum_set(right_bits);
        let intersection_ids:Vec<_>=weights.iter().enumerate().filter(|(i,_)|left[*i] && right[*i])
            .map(|(i,_)| AuthorityId(format!("a{i}"))).collect();
        let intersection=committee.distinct_stake(intersection_ids.iter());
        let byzantine_bound=committee.byzantine_safety_bound();
        prop_assert!(intersection>byzantine_bound, "total={total}, q={quorum}, intersection={intersection}, f={byzantine_bound}");
    }
}

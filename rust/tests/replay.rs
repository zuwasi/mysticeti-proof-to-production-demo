use mysticeti_twin::{
    SimulationConfig,
    dag::Dag,
    decision::DecisionKind,
    model::{AuthorityId, Committee},
    simulate,
    trace::{TRACE_SCHEMA_VERSION, VerifyError, audit_decisions},
    verify_trace,
};

#[test]
fn replay_is_deterministic() {
    let config = SimulationConfig::default();
    let a = simulate(&config).unwrap();
    let b = simulate(&config).unwrap();
    assert_eq!(a, b);
    assert_eq!(verify_trace(&a).unwrap(), a.decisions);
}

#[test]
fn schema_rejection() {
    let mut trace = simulate(&SimulationConfig::default()).unwrap();
    assert_eq!(trace.schema_version, TRACE_SCHEMA_VERSION);
    trace.schema_version = "future.v99".into();
    assert!(matches!(verify_trace(&trace), Err(VerifyError::Schema(_))));
}

#[test]
fn malformed_trace_rejection() {
    let mut trace = simulate(&SimulationConfig::default()).unwrap();
    trace.blocks[0].id = "tampered".into();
    assert!(matches!(
        verify_trace(&trace),
        Err(VerifyError::Dag(_) | VerifyError::Invariant(_))
    ));
}

#[test]
fn network_parameters_change_production_but_remain_reproducible() {
    let base = SimulationConfig {
        seed: 91,
        slots: 8,
        ..Default::default()
    };
    let a = simulate(&base).unwrap();
    assert_eq!(a, simulate(&base).unwrap());
    let slow = simulate(&SimulationConfig {
        round_duration_ms: 3,
        ..base.clone()
    })
    .unwrap();
    let lossy = simulate(&SimulationConfig {
        packet_loss: 0.8,
        ..base
    })
    .unwrap();
    assert_ne!(a.blocks, slow.blocks);
    assert_ne!(a.blocks, lossy.blocks);
    assert_ne!(a.decisions, lossy.decisions);
}

#[test]
fn unknown_nested_field_and_tampering_are_rejected() {
    let trace = simulate(&SimulationConfig::default()).unwrap();
    let mut value = serde_json::to_value(&trace).unwrap();
    value["events"][0]["unknown"] = serde_json::json!(true);
    assert!(serde_json::from_value::<mysticeti_twin::Trace>(value).is_err());
    let mut tampered = trace;
    tampered.scope.push_str(" tampered");
    assert!(matches!(
        verify_trace(&tampered),
        Err(VerifyError::Provenance("scope"))
    ));
}

#[test]
fn independent_audit_rejects_self_consistent_but_dag_wrong_evidence() {
    let trace = simulate(&SimulationConfig::default()).unwrap();
    let committee = Committee::new(trace.committee.clone()).unwrap();
    let dag = Dag::build(&committee, &trace.blocks).unwrap();
    let mut decisions = trace.decisions.clone();
    let decision = decisions
        .iter_mut()
        .find(|d| d.proposal_id.is_some())
        .unwrap();
    decision.support_authors = vec![AuthorityId("authority-0".into())];
    decision.support_stake = committee.distinct_stake(decision.support_authors.iter());
    decision.certificate_authors = vec![AuthorityId("authority-0".into())];
    decision.certificate_author_stake =
        committee.distinct_stake(decision.certificate_authors.iter());
    decision.kind = DecisionKind::NoDecision;
    assert!(!audit_decisions(
        &committee,
        &dag,
        &decisions,
        trace.config.slots
    ));
}

#[test]
fn failed_byzantine_assumption_fails_report_and_strict_verify() {
    let trace = simulate(&SimulationConfig {
        stakes: vec![1, 1, 1],
        byzantine_authorities: vec![0],
        ..Default::default()
    })
    .unwrap();
    assert!(!trace.invariants.all_passed);
    assert!(matches!(
        verify_trace(&trace),
        Err(VerifyError::Invariant(_))
    ));
}

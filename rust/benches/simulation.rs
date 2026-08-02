use criterion::{Criterion, criterion_group, criterion_main};
use mysticeti_twin::{SimulationConfig, simulate, verify_trace};

fn medium_simulation(c: &mut Criterion) {
    let config = SimulationConfig {
        seed: 42,
        stakes: vec![1; 32],
        slots: 50,
        packet_loss: 0.05,
        byzantine_authorities: vec![30, 31],
        ..Default::default()
    };
    c.bench_function("simulate_and_replay_32_authorities_50_slots", |b| {
        b.iter(|| {
            let trace = simulate(&config).unwrap();
            verify_trace(&trace).unwrap();
        })
    });
}
criterion_group!(benches, medium_simulation);
criterion_main!(benches);

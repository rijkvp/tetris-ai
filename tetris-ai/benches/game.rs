use criterion::{criterion_group, criterion_main, Criterion};
use tetris_ai::Simulator;

fn benchmark(c: &mut Criterion) {
    let mut simulator = Simulator::new_with_preset("infinite");
    c.bench_function("tetris", |b| b.iter(|| simulator.step()));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

use criterion::{Criterion, criterion_group, criterion_main};
use tetris_ai::simulator::Simulator;

fn benchmark(c: &mut Criterion) {
    let mut simulator = Simulator::new_with_preset("infinite");
    c.bench_function("tetris", |b| b.iter(|| simulator.step()));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

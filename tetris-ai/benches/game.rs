use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
use tetris_ai::{board::Board, feature, piece::Piece, state::State};

fn run_game() {
    let state = State::new(Board::default());
    tetris_ai::simulate(
        state,
        || Piece::from_index(rand::thread_rng().gen_range(0..7)),
        tetris_ai::r#move::move_drop,
        |state| feature::evaluate_default(state),
    );
}

fn benchmark(c: &mut Criterion) {
    c.bench_function("tetris", |b| b.iter(|| run_game()));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
use tetris_ai::{board::Board, feature, piece::Piece, state::State};

fn run_game() {
    fn policy_best(pool: Vec<State>) -> Option<State> {
        let mut best = None;
        let mut best_score = f64::NEG_INFINITY;
        for state in pool {
            let score = feature::evaluate(&state, feature::DEFAULT_WEIGHTS);
            if score > best_score {
                best = Some(state);
                best_score = score;
            }
        }
        best
    }

    let state = State::new(Board::default());
    tetris_ai::simulate(
        state,
        || Piece::from_index(rand::thread_rng().gen_range(0..7)),
        tetris_ai::r#move::move_drop,
        policy_best,
    );
}

fn benchmark(c: &mut Criterion) {
    c.bench_function("tetris", |b| b.iter(|| run_game()));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

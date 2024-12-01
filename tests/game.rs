use rand::Rng;
use std::sync::atomic::AtomicUsize;
use tetris_ai::{board::Board, feature, piece::Piece, state::State};

#[test]
fn test_game() {
    let index = AtomicUsize::new(0);

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
        || {
            // let i = &index.load(Ordering::Relaxed);
            // let piece = Piece::from_index(*i);
            // index.store((i + 1) % 7, Ordering::Relaxed);
            // piece
            Piece::from_index(rand::thread_rng().gen_range(0..7))
        },
        tetris_ai::r#move::move_drop,
        policy_best,
    );
}

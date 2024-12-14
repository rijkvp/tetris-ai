use rand::Rng;
use std::time::Instant;
use tetris_ai::{board::Board, feature, piece::Piece, state::State};

fn main() {
    let state = State::new(Board::default());
    let start = Instant::now();
    let (state, steps) = tetris_ai::simulate(
        state,
        || Piece::from_index(rand::thread_rng().gen_range(0..7)),
        tetris_ai::r#move::move_drop,
        |state| feature::evaluate_default(state),
    );
    let elapsed = start.elapsed();
    println!(
        "moves: {}, elapsed: {:?}, moves/sec: {:.2}",
        steps,
        elapsed,
        steps as f64 / elapsed.as_secs_f64()
    );
    println!("Final board: {}", state.board);
}

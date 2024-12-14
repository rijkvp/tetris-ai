use rand::Rng;
use tetris_ai::{board::Board, feature, piece::Piece, state::State};

fn main() {
    let state = State::new(Board::default());
    tetris_ai::simulate(
        state,
        || Piece::from_index(rand::thread_rng().gen_range(0..7)),
        tetris_ai::r#move::move_drop,
        |state| feature::evaluate(state, feature::DEFAULT_WEIGHTS),
    );
}

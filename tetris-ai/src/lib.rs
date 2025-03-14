use board::{BOARD_HEIGHT, BOARD_WIDTH};
use feature::{Weights, WeightsMap};
use piece::Piece;
use r#move::Move;
use rand::Rng;
use serde::Serialize;
use state::State;

pub mod board;
pub mod feature;
pub mod r#move;
pub mod piece;
pub mod state;
#[cfg(test)]
pub mod test;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Default)]
pub struct Simulator {
    state: State,
    steps: u64,
    weights: Weights,
    move_result: Option<MoveResult>,
}

#[derive(Serialize, Clone)]
struct MoveResult {
    piece_idx: usize,
    path: Vec<Vec<Move>>,
}

#[derive(Serialize)]
pub struct GameState<'a> {
    board: &'a [[u8; BOARD_WIDTH]; BOARD_HEIGHT],
    stats: Stats,
    r#move: Option<&'a MoveResult>,
}

#[derive(Serialize)]
pub struct Stats {
    pub steps: u64,
    pub lines: u64,
    pub score: u64,
    pub level: u64,
    pub tetrises: u64,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Simulator {
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_preset(preset: &str) -> Self {
        Self {
            weights: Weights::from_preset(preset),
            ..Self::default()
        }
    }

    pub fn reset(&mut self) {
        self.state = State::default();
        self.steps = 0;
        self.move_result = None;
    }

    #[inline]
    fn stats_inner(&self) -> Stats {
        Stats {
            steps: self.steps,
            lines: self.state.cleared_rows,
            score: self.state.score,
            level: self.state.level(),
            tetrises: self.state.tetrises,
        }
    }

    #[cfg(not(feature = "wasm"))]
    pub fn stats(&self) -> Stats {
        self.stats_inner()
    }

    #[cfg(not(feature = "wasm"))]
    pub fn board(&self) -> board::Board {
        self.state.board
    }

    pub fn run(&mut self) {
        while self.step() {}
    }

    pub fn step(&mut self) -> bool {
        let piece = gen_random_piece(self.state.delta.as_ref().map(|d| d.piece.index()));

        // Use resivoir sampling to ramdomly select one of the best possible moves
        let mut chosen = None;
        let mut best_score = f64::NEG_INFINITY;
        let mut count = 0;
        let mut rng = rand::thread_rng();
        for path in r#move::move_dijkstra(self.state.board, piece, self.state.level()) {
            let future = self.state.future(piece, path.final_move());
            let score = self.weights.evaluate(&future);
            if score > best_score {
                best_score = score;
                chosen = Some((future, path));
                count = 1;
            } else if score == best_score {
                count += 1;
                if rng.gen_range(0..count) == 0 {
                    chosen = Some((future, path));
                }
            }
        }

        if let Some((next, path)) = chosen {
            self.state = next; // update state
            self.steps += 1;
            self.move_result = Some(MoveResult {
                piece_idx: piece.index(),
                path: path.into_moves(),
            });
            return true;
        }
        false
    }

    pub fn update_weights(&mut self, weights_map: WeightsMap) {
        self.weights = weights_map.into();
    }

    #[cfg(feature = "wasm")]
    fn game_state(&self) -> GameState<'_> {
        GameState {
            board: self.state.board.get_raw_data(),
            stats: self.stats_inner(),
            r#move: self.move_result.as_ref(),
        }
    }

    #[cfg(feature = "wasm")]
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn state(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.game_state()).unwrap()
    }
}

// Based on how NES Tetris generates pieces
fn gen_random_piece(previous: Option<usize>) -> Piece {
    let first_roll = rand::thread_rng().gen_range(0..=piece::N_PIECES);
    if first_roll == piece::N_PIECES || previous == Some(first_roll) {
        // reroll if the first roll is the same as the previous piece
        // or if the 'reroll' number is hit
        return Piece::from_index(rand::thread_rng().gen_range(0..piece::N_PIECES));
    }
    Piece::from_index(first_roll)
}

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
#[wasm_bindgen(start)]
pub fn wasm_main() {
    console_error_panic_hook::set_once();
}

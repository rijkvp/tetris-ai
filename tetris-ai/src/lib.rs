use board::Board;
use feature::Weights;
use piece::Piece;
use r#move::Move;
use rand::Rng;
use state::State;

pub mod board;
pub mod feature;
pub mod r#move;
pub mod piece;
pub mod state;
#[cfg(test)]
pub mod test;

use wasm_bindgen::prelude::*;

fn random_piece() -> Piece {
    Piece::from_index(rand::thread_rng().gen_range(0..7))
}

#[wasm_bindgen]
pub struct Simulator {
    #[wasm_bindgen(skip)]
    pub state: State,
    steps: u64,
    weights: Weights,
}

#[wasm_bindgen(getter_with_clone)]
pub struct MoveResult {
    pub piece_idx: usize,
    pub path: Vec<Move>,
}

#[wasm_bindgen]
pub struct Stats {
    pub steps: u64,
    pub lines: u64,
    pub score: u64,
    pub level: u64,
}

#[wasm_bindgen]
impl Simulator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            state: State::new(Board::default()),
            steps: 0,
            weights: Weights::default(),
        }
    }

    pub fn update_weights(&mut self, weights: Weights) {
        self.weights = weights;
    }

    pub fn stats(&self) -> Stats {
        Stats {
            steps: self.steps,
            lines: self.state.cleared_rows,
            score: self.state.score,
            level: self.state.level(),
        }
    }

    pub fn step(&mut self) -> Option<MoveResult> {
        let piece = random_piece();

        let mut best = None;
        let mut best_score = f64::NEG_INFINITY;
        for path in r#move::move_dijkstra(self.state.board, piece) {
            let r#move = path.last().unwrap();
            let future = self.state.future(piece, *r#move);
            let score = feature::evaluate(&future, &self.weights);
            if score > best_score {
                best = Some((future, path));
                best_score = score;
            }
        }

        if let Some((next, path)) = best {
            self.state = next; // update state
            self.steps += 1;
            return Some(MoveResult {
                piece_idx: piece.index(),
                path,
            });
        }
        None
    }

    pub fn board_data(&self) -> Box<[u8]> {
        let data = self.state.board.get_data();
        let mut result = Vec::with_capacity(board::BOARD_HEIGHT);
        for row in data.iter() {
            for cell in row.iter() {
                result.push(cell.inner());
            }
        }
        result.into_boxed_slice()
    }

    pub fn generate_moves(&self, piece_idx: usize) -> Vec<Move> {
        let piece = Piece::from_index(piece_idx);
        r#move::move_dijkstra(self.state.board, piece)
            .into_iter()
            .flatten()
            .collect()
    }
}

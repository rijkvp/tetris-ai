#[cfg(not(feature = "wasm"))]
use crate::board::Board;
use crate::feature::{Weights, WeightsMap};
use crate::r#move::{Move, Path, move_dijkstra};
use crate::rng::gen_random_piece;
use crate::state::State;
#[cfg(not(feature = "wasm"))]
use crate::state::Stats;
use rand::Rng;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
#[wasm_bindgen(start)]
pub fn wasm_main() {
    console_error_panic_hook::set_once();
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Default)]
pub struct Simulator {
    state: State,
    weights: Weights,
    current_path: Option<Path>,
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
        self.current_path = None;
    }

    #[cfg(not(feature = "wasm"))]
    pub fn stats(&self) -> Stats {
        self.state.stats()
    }

    #[cfg(not(feature = "wasm"))]
    pub fn board(&self) -> &Board {
        self.state.board()
    }

    pub fn run(&mut self) {
        while self.step() {}
    }

    pub fn step(&mut self) -> bool {
        let piece = gen_random_piece(self.state.delta().map(|d| d.r#move.piece.index()));

        // Use resivoir sampling to ramdomly select one of the best possible moves
        let mut chosen = None;
        let mut best_score = f64::NEG_INFINITY;
        let mut count = 0;
        let mut rng = rand::rng();
        for path in move_dijkstra(self.state.board(), piece, self.state.stats().level) {
            let future = self.state.future(path.final_move());
            let score = self.weights.evaluate(&future);
            if score > best_score {
                best_score = score;
                chosen = Some((future, path));
                count = 1;
            } else if score == best_score {
                count += 1;
                if rng.random_range(0..count) == 0 {
                    chosen = Some((future, path));
                }
            }
        }

        if let Some((next, path)) = chosen {
            self.state = next; // update state
            self.current_path = Some(path);
            return true;
        }
        self.state.set_game_over();
        false
    }

    pub fn update_weights(&mut self, weights_map: WeightsMap) {
        self.weights = weights_map.into();
    }

    #[cfg(feature = "wasm")]
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn state(&self) -> JsValue {
        self.state.js_value()
    }

    #[cfg(feature = "wasm")]
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn path(&self) -> Option<Path> {
        self.current_path.clone()
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Default)]
pub struct Game {
    state: State,
    current_move: Option<Move>,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Game {
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.state = State::default();
        self.current_move = None;
    }

    pub fn step(&mut self) -> bool {
        let piece = gen_random_piece(self.state.delta().map(|d| d.r#move.piece.index()));
        if let Some(current_move) = self.current_move {
            let next_move = current_move.drop(self.state.board());
            if next_move.is_none() {
                self.state = self.state.future(current_move);
            }
            self.current_move = next_move;
        } else {
            let start_move = piece.into_start_move();
            if start_move.is_valid(self.state.board()) {
                self.current_move = Some(start_move);
            } else {
                self.state.set_game_over();
                return false;
            }
        }
        true
    }

    #[inline]
    fn try_move(&mut self, move_change: impl FnOnce(Move) -> Move) {
        if let Some(next_move) = self.current_move.map(move_change) {
            if next_move.is_valid(self.state.board()) {
                self.current_move = Some(next_move);
            }
        }
    }

    pub fn move_left(&mut self) {
        self.try_move(|mut m| {
            m.pos.col -= 1;
            m
        })
    }

    pub fn move_right(&mut self) {
        self.try_move(|mut m| {
            m.pos.col += 1;
            m
        })
    }

    pub fn move_down(&mut self) {
        self.try_move(|mut m| {
            m.pos.row += 1;
            m
        })
    }

    pub fn hard_drop(&mut self) {
        while let Some(next_move) = self.current_move.and_then(|m| m.drop(self.state.board())) {
            self.current_move = Some(next_move);
        }
    }

    pub fn rotate(&mut self) {
        self.try_move(|mut m| {
            m.pos.rot = (m.pos.rot + 1) % m.piece.num_rotations();
            m
        })
    }

    #[cfg(feature = "wasm")]
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn state(&self) -> JsValue {
        self.state.js_value()
    }

    #[cfg(feature = "wasm")]
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn r#move(&self) -> Option<Move> {
        self.current_move
    }
}

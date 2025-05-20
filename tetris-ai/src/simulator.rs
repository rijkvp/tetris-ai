use crate::board::Board;
use crate::feature::{Weights, WeightsMap};
use crate::r#move::{Path, move_dijkstra};
use crate::rng::gen_random_piece;
use crate::state::{State, Stats};
use rand::Rng;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Simulator {
    state: State,
    weights: Weights,
    current_path: Option<Path>,
    time_pressure: bool,
}

impl Simulator {
    pub fn new_with_weights(weights: Weights) -> Self {
        Self {
            weights,
            ..Self::default()
        }
    }

    pub fn stats(&self) -> Stats {
        self.state.stats()
    }

    pub fn board(&self) -> &Board {
        self.state.board()
    }
}

impl Default for Simulator {
    fn default() -> Self {
        Self {
            state: State::default(),
            weights: Weights::default(),
            current_path: None,
            time_pressure: true,
        }
    }
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

    pub fn run(&mut self) {
        while self.step() {}
    }

    pub fn run_for(&mut self, iterations: usize) {
        for _ in 0..iterations {
            if !self.step() {
                break;
            }
        }
    }

    pub fn step(&mut self) -> bool {
        let piece = gen_random_piece(self.state.delta().map(|d| d.r#move.piece.index()));

        // Use resivoir sampling to ramdomly select one of the best possible moves
        let mut chosen = None;
        let mut best_score = f64::NEG_INFINITY;
        let mut count = 0;
        let mut rng = rand::rng();
        for path in move_dijkstra(
            self.state.board(),
            piece,
            if self.time_pressure {
                Some(self.state.stats().level)
            } else {
                None
            },
        ) {
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

    pub fn set_time_pressure(&mut self, time_pressure: bool) {
        self.time_pressure = time_pressure;
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

use crate::r#move::Move;
use crate::rng::gen_random_piece;
use crate::state::State;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

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
        if let Some(current_move) = self.current_move {
            // move the current piece down
            let next_move = current_move.drop(self.state.board());
            if next_move.is_none() {
                // if the piece is dropped, update the state
                self.state = self.state.future(current_move);
            }
            self.current_move = next_move;
        } else {
            // spawn the next piece
            let piece = gen_random_piece(self.state.delta().map(|d| d.r#move.piece.index()));
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

    pub fn soft_drop(&mut self) {
        self.try_move(|mut m| {
            m.pos.row += 1;
            m
        });
    }

    pub fn hard_drop(&mut self) {
        while let Some(next_move) = self.current_move.and_then(|m| m.drop(self.state.board())) {
            self.current_move = Some(next_move);
        }
        self.step(); // don't waste the next tick doing nothing
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

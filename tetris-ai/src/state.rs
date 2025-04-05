#[cfg(feature = "wasm")]
use crate::board::{BOARD_HEIGHT, BOARD_WIDTH};
use crate::{board::Board, r#move::Move};
use serde::Serialize;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[derive(Debug, Default, Copy, Clone, Serialize)]
pub struct Stats {
    pub steps: u64,
    pub lines: u64,
    pub score: u64,
    pub level: u64,
    pub tetrises: u64,
}

#[derive(Debug, Default, Clone)]
pub struct State {
    board: Board,
    stats: Stats,
    game_over: bool,
    delta: Option<Delta>,
}

#[derive(Debug, Clone)]
pub struct Delta {
    pub r#move: Move,
    pub eroded: usize,
    #[cfg(test)]
    pub cleared: Vec<usize>,
}

#[cfg(feature = "wasm")]
#[derive(Serialize)]
struct WasmState<'a> {
    board: &'a [[u8; BOARD_WIDTH]; BOARD_HEIGHT],
    stats: Stats,
    #[serde(rename = "gameOver")]
    game_over: bool,
}

const POINTS_PER_CLEARED_ROWS: [u64; 5] = [0, 40, 100, 300, 1200];

impl State {
    pub fn new(board: Board) -> Self {
        Self {
            board,
            ..Default::default()
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn stats(&self) -> Stats {
        self.stats
    }

    pub fn game_over(&self) -> bool {
        self.game_over
    }

    pub fn set_game_over(&mut self) {
        self.game_over = true;
    }

    pub fn delta(&self) -> Option<&Delta> {
        self.delta.as_ref()
    }

    /// Computes the new 'future' state after a piece has been moved.
    pub(crate) fn future(&self, r#move: Move) -> Self {
        let mut board = self.board;
        board.imprint(
            r#move.piece.rotation(r#move.pos.rot),
            r#move.pos.row,
            r#move.pos.col,
            r#move.piece.cell(),
        );
        let cleared_rows = board.clear_full();
        let cleared = cleared_rows.len();

        // count the number of cells in the piece that were cleared by the move (eroded cells)
        let mut eroded = 0;
        for row in cleared_rows.iter().copied() {
            let pattern = r#move.pattern();
            let pos = r#move.pos;
            if row as isize >= pos.row && (row as isize) < pos.row + pattern.cols() as isize {
                if let Some(row) = pattern.get_row(row - pos.row as usize) {
                    for filled in row {
                        if *filled {
                            eroded += 1;
                        }
                    }
                }
            }
        }

        let new_lines = self.stats.lines + cleared as u64;
        Self {
            board,
            stats: Stats {
                steps: self.stats.steps + 1,
                lines: new_lines,
                score: self.stats.score + POINTS_PER_CLEARED_ROWS[cleared] * (self.stats.level + 1),
                level: new_lines / 10,
                tetrises: self.stats.tetrises + if cleared == 4 { 1 } else { 0 },
            },
            game_over: false,
            delta: Some(Delta {
                r#move,
                eroded,
                #[cfg(test)]
                cleared: cleared_rows,
            }),
        }
    }

    #[cfg(feature = "wasm")]
    pub fn js_value(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&WasmState {
            board: self.board.get_raw_data(),
            stats: self.stats(),
            game_over: self.game_over,
        })
        .unwrap()
    }

    #[cfg(test)]
    pub(crate) fn test_delta(mut self, delta: Delta) -> Self {
        self.delta = Some(delta);
        self
    }
}

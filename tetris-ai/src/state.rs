#[cfg(feature = "wasm")]
use crate::board::{BOARD_HEIGHT, BOARD_WIDTH};
use crate::{board::Board, r#move::Move};
use serde::Serialize;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[derive(Debug, Default, Clone, Serialize)]
pub struct Stats {
    pub steps: u64,
    pub lines: u64,
    pub score: u64,
    pub level: u64,
    pub tetrises: u64,
}

#[derive(Debug, Default, Clone)]
pub struct State {
    pub board: Board,
    pub steps: u64,
    pub cleared_rows: u64,
    pub score: u64,
    pub tetrises: u64,
    pub delta: Option<Delta>,
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
}

const POINTS_PER_CLEARED_ROWS: [u64; 5] = [0, 40, 100, 300, 1200];

impl State {
    pub fn new(board: Board) -> Self {
        Self {
            board,
            steps: 0,
            cleared_rows: 0,
            delta: None,
            score: 0,
            tetrises: 0,
        }
    }

    pub fn level(&self) -> u64 {
        self.cleared_rows / 10
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

        Self {
            board,
            steps: self.steps + 1,
            delta: Some(Delta {
                r#move,
                eroded,
                #[cfg(test)]
                cleared: cleared_rows,
            }),
            cleared_rows: self.cleared_rows + cleared as u64,
            score: self.score + POINTS_PER_CLEARED_ROWS[cleared] * (self.level() + 1),
            tetrises: self.tetrises + if cleared == 4 { 1 } else { 0 },
        }
    }

    pub fn stats(&self) -> Stats {
        Stats {
            steps: self.steps,
            lines: self.cleared_rows,
            score: self.score,
            level: self.level(),
            tetrises: self.tetrises,
        }
    }

    #[cfg(feature = "wasm")]
    pub fn js_value(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&WasmState {
            board: self.board.get_raw_data(),
            stats: self.stats(),
        })
        .unwrap()
    }
}

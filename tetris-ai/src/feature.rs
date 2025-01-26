use crate::{
    board::{BOARD_HEIGHT, BOARD_WIDTH},
    state::State,
};
use std::cmp::{max, min};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Weights {
    pub row_trans: f64,
    pub col_trans: f64,
    pub cuml_wells: f64,
    pub pits: f64,
    pub landing_height: f64,
    pub eroded_cells: f64,
}

impl Default for Weights {
    fn default() -> Self {
        Self {
            row_trans: -2.700,
            col_trans: -6.786,
            cuml_wells: -0.396,
            pits: -12.668,
            landing_height: -3.383,
            eroded_cells: -9.277,
        }
    }
}

#[wasm_bindgen]
impl Weights {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()

    }

    fn to_array(&self) -> [(fn(&State) -> usize, f64); 6] {
        [
            (row_trans, self.row_trans),
            (col_trans, self.col_trans),
            (cuml_wells, self.cuml_wells),
            (pits, self.pits),
            (landing_height, self.landing_height),
            (eroded_cells, self.eroded_cells),
        ]
    }
}

pub fn evaluate_default(state: &State) -> f64 {
    evaluate(state, &Weights::default())
}

pub fn evaluate(state: &State, weights: &Weights) -> f64 {
    let mut score = 0.0;
    for (feature, weight) in weights.to_array().iter() {
        score += feature(state) as f64 * weight;
    }
    score
}

/// The number of times that two adjacent cells in the same row mismatch.
fn row_trans(state: &State) -> usize {
    let mut sum = 0;
    for r in 0..BOARD_HEIGHT {
        for c in 0..BOARD_WIDTH - 1 {
            if state.board[(r, c)].filled() != state.board[(r, c + 1)].filled() {
                sum += 1;
            }
        }
    }
    sum
}

/// The number of times that two adjacent cells in the same column mismatch.
fn col_trans(state: &State) -> usize {
    let mut sum = 0;
    for c in 0..BOARD_WIDTH {
        for r in 0..BOARD_HEIGHT - 1 {
            if state.board[(r, c)].filled() != state.board[(r + 1, c)].filled() {
                sum += 1;
            }
        }
    }
    sum
}

/// The depth of each column with respect to its adjacent columns.
/// If a column is not shorter than both of its neighbors, it has a value of 0.
/// Otherwise, its value is how much shorter it is than its shortest neighbor.
fn wells(state: &State) -> [i64; BOARD_WIDTH] {
    let heights = state.board.heights().map(|x| x as i64);
    let mut wells = [0i64; BOARD_WIDTH];
    for i in 1..BOARD_WIDTH - 1 {
        wells[i] = max(0, min(heights[i - 1], heights[i + 1]) - heights[i]);
    }
    wells[0] = max(0, heights[1] - heights[0]);
    wells[BOARD_WIDTH - 1] = max(0, heights[BOARD_WIDTH - 2] - heights[BOARD_WIDTH - 1]);
    wells
}

/// The sum from 1 to wells.
// Computed using the formula for the sum of natural numbers.
fn cuml_wells(state: &State) -> usize {
    wells(&state)
        .into_iter()
        .map(|x| x * (x + 1) / 2)
        .sum::<i64>() as usize
}

/// The number of empty cells that have at least one filled cell above them.
fn pits(state: &State) -> usize {
    let mut total = 0;
    for c in 0..BOARD_WIDTH {
        for r in BOARD_HEIGHT - state.board.height(c)..BOARD_HEIGHT {
            if state.board[(r, c)].empty() {
                total += 1;
            }
        }
    }
    total
}

/// The height of the row containing the bottom-most cell of the previously placed piece.
fn landing_height(state: &State) -> usize {
    BOARD_HEIGHT
        - state
            .delta
            .as_ref()
            .map(|delta| {
                let piece_height = delta.piece.rotation(delta.r#move.rot).rows();
                delta.r#move.row.max(0) as usize + piece_height
            })
            .unwrap_or(0)
            .min(BOARD_HEIGHT)
}

/// The number of cells that were cleared from the previously placed piece.
fn eroded_cells(state: &State) -> usize {
    state.delta.as_ref().map(|delta| delta.eroded).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    const TEST_ITERATIONS: usize = 100;

    const TEST_FEATURES: &[(&'static str, fn(&State) -> usize)] = &[
        ("row_trans", row_trans),
        ("col_trans", col_trans),
        ("cuml_wells", cuml_wells),
        ("pits", pits),
        ("landing_height", landing_height),
        ("eroded_cells", eroded_cells),
    ];

    #[test]
    fn test_features_random_board() {
        for (feature_name, feature) in TEST_FEATURES {
            for _ in 0..TEST_ITERATIONS {
                let state = State::new(test::random_board());
                let py_output = test::run_py_feature(&state, feature_name);
                let rust_output = feature(&state);
                if py_output != rust_output {
                    panic!(
                        "Mismatch for feature {}\nPython: {}\nRust: {}\nBoard {}\nHeights: {:?}",
                        feature_name,
                        py_output,
                        rust_output,
                        state.board,
                        state.board.heights()
                    );
                }
            }
        }
    }

    #[test]
    fn test_features_random_state() {
        for (feature_name, feature) in TEST_FEATURES {
            for _ in 0..TEST_ITERATIONS {
                let state = test::random_state();
                let py_output = test::run_py_feature(&state, feature_name);
                let rust_output = feature(&state);
                if py_output != rust_output {
                    if let Some(delta) = &state.delta {
                        println!("Piece rotation: {}", delta.piece.rotation(delta.r#move.rot));
                    }
                    panic!(
                        "Mismatch for feature {}\nPython: {}\nRust: {}\nBoard {}\nHeights: {:?}\nDelta: {:?}",
                        feature_name,
                        py_output,
                        rust_output,
                        state.board,
                        state.board.heights(),
                        state.delta
                    );
                }
            }
        }
    }
}

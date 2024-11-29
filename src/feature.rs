use crate::{board::BOARD_WIDTH, state::State};
use std::cmp::{max, min};

/// The number of times that two adjacent cells in the same row mismatch.
fn row_trans(state: &State) -> usize {
    let mut sum = 0;
    for row in 0..state.board.rows() {
        for col in 0..state.board.cols() - 1 {
            if state.board[(row, col)] != state.board[(row, col + 1)] {
                sum += 1;
            }
        }
    }
    sum
}

/// The number of times that two adjacent cells in the same column mismatch.
fn col_trans(state: &State) -> usize {
    let mut sum = 0;
    for col in 0..state.board.cols() {
        for row in 0..state.board.rows() - 1 {
            if state.board[(row, col)] != state.board[(row + 1, col)] {
                sum += 1;
            }
        }
    }
    sum
}

/// The depth of each column with respect to its adjacent columns, as a list.
/// If a column is not shorter than both of its neighbors, it has a value of 0.
/// Otherwise, its value is how much shorter it is than its shortest neighbor.
fn wells(state: &State) -> [usize; BOARD_WIDTH] {
    let heights = state.board.heights();
    let mut wells = [0; BOARD_WIDTH];
    for i in 1..state.board.cols() - 1 {
        wells[i] = max(0, min(heights[i - 1], heights[i + 1]) - heights[i]);
    }
    wells
}

/// The sum from 1 to wells.
// Computed using the formula for the sum of natural numbers.
fn cuml_wells(state: &State) -> usize {
    let wells = wells(&state);
    wells.iter().map(|&x| x * (x + 1) / 2).sum()
}

fn pits(_state: &State) -> usize {
    todo!()
}

fn landing_height(state: &State) -> usize {
    state.board.rows()
        + state
            .delta
            .map(|delta| {
                let piece_height = delta.piece.get_rotation(delta.r#move.rot).rows();
                delta.r#move.row + piece_height
            })
            .unwrap_or(0)
}

fn eroded_cells(_state: &State) -> usize {
    todo!()
}

pub type FeatureWeights<'a> = &'a [(fn(&State) -> usize, f64)];

pub const DEFAULT_WEIGHTS: FeatureWeights = &[
    (row_trans, -2.700),
    (col_trans, -6.786),
    (cuml_wells, -0.396),
    // (pits, -12.668),
    (landing_height, -3.383),
    // (eroded_cells, -9.277),
];

pub fn evaluate(state: &State, weights: FeatureWeights) -> f64 {
    let mut score = 0.0;
    for (feature, weight) in weights {
        score += feature(state) as f64 * weight;
    }
    score
}

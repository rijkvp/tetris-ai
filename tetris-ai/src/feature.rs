use crate::{
    board::{BOARD_HEIGHT, BOARD_WIDTH},
    state::State,
};
use std::cmp::{max, min};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

type FeatureFn = fn(&State) -> usize;

const PRESET_MAP: [(&str, &[(&str, f64)]); 2] = [
    (
        "infinite",
        &[
            ("col_trans", -8.4),
            ("row_trans", -2.4),
            ("pits", -10.0),
            ("landing_height", -5.0),
            ("eroded_cells", 10.0),
            ("cuml_wells", -3.5),
        ],
    ),
    (
        "score",
        &[
            ("col_trans", -6.8),
            ("row_trans", -2.7),
            ("pits", -12.7),
            ("landing_height", -3.8),
            ("eroded_cells", -10.0),
            ("cuml_wells", -0.4),
        ],
    ),
];

const FEATURE_LOOKUP: [(&str, FeatureFn); 6] = [
    ("col_trans", col_trans),
    ("row_trans", row_trans),
    ("pits", pits),
    ("landing_height", landing_height),
    ("eroded_cells", eroded_cells),
    ("cuml_wells", cuml_wells),
];

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, Clone)]
pub struct WeightsMap(Vec<(String, f64)>);

impl Default for WeightsMap {
    fn default() -> Self {
        WeightsMap(
            FEATURE_LOOKUP
                .iter()
                .map(|(name, _)| (name.to_string(), 0.0))
                .collect::<Vec<_>>(),
        )
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl WeightsMap {
    #[cfg(feature = "wasm")]
    pub fn defaults() -> Self {
        Self::default()
    }

    #[cfg(feature = "wasm")]
    pub fn preset(preset: &str) -> Self {
        if let Some((_, weight_map)) = PRESET_MAP.iter().find(|(name, _)| name == &preset) {
            Self(
                weight_map
                    .iter()
                    .map(|(name, weight)| (name.to_string(), *weight))
                    .collect(),
            )
        } else {
            panic!("Unknown preset: {}", preset);
        }
    }

    #[cfg(feature = "wasm")]
    pub fn from_js(val: wasm_bindgen::JsValue) -> Self {
        Self(serde_wasm_bindgen::from_value(val).unwrap())
    }

    #[cfg(feature = "wasm")]
    pub fn into_js(self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.0).unwrap()
    }
}

impl From<WeightsMap> for Weights {
    fn from(map: WeightsMap) -> Self {
        Self::from_iter(map.0.iter().map(|(name, weight)| (name.as_str(), *weight)))
    }
}

#[derive(Debug, Default, Clone)]
pub struct Weights(Vec<(FeatureFn, f64)>);

impl Weights {
    pub fn from_preset(preset: &str) -> Self {
        if let Some((_, weight_map)) = PRESET_MAP.iter().find(|(name, _)| name == &preset) {
            Weights::from_iter(weight_map.iter().copied())
        } else {
            panic!("Unknown preset: {}", preset);
        }
    }

    fn from_iter<'a>(iter: impl Iterator<Item = (&'a str, f64)>) -> Self {
        let mut weights = Vec::with_capacity(FEATURE_LOOKUP.len());
        for (name, weight) in iter.into_iter() {
            if let Some((_, feature)) = FEATURE_LOOKUP.iter().find(|(n, _)| n == &name) {
                weights.push((*feature, weight));
            } else {
                panic!("Unknown feature: {}", name);
            }
        }
        Weights(weights)
    }

    pub fn evaluate(&self, state: &State) -> f64 {
        let mut score = 0.0;
        for (feature, weight) in self.0.iter() {
            score += feature(state) as f64 * weight;
        }
        score
    }
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
    wells(state)
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

    const TEST_FEATURES: &[(&str, FeatureFn)] = &[
        ("col_trans", col_trans),
        ("row_trans", row_trans),
        ("pits", pits),
        // ("landing_height", landing_height), // TOFIX: differs from Python since piece includes empty rows/columns
        ("eroded_cells", eroded_cells),
        ("cuml_wells", cuml_wells),
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

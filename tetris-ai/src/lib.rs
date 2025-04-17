pub mod board;
pub mod feature;
pub mod r#move;
pub mod piece;
pub mod rng;
pub mod simulator;
pub mod state;
#[cfg(test)]
pub mod test;
pub mod train;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
#[wasm_bindgen(start)]
pub fn wasm_main() {
    console_error_panic_hook::set_once();
}

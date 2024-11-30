use std::time::Instant;

use board::{BOARD_HEIGHT, BOARD_WIDTH};
use piece::Piece;
use state::{Move, State};

pub mod board;
pub mod feature;
pub mod piece;
pub mod state;

/// Returns all possible moves when dropping a piece.
pub fn move_drop(state: &State, piece: Piece) -> Vec<Move> {
    let board = &state.board;
    let mut moves = Vec::new();
    // for each rotation of the piece
    for (rot, pattern) in piece.rotations().enumerate() {
        // for each column where the piece can be placed
        for col in 0..=(BOARD_WIDTH - pattern.cols()) {
            // the highest point where the pattern can be placed
            let highest = (0..pattern.cols())
                .map(|c| board.height(col + c))
                .max()
                .unwrap();
            let mut row = (BOARD_HEIGHT - highest).saturating_sub(pattern.rows()); // saturating_sub to avoid underflow
            if !board.overlaps(&pattern, row, col) {
                while !board.overlaps(&pattern, row + 1, col) {
                    row += 1;
                }
                moves.push(Move { rot, row, col });
            }
        }
    }
    moves
}

pub fn simulate(
    mut state: State,
    piece_gen: impl Fn() -> Piece,
    move_gen: impl Fn(&State, Piece) -> Vec<Move>,
    move_policy: impl Fn(Vec<State>) -> Option<State>,
) {
    let mut piece;
    let mut i = 0;
    let start = Instant::now();
    loop {
        // println!("\n\nIteration {i}");
        piece = piece_gen(); // generate a new piece
                             // println!("Piece: {}", piece);
                             // println!("Board: {}", state.board);

        // TODO: every state is a copy in this pool, optimize it
        let pool = move_gen(&state, piece)
            .into_iter()
            .map(|r#move| state.future(piece, r#move))
            .collect::<Vec<_>>();

        // println!("Pool size: {}", pool.len());

        if let Some(next) = move_policy(pool) {
            state = next; // update state
        } else {
            // no possible moves, game over
            break;
        }
        i += 1;
    }
    let elapsed = start.elapsed();
    println!(
        "moves: {}, elapsed: {:.2}s, moves/sec: {:.2}",
        i,
        elapsed.as_secs_f64(),
        i as f64 / elapsed.as_secs_f64()
    );
    println!("Final board: {}", state.board);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;
    use pyo3::ffi::c_str;
    use pyo3::prelude::*;
    use pyo3::types::PyTuple;
    use rand::Rng;

    const TEST_ITERATIONS: usize = 100;

    /// Generates a random board with no gaps.
    fn random_board() -> Board {
        let mut board = Board::default();
        let mut rng = rand::thread_rng();
        let iterations = rng.gen_range(0..=BOARD_HEIGHT * BOARD_WIDTH);
        for _ in 0..iterations {
            let col = rng.gen_range(0..BOARD_WIDTH);
            let col_height = board.height(col);
            if col_height == BOARD_HEIGHT {
                continue;
            }
            let row = BOARD_HEIGHT - col_height - 1;
            board.fill_cell(row, col);
        }
        board
    }

    fn run_py(state: &State, piece: Piece) -> Vec<Move> {
        let board_data = state
            .board
            .get_data()
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|cell| cell.occupied())
                    .collect::<Vec<bool>>()
            })
            .collect::<Vec<_>>();
        Python::with_gil(|py| {
            let py_mod = PyModule::from_code(
                py,
                c_str!(include_str!("../tetris.py")),
                c_str!("tetris.py"),
                c_str!("tetris"),
            )
            .unwrap();
            let py_state = py_mod
                .call_method(
                    "import_state",
                    PyTuple::new(py, &[board_data]).unwrap(),
                    None,
                )
                .unwrap();
            let py_zoid = py_mod
                .call_method(
                    "import_zoid",
                    PyTuple::new(py, &[piece.get_index()]).unwrap(),
                    None,
                )
                .unwrap();

            let output = py_mod
                .call_method(
                    "collect_moves",
                    PyTuple::new(py, &[py_state, py_zoid]).unwrap(),
                    None,
                )
                .unwrap();
            let output = output.extract::<Vec<(usize, usize, usize)>>().unwrap();
            output
                .into_iter()
                .map(|(rot, row, col)| Move { rot, row, col })
                .collect()
        })
    }

    #[test]
    fn test_move_drop() {
        for _ in 0..TEST_ITERATIONS {
            let state = State::new(random_board());
            // println!("{}", state.board);
            // println!("Heights: {:?}", state.board.heights());
            for piece_index in 0..piece::N_PIECES {
                let piece = Piece::from_index(piece_index);
                let py_output = run_py(&state, piece);
                let rust_output = move_drop(&state, piece);
                if py_output != rust_output {
                    panic!(
                        "move_drop mismatch for piece {}\nPython ({}): {:?}\nRust ({}): {:?}\nBoard {}",
                        piece, py_output.len(), py_output, rust_output.len(), rust_output, state.board
                    );
                }
            }
        }
    }
}

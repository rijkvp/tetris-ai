use crate::board::Board;
use crate::board::{BOARD_HEIGHT, BOARD_WIDTH};
use crate::piece::Piece;
use crate::r#move::move_drop;
use crate::state::{Move, State};
use pyo3::ffi::c_str;
use pyo3::types::{PyDict, PyTuple};
use pyo3::{prelude::*, IntoPyObjectExt};
use rand::Rng;

/// Generates a random board.
pub fn random_board() -> Board {
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
    // make some cells empty
    for r in 0..BOARD_HEIGHT {
        for c in 0..BOARD_WIDTH {
            if board[(r, c)].filled() {
                if rng.gen_bool(0.05) {
                    board.clear_cell(r, c);
                }
            }
        }
    }
    board.clear_full();
    board
}

/// Generates a random state by dropping pieces until no more moves are possible.
pub fn random_state() -> State {
    let mut state = State::new(Board::default());
    let mut rng = rand::thread_rng();
    loop {
        let piece = Piece::from_index(rng.gen_range(0..7));
        let possible_moves = move_drop(&state, piece);
        if possible_moves.is_empty() {
            break;
        }
        let r#move = possible_moves[rng.gen_range(0..possible_moves.len())];
        println!("Previous board:{}", state.board);
        state = state.future(piece, r#move);
    }
    state
}

/// Runs a Python feature function.
pub fn run_py_feature(state: &State, feature_name: &str) -> usize {
    let board_data = state
        .board
        .get_data()
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|cell| cell.filled())
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
        let delta_dict = PyDict::new(py);
        if let Some(delta) = &state.delta {
            delta_dict
                .set_item("piece_idx", delta.piece.get_index())
                .unwrap();
            delta_dict.set_item("rot", delta.r#move.rot).unwrap();
            delta_dict.set_item("col", delta.r#move.col).unwrap();
            delta_dict.set_item("row", delta.r#move.row).unwrap();
            delta_dict
                .set_item("cleared", delta.cleared.clone())
                .unwrap();
        }

        let args_tuple = PyTuple::new(
            py,
            &[board_data.into_py_any(py).unwrap(), delta_dict.into()],
        )
        .unwrap();

        let py_state = py_mod
            .call_method("import_state", args_tuple, None)
            .unwrap();

        let output = py_mod
            .call_method(feature_name, PyTuple::new(py, &[py_state]).unwrap(), None)
            .unwrap();
        output.extract::<usize>().unwrap()
    })
}

/// Runs a Python move function, and collects the output into a Vec<Move>.
pub fn run_py_move(state: &State, piece: Piece) -> Vec<Move> {
    let board_data = state
        .board
        .get_data()
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|cell| cell.filled())
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
                PyTuple::new(
                    py,
                    &[board_data.into_py_any(py).unwrap(), PyDict::new(py).into()],
                )
                .unwrap(),
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

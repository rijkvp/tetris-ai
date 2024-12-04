use crate::board::{BOARD_HEIGHT, BOARD_WIDTH};
use crate::piece::Piece;
use crate::state::{Move, State};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{piece, test};

    const TEST_ITERATIONS: usize = 100;

    #[test]
    fn test_move_drop() {
        for _ in 0..TEST_ITERATIONS {
            let state = State::new(test::random_board());
            for piece_index in 0..piece::N_PIECES {
                let piece = Piece::from_index(piece_index);
                let py_output = test::run_py_move(&state, piece);
                let rust_output = move_drop(&state, piece);
                if py_output != rust_output {
                    let mut diff = vec![];
                    for item in rust_output.iter() {
                        if !py_output.contains(&item) {
                            diff.push(item);
                        }
                    }
                    for diff in diff.iter() {
                        let mut board = state.board.clone();
                        let pattern = piece.get_rotation(diff.rot);
                        println!("Pattern: {}", pattern);
                        println!("Position: {},{}", diff.row, diff.col);
                        board.imprint(pattern, diff.row, diff.col);
                        println!("Modified board: {}", board);
                    }
                    panic!(
                        "move_drop mismatch for {}\nPython ({}): {:?}\nRust ({}): {:?}\nDifference: {:?}\nBoard {}",
                        piece,
                        py_output.len(),
                        py_output,
                        rust_output.len(),
                        rust_output,
                        diff,
                        state.board
                    );
                }
            }
        }
    }
}

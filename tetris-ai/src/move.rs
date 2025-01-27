use crate::board::{BOARD_HEIGHT, BOARD_WIDTH, Board};
use crate::piece::Piece;
use std::collections::{BinaryHeap, HashMap};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Move {
    pub rot: usize,
    pub row: isize,
    pub col: isize,
}

/// Returns all possible moves when dropping a piece.
pub fn move_drop(board: Board, piece: Piece) -> Vec<Move> {
    let mut moves = Vec::new();
    // for each rotation of the piece
    for rot in 0..piece.num_rotations() {
        let pattern = piece.rotation(rot);
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
                moves.push(Move {
                    rot,
                    row: row as isize,
                    col: col as isize,
                });
            }
        }
    }
    moves
}

fn spawn_position(piece: Piece) -> Move {
    let offset = piece.spawn_offset();
    Move {
        rot: 0,
        row: 0 - offset.0,
        col: (BOARD_WIDTH / 2) as isize - offset.1,
    }
}

fn next_moves(current: Move, piece: Piece, board: &Board) -> Vec<Move> {
    let mut candidates = Vec::new();
    // Rotate or move left/right/down
    if piece.num_rotations() > 1 {
        candidates.push(Move {
            rot: (current.rot + 1) % piece.num_rotations(),
            row: current.row,
            col: current.col,
        });
        if piece.num_rotations() > 2 {
            candidates.push(Move {
                rot: current.rot.wrapping_sub(1) % piece.num_rotations(),
                row: current.row,
                col: current.col,
            });
        }
    }
    candidates.push(Move {
        rot: current.rot,
        row: current.row,
        col: current.col - 1, // left
    });
    candidates.push(Move {
        rot: current.rot,
        row: current.row,
        col: current.col + 1, // right
    });
    candidates.push(Move {
        rot: current.rot,
        row: current.row + 1, // down
        col: current.col,
    });
    candidates
        .into_iter()
        .filter(|m| !board.overlaps_move(piece, *m))
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Node {
    r#move: Move,
    cost: u64,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn touches_ground(piece: Piece, r#move: Move, board: &Board) -> bool {
    board.overlaps_move(piece, Move {
        rot: r#move.rot,
        row: r#move.row + 1,
        col: r#move.col,
    })
}

pub fn move_dijkstra(board: Board, piece: Piece) -> Vec<Vec<Move>> {
    let mut distance = HashMap::<Move, u64>::new();
    let mut cost = HashMap::<Move, Move>::new();
    let mut to_visit = BinaryHeap::new();
    let mut destinations = Vec::new();

    let start_move = spawn_position(piece);
    if board.overlaps_move(piece, start_move) {
        return Vec::new();
    }
    distance.insert(start_move, 0);
    to_visit.push(Node {
        r#move: start_move,
        cost: 0,
    });

    while let Some(Node {
        r#move: current,
        cost: current_cost,
    }) = to_visit.pop()
    {
        if touches_ground(piece, current, &board) {
            destinations.push(current);
        }
        for next in next_moves(current, piece, &board).into_iter() {
            let new_cost = if next.row != current.row || next.col != current.col {
                current_cost + 1
            } else {
                current_cost
            };
            if distance.get(&next).is_none() || new_cost < distance[&next] {
                distance.insert(next, new_cost);
                to_visit.push(Node {
                    r#move: next,
                    cost: new_cost,
                });
                cost.insert(next, current);
            }
        }
    }
    destinations
        .into_iter()
        .map(|end| {
            let mut current = end;
            let mut path = Vec::new();
            while current != start_move {
                path.push(current);
                current = cost[&current];
            }
            path.push(start_move);
            path.reverse();
            path
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{piece, state::State, test};

    const TEST_ITERATIONS: usize = 100;

    #[test]
    fn test_move_drop() {
        for _ in 0..TEST_ITERATIONS {
            let state = State::new(test::random_board());
            for piece_index in 0..piece::N_PIECES {
                let piece = Piece::from_index(piece_index);
                let mut py_output = test::run_py_move(&state, piece);
                py_output.sort();
                let mut rust_output = move_drop(state.board, piece);
                rust_output.sort();
                if py_output != rust_output {
                    let mut diff = vec![];
                    for item in rust_output.iter() {
                        if !py_output.contains(&item) {
                            diff.push(item);
                        }
                    }
                    println!("{} differences found", diff.len());
                    for diff in diff.iter() {
                        let mut board = state.board.clone();
                        let pattern = piece.rotation(diff.rot);
                        println!("Pattern: {}", pattern);
                        println!("Position: {},{}", diff.row, diff.col);
                        board.imprint(pattern, diff.row, diff.col, piece.cell());
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

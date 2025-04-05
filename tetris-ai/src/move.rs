use crate::board::{BOARD_HEIGHT, BOARD_WIDTH, Board};
#[cfg(feature = "wasm")]
use crate::piece::WasmPattern;
use crate::piece::{Pattern, Piece};
use serde::Serialize;
use std::collections::{BinaryHeap, HashMap};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Move {
    pub piece: Piece,
    pub pos: Position,
}

impl Move {
    pub fn pattern(&self) -> Pattern {
        self.piece.rotation(self.pos.rot)
    }

    pub fn is_valid(&self, board: &Board) -> bool {
        !board.overlaps_move(*self)
    }

    pub fn drop(mut self, board: &Board) -> Option<Move> {
        self.pos.row += 1;
        if !board.overlaps_move(self) {
            Some(self)
        } else {
            None
        }
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Move {
    #[cfg(feature = "wasm")]
    pub fn get_pattern(&self) -> WasmPattern {
        self.piece.rotation(self.pos.rot).into_wasm()
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize)]
pub struct Position {
    pub rot: usize,
    pub row: isize,
    pub col: isize,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Path {
    piece: Piece,
    positions: Vec<Vec<Position>>,
}

impl Path {
    fn from_path_reverse(mut reversed_path: Vec<Position>, piece: Piece) -> Self {
        debug_assert!(!reversed_path.is_empty());
        let mut moves_per_tick = Vec::new();
        let mut current_row = reversed_path.last().unwrap().row;
        let mut current_tick = Vec::new();
        while let Some(r#move) = reversed_path.pop() {
            if r#move.row == current_row {
                current_tick.push(r#move);
            } else {
                moves_per_tick.push(current_tick);
                current_tick = Vec::with_capacity(MAX_MOVES_PER_TICK as usize);
                current_tick.push(r#move);
                current_row = r#move.row;
            }
        }
        moves_per_tick.push(current_tick);
        Self {
            piece,
            positions: moves_per_tick,
        }
    }

    pub fn final_move(&self) -> Move {
        if self.positions.is_empty() {
            panic!("no positions in path");
        }
        let last_pos = self
            .positions
            .last()
            .and_then(|m| m.last())
            .copied()
            .unwrap();
        Move {
            piece: self.piece,
            pos: last_pos,
        }
    }

    pub fn into_moves(self) -> Vec<Vec<Position>> {
        self.positions
    }
}

#[cfg(feature = "wasm")]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Path {
    #[wasm_bindgen(getter)]
    pub fn length(&self) -> usize {
        self.positions.len()
    }

    pub fn transition_move(&self, index: usize, progress: f64) -> Move {
        let curr = &self.positions[index];
        let idx = ((progress * curr.len() as f64).floor() as usize).min(curr.len() - 1);
        Move {
            piece: self.piece,
            pos: curr[idx],
        }
    }
}

/// Returns all possible moves when dropping a piece.
pub fn move_drop(board: &Board, piece: Piece) -> Vec<Position> {
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
                moves.push(Position {
                    rot,
                    row: row as isize,
                    col: col as isize,
                });
            }
        }
    }
    moves
}

const MIN_MOVES: u64 = 5; // minimum number of moves to perform in total
const MAX_MOVES: u64 = 12; // maximum number of moves to perform in total
const MAX_MOVES_PER_TICK: u64 = 3; // maximum number of moves to perform per tick
// lower settings makes the AI more realistic

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Node {
    pos: Position,
    cost: u64,       // How many ticks it takes to reach this node
    moves: u64,      // How many moves have been made
    tick_moves: u64, // How many moves have been made in the current tick
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

fn next_positions(candidates: &mut [Position], pos: Position, piece: Piece) -> usize {
    candidates[0] = Position {
        rot: pos.rot,
        row: pos.row,
        col: pos.col - 1, // left
    };
    candidates[1] = Position {
        rot: pos.rot,
        row: pos.row,
        col: pos.col + 1, // right
    };
    candidates[2] = Position {
        rot: pos.rot,
        row: pos.row + 1, // down
        col: pos.col,
    };
    // Rotate
    if piece.num_rotations() > 1 {
        candidates[3] = Position {
            rot: (pos.rot + 1) % piece.num_rotations(),
            row: pos.row,
            col: pos.col,
        };
        if piece.num_rotations() > 2 {
            candidates[4] = Position {
                rot: pos.rot.wrapping_sub(1) % piece.num_rotations(),
                row: pos.row,
                col: pos.col,
            };
            5
        } else {
            4
        }
    } else {
        3
    }
}
fn touches_ground(piece: Piece, pos: Position, board: &Board) -> bool {
    board.overlaps_move(Move {
        piece,
        pos: Position {
            rot: pos.rot,
            row: pos.row + 1,
            col: pos.col,
        },
    })
}

pub fn move_dijkstra(board: &Board, piece: Piece, level: u64) -> Vec<Path> {
    let mut cost = HashMap::<Position, u64>::new();
    let mut parent = HashMap::<Position, Position>::new();
    let mut to_visit = BinaryHeap::new();
    let mut destinations = Vec::new();

    let speed: f64 = level as f64 / 30.0; // speed cap at level 30

    // linearly decrease with speed
    let max_moves = MIN_MOVES + ((1.0 - speed) * (MAX_MOVES - MIN_MOVES) as f64).round() as u64;
    let max_tick_moves = 1 + ((1.0 - speed) * (MAX_MOVES_PER_TICK - 1) as f64).round() as u64;

    let start_move = piece.into_start_move();
    if board.overlaps_move(start_move) {
        return Vec::new();
    }
    cost.insert(start_move.pos, 0);
    to_visit.push(Node {
        pos: start_move.pos,
        cost: 0,
        moves: 0,
        tick_moves: 0,
    });

    let mut candidates = [Position::default(); 5];

    while let Some(Node {
        pos: current,
        cost: current_cost,
        moves: current_moves,
        tick_moves: current_tick_moves,
    }) = to_visit.pop()
    {
        if touches_ground(piece, current, board) {
            destinations.push(current);
        }
        let next_count = next_positions(&mut candidates, current, piece);
        for next in candidates[..next_count].iter().copied() {
            if board.overlaps_move(Move { piece, pos: next }) {
                continue;
            }
            // staying on the same row, i.e. the same tick does not cost anything
            // but is limited by max_moves and max_tick_moves
            let (new_cost, new_moves, new_tick_moves) = if next.row == current.row {
                (current_cost, current_moves + 1, current_tick_moves + 1)
            } else {
                (current_cost + 1, current_moves, 0)
            };
            if new_moves > max_moves || new_tick_moves > max_tick_moves {
                continue;
            }
            if !cost.contains_key(&next) || new_cost < cost[&next] {
                cost.insert(next, new_cost);
                to_visit.push(Node {
                    pos: next,
                    cost: new_cost,
                    moves: new_moves,
                    tick_moves: new_tick_moves,
                });
                parent.insert(next, current);
            }
        }
    }
    destinations
        .into_iter()
        .map(|end| {
            let mut current = end;
            let mut path = Vec::new();
            while current != start_move.pos {
                path.push(current);
                current = parent[&current];
            }
            path.push(start_move.pos);
            Path::from_path_reverse(path, piece)
        })
        .collect()
}

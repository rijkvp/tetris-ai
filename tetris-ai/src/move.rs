use crate::board::{Board, BOARD_HEIGHT, BOARD_WIDTH};
use crate::piece::Piece;
use serde::Serialize;
use std::collections::{BinaryHeap, HashMap};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize)]
pub struct Move {
    pub rot: usize,
    pub row: isize,
    pub col: isize,
}

pub struct Path {
    moves: Vec<Vec<Move>>,
}

impl Path {
    fn from_path_reverse(mut reversed_path: Vec<Move>) -> Self {
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
            moves: moves_per_tick,
        }
    }

    pub fn final_move(&self) -> Move {
        if self.moves.is_empty() {
            panic!("no moves");
        }
        self.moves.last().and_then(|m| m.last()).copied().unwrap()
    }

    pub fn into_moves(self) -> Vec<Vec<Move>> {
        self.moves
    }
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

const MAX_MOVES_PER_TICK: u64 = 3; // maximum number of moves to perform per tick
                                   // this makes the AI more realistic

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Node {
    r#move: Move,
    cost: u64,
    tick_moves: u64,
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
    board.overlaps_move(
        piece,
        Move {
            rot: r#move.rot,
            row: r#move.row + 1,
            col: r#move.col,
        },
    )
}

pub fn move_dijkstra(board: Board, piece: Piece) -> Vec<Path> {
    let mut cost = HashMap::<Move, u64>::new();
    let mut parent = HashMap::<Move, Move>::new();
    let mut to_visit = BinaryHeap::new();
    let mut destinations = Vec::new();

    let start_move = spawn_position(piece);
    if board.overlaps_move(piece, start_move) {
        return Vec::new();
    }
    cost.insert(start_move, 0);
    to_visit.push(Node {
        r#move: start_move,
        cost: 0,
        tick_moves: 0,
    });

    while let Some(Node {
        r#move: current,
        cost: current_cost,
        tick_moves: current_tick_moves,
    }) = to_visit.pop()
    {
        if touches_ground(piece, current, &board) {
            destinations.push(current);
        }
        for next in next_moves(current, piece, &board).into_iter() {
            // staying on the same row, i.e. the same tick does not cost anything
            // but is limited by MAX_MOVES_PER_TICK
            let (new_cost, new_tick_moves) = if next.row == current.row {
                (current_cost, current_tick_moves + 1)
            } else {
                (current_cost + 1, 0)
            };
            if new_tick_moves >= MAX_MOVES_PER_TICK {
                continue;
            }
            if !cost.contains_key(&next) || new_cost < cost[&next] {
                cost.insert(next, new_cost);
                to_visit.push(Node {
                    r#move: next,
                    cost: new_cost,
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
            while current != start_move {
                path.push(current);
                current = parent[&current];
            }
            path.push(start_move);
            Path::from_path_reverse(path)
        })
        .collect()
}

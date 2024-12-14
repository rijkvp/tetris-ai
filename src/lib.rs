#![feature(gen_blocks)]

use board::Board;
use piece::Piece;
use state::{Move, State};

pub mod board;
pub mod feature;
pub mod r#move;
pub mod piece;
pub mod state;
#[cfg(test)]
pub mod test;

pub fn simulate<F, I>(
    mut state: State,
    piece_gen: impl Fn() -> Piece,
    move_gen: F,
    eval: impl Fn(&State) -> f64,
) -> (State, u64)
where
    F: Fn(Board, Piece) -> I,
    I: Iterator<Item = Move>,
{
    let mut piece;
    let mut steps = 0;
    loop {
        piece = piece_gen(); // generate a new piece

        let mut best = None;
        let mut best_score = f64::NEG_INFINITY;
        for r#move in move_gen(state.board, piece) {
            let future = state.future(piece, r#move);
            let score = eval(&future);
            if score > best_score {
                best = Some(future);
                best_score = score;
            }
        }

        if let Some(next) = best {
            state = next; // update state
        } else {
            break;
        }
        steps += 1;
    }
    (state, steps)
}

use crate::piece::{Piece, N_PIECES};
use rand::Rng;

// Based on how NES Tetris generates pieces
pub fn gen_random_piece(previous: Option<usize>) -> Piece {
    let first_roll = rand::thread_rng().gen_range(0..=N_PIECES);
    if first_roll == N_PIECES || previous == Some(first_roll) {
        // reroll if the first roll is the same as the previous piece
        // or if the 'reroll' number is hit
        return Piece::from_index(rand::thread_rng().gen_range(0..N_PIECES));
    }
    Piece::from_index(first_roll)
}

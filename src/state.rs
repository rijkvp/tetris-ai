use crate::{board::Board, piece::Piece};

#[derive(Clone, Copy, Debug)]
pub struct State {
    pub board: Board,
    pub delta: Option<Delta>,
}

#[derive(Clone, Copy, Debug)]
pub struct Delta {
    pub piece: Piece,
    pub r#move: Move,
}

#[derive(Clone, Copy, Debug)]
pub struct Move {
    pub rot: usize,
    pub row: usize,
    pub col: usize,
}

impl State {
    pub fn new(board: Board) -> Self {
        Self { board, delta: None }
    }

    /// Computes the new 'future' state after a piece has been moved.
    pub(crate) fn future(&self, piece: Piece, r#move: Move) -> Self {
        let mut board = self.board.clone();
        board.imprint(piece.get_rotation(r#move.rot), r#move.row, r#move.col);
        Self {
            board,
            delta: Some(Delta { piece, r#move }),
        }
    }
}

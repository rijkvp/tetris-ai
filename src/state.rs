use crate::{board::Board, piece::Piece};

#[derive(Clone)]
pub struct State {
    pub board: Board,
    pub delta: Option<Delta>,
}

#[derive(Clone, Debug)]
pub struct Delta {
    pub piece: Piece,
    pub r#move: Move,
    pub eroded: usize,
    #[cfg(test)]
    pub cleared: Vec<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
        let mut board = self.board;
        board.imprint(piece.get_rotation(r#move.rot), r#move.row, r#move.col);
        let cleared = board.clear_full();

        // count the number of cells that were cleared by the move.
        let mut eroded = 0;
        for row in &cleared {
            let pattern = piece.get_rotation(r#move.rot);
            if let Some(row) = pattern.get_row(row - r#move.row) {
                for cell in row {
                    if cell.filled() {
                        eroded += 1;
                    }
                }
            }
        }

        Self {
            board,
            delta: Some(Delta {
                piece,
                r#move,
                eroded,
                #[cfg(test)]
                cleared,
            }),
        }
    }
}

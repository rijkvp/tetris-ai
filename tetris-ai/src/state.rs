use crate::{board::Board, piece::Piece, r#move::Move};

#[derive(Clone)]
pub struct State {
    pub board: Board,
    pub cleared_rows: usize,
    pub score: usize,
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

const POINTS_PER_CLEARED_ROWS: [usize; 5] = [0, 40, 100, 300, 1200];

impl State {
    pub fn new(board: Board) -> Self {
        Self {
            board,
            cleared_rows: 0,
            delta: None,
            score: 0,
        }
    }

    pub fn level(&self) -> usize {
        (self.cleared_rows / 10) + 1
    }

    /// Computes the new 'future' state after a piece has been moved.
    pub(crate) fn future(&self, piece: Piece, r#move: Move) -> Self {
        let mut board = self.board;
        board.imprint(
            piece.rotation(r#move.rot),
            r#move.row,
            r#move.col,
            piece.cell(),
        );
        let cleared_rows = board.clear_full();
        let cleared = cleared_rows.len();

        // count the number of cells that were cleared by the move.
        let mut eroded = 0;
        for row in cleared_rows.iter().copied() {
            let pattern = piece.rotation(r#move.rot);
            if row as isize >= r#move.row && (row as isize) < r#move.row + pattern.cols() as isize {
                if let Some(row) = pattern.get_row(row - r#move.row as usize) {
                    for filled in row {
                        if *filled {
                            eroded += 1;
                        }
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
                cleared: cleared_rows,
            }),
            cleared_rows: self.cleared_rows + cleared,
            score: self.score + POINTS_PER_CLEARED_ROWS[cleared] * (self.level() + 1),
        }
    }
}

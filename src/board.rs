use crate::piece::Pattern;
use std::{fmt::Display, ops::Index};

/// Represents a cell on the board.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Cell(u8);

impl Cell {
    pub const fn new(inner: u8) -> Self {
        Self(inner)
    }

    /// Returns true if the cell is occupied.
    pub fn occupied(&self) -> bool {
        self.0 != 0
    }

    /// Returns true if the cell is empty.
    pub fn empty(&self) -> bool {
        self.0 == 0
    }

    /// Adds the other cell to this cell, replacing it if it is occupied.
    pub fn add(&mut self, other: Cell) {
        if other.occupied() {
            self.0 = other.0;
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.occupied() {
            write!(f, "#")
        } else {
            write!(f, ".")
        }
    }
}

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;

/// Represents a Tetris board.
#[derive(Copy, Clone, Debug, Default)]
pub struct Board {
    data: [[Cell; BOARD_WIDTH]; BOARD_HEIGHT],
    heights: [usize; BOARD_WIDTH],
}

impl Board {
    /// Returns the height of the given column.
    pub(crate) fn height(&self, col: usize) -> usize {
        return self.heights[col];
    }

    pub(crate) fn heights(&self) -> &[usize; BOARD_WIDTH] {
        &self.heights
    }

    /// Imprints the given matrix onto the board at the given row and column.
    pub(crate) fn imprint(&mut self, pattern: Pattern, row: usize, col: usize) {
        for (r, p_row) in pattern.iter_rows().enumerate() {
            for (c, cell) in p_row.iter().enumerate() {
                self.data[row + r][col + c].add(*cell);
                self.heights[col + c] = self.heights[col + c].max(BOARD_HEIGHT - (row + r));
            }
        }
    }

    /// Returns true if the pattern overlaps with the board.
    pub(crate) fn overlaps(&self, pattern: &Pattern, row: usize, col: usize) -> bool {
        if row + pattern.rows() > BOARD_HEIGHT || col + pattern.cols() > BOARD_WIDTH {
            return true; // out of bounds
        }
        for (r, p_row) in pattern.iter_rows().enumerate() {
            for (c, cell) in p_row.iter().enumerate() {
                if cell.occupied() && self.data[row + r][col + c].occupied() {
                    return true;
                }
            }
        }
        return false;
    }

    /// Clears the full rows and returns the number of rows cleared.
    pub(crate) fn clear_full(&mut self) -> Vec<usize> {
        let mut rows = Vec::new();
        let mut bottom = (BOARD_HEIGHT - 1) as i64;
        while bottom >= 0 {
            let mut top = bottom;
            while top >= 0 && self.data[top as usize].iter().all(|cell| cell.occupied()) {
                rows.push(top as usize);
                top -= 1;
            }
            if top == bottom {
                bottom -= 1;
                continue;
            }
            // shift rows down
            let shift = (bottom - top) as usize;
            if shift < BOARD_HEIGHT {
                for r in (0..=top.max(0) as usize).rev() {
                    for c in 0..BOARD_WIDTH {
                        self.data[r + shift][c] = self.data[r][c]; // shift down
                        self.data[r][c] = Cell::default(); // clear row
                    }
                }
            }
            // clear the rest of the rows
            for r in (top + 1) as usize..shift {
                for c in 0..BOARD_WIDTH {
                    self.data[r][c] = Cell::default(); // clear row
                }
            }
            bottom = top;
        }
        let rows_cleared = rows.len();
        if rows_cleared > 0 {
            // update heights accordingly
            for c in 0..BOARD_WIDTH {
                // Important: this assumes there are no empty cells below full rows
                // otherwise the heights calculation will be incorrect
                self.heights[c] -= rows_cleared;
            }
        }
        rows
    }

    #[cfg(test)]
    pub(crate) fn get_data(&self) -> &[[Cell; BOARD_WIDTH]; BOARD_HEIGHT] {
        &self.data
    }

    #[cfg(test)]
    pub(crate) fn fill_cell(&mut self, row: usize, col: usize) {
        self.data[row][col] = Cell::new(1);
        self.heights[col] = self.heights[col].max(BOARD_HEIGHT - row);
    }

    #[cfg(test)]
    pub(crate) fn clear_cell(&mut self, row: usize, col: usize) {
        self.data[row][col] = Cell::default();
        // if the top cell was cleared
        if self.heights[col] == BOARD_HEIGHT - row {
            // find the highest cell in the column
            for r in 0..BOARD_HEIGHT {
                if self.data[r][col].occupied() {
                    self.heights[col] = BOARD_HEIGHT - r;
                    return;
                }
            }
            self.heights[col] = 0; // reset to 0 if column is empty
        }
    }
}

impl Index<(usize, usize)> for Board {
    type Output = Cell;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row][col]
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for r in 0..BOARD_HEIGHT {
            for c in 0..BOARD_WIDTH {
                write!(f, "{}", self.data[r][c])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fill_row(board: &mut Board, row: usize) {
        for c in 0..BOARD_WIDTH {
            board.fill_cell(row, c);
        }
    }

    #[test]
    fn test_clear_board_single() {
        let mut board = Board::default();
        fill_row(&mut board, BOARD_HEIGHT - 1);
        assert_eq!(board.clear_full(), vec![BOARD_HEIGHT - 1]);
        assert_eq!(board.heights(), &[0; BOARD_WIDTH]);
    }

    #[test]
    fn test_clear_board_single_2() {
        let mut board = Board::default();
        fill_row(&mut board, 8);
        assert_eq!(board.clear_full(), vec![8]);
    }

    #[test]
    fn test_clear_board_single_3() {
        let mut board = Board::default();
        fill_row(&mut board, 0);
        assert_eq!(board.clear_full(), vec![0]);
    }

    #[test]
    fn test_clear_board_multiple() {
        let ranges = vec![0..3, 4..9, 5..14, 16..BOARD_HEIGHT, 0..BOARD_HEIGHT];
        for (n, range) in ranges.into_iter().enumerate() {
            let mut board = Board::default();
            for r in range.clone() {
                fill_row(&mut board, r);
            }
            assert_eq!(board.clear_full(), range.rev().collect::<Vec<_>>());
            if n >= 3 {
                // last three are valid Tetris boards
                assert_eq!(board.heights(), &[0; BOARD_WIDTH]);
            }
        }
    }

    #[test]
    fn test_clear_board_bottom_fill() {
        let mut board = Board::default();
        for i in 0..BOARD_HEIGHT - 1 {
            let range = i..BOARD_HEIGHT;
            for r in range.clone() {
                fill_row(&mut board, r);
            }
            assert_eq!(board.clear_full(), range.rev().collect::<Vec<_>>());
            for r in 0..BOARD_HEIGHT {
                for c in 0..BOARD_WIDTH {
                    assert_eq!(board[(r, c)].empty(), true);
                }
            }
            assert_eq!(board.heights(), &[0; BOARD_WIDTH]);
        }
    }
}

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
        let mut iter = pattern.iter_rows().enumerate();
        // first row also updates heights
        if let Some((r, p_row)) = iter.next() {
            for (c, cell) in p_row.iter().enumerate() {
                self.data[row + r][col + c].add(*cell);
                self.heights[col + c] = self.heights[col + c].max(BOARD_HEIGHT - (row + r));
            }
        }
        for (r, p_row) in iter {
            for (c, cell) in p_row.iter().enumerate() {
                self.data[row + r][col + c].add(*cell);
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
        for bottom in (1..=BOARD_HEIGHT).rev() {
            let mut top = bottom;
            while {
                top -= 1;
                self.data[top].iter().all(|cell| cell.occupied())
            } {
                rows.push(top);
            }
            // shift all rows above top down
            let shift = bottom - top - 1;
            if shift == 0 {
                continue;
            }
            // shift rows down
            for r in (0..=top).rev() {
                for c in 0..BOARD_WIDTH {
                    self.data[r + shift][c] = self.data[r][c]; // shift down
                    self.data[r][c] = Cell::default(); // clear row
                }
            }
        }
        let rows_cleared = rows.len();
        if rows_cleared > 0 {
            // update heights accordingly
            for c in 0..BOARD_WIDTH {
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

use crate::{
    r#move::Move,
    piece::{Pattern, Piece},
};
use std::{fmt::Display, ops::Index, str::FromStr};

/// Represents a cell on the board.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct Cell(u8);

impl Cell {
    pub const fn new(inner: u8) -> Self {
        Self(inner)
    }

    /// Returns true if the cell is occupied.
    pub fn filled(&self) -> bool {
        self.0 != 0
    }

    /// Returns true if the cell is empty.
    pub fn empty(&self) -> bool {
        self.0 == 0
    }

    /// Returns the inner value of the cell.
    pub fn inner(&self) -> u8 {
        self.0
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.filled() {
            write!(f, "#")
        } else {
            write!(f, ".")
        }
    }
}

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;

/// Represents a Tetris board.
#[derive(Copy, Clone, Default)]
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
    pub(crate) fn imprint(&mut self, pattern: Pattern, row: isize, col: isize, cell: Cell) {
        for (r_offset, p_row) in pattern.iter_rows().enumerate() {
            for (c_offset, filled) in p_row.into_iter().enumerate() {
                let r = row + r_offset as isize;
                let c = col + c_offset as isize;
                if *filled
                    && r >= 0
                    && r < BOARD_HEIGHT as isize
                    && c >= 0
                    && c < BOARD_WIDTH as isize
                {
                    let (r, c) = (r as usize, c as usize);
                    self.data[r][c] = cell;
                    self.heights[c] = self.heights[c].max(BOARD_HEIGHT - r);
                }
            }
        }
    }

    /// Returns true if the pattern overlaps with the board.
    // TODO: obsolete
    pub(crate) fn overlaps(&self, pattern: &Pattern, row: usize, col: usize) -> bool {
        if row + pattern.rows() > BOARD_HEIGHT || col + pattern.cols() > BOARD_WIDTH {
            return true; // out of bounds
        }
        for (r, p_row) in pattern.iter_rows().enumerate() {
            for (c, filled) in p_row.into_iter().enumerate() {
                if *filled && self.data[row + r][col + c].filled() {
                    return true;
                }
            }
        }
        return false;
    }

    /// Returns true if the move overlaps with the board.
    pub(crate) fn overlaps_move(&self, piece: Piece, r#move: Move) -> bool {
        let pattern = piece.rotation(r#move.rot);
        for (r, p_row) in pattern.iter_rows().enumerate() {
            for (c, filled) in p_row.into_iter().enumerate() {
                if !*filled {
                    continue;
                }
                let row = r as isize + r#move.row;
                let col = c as isize + r#move.col;
                if row >= BOARD_HEIGHT as isize || col < 0 || col >= BOARD_WIDTH as isize {
                    return true; // collision with the bottom or sides
                } else if row >= 0 // in bounds of board
                    && row < BOARD_HEIGHT as isize
                    && col >= 0
                    && col < BOARD_WIDTH as isize
                    && self.data[row as usize][col as usize].filled()
                {
                    return true; // collision with a filled cell
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
            while top >= 0
                && self.data[top as usize]
                    .into_iter()
                    .all(|cell| cell.filled())
            {
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
                for r in (0..(top + 1) as usize).rev() {
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
            'outer: for c in 0..BOARD_WIDTH {
                let max_height = self.heights[c] - rows_cleared;
                for r in (BOARD_HEIGHT - max_height)..BOARD_HEIGHT {
                    if self.data[r][c].filled() {
                        self.heights[c] = BOARD_HEIGHT - r;
                        continue 'outer;
                    }
                }
                self.heights[c] = 0;
            }
        }
        rows
    }

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
                if self.data[r][col].filled() {
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

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.trim().lines().collect::<Vec<_>>();
        assert!(lines.len() == BOARD_HEIGHT);
        let mut data = [[Cell::default(); BOARD_WIDTH]; BOARD_HEIGHT];
        let mut heights = [0; BOARD_WIDTH];
        for (r, line) in lines.iter().enumerate() {
            assert!(line.len() == BOARD_WIDTH);
            for (c, ch) in line.chars().enumerate() {
                if ch == '#' {
                    data[r][c] = Cell::new(1);
                    heights[c] = heights[c].max(BOARD_HEIGHT - r);
                } else if ch == '.' {
                    data[r][c] = Cell::default();
                } else {
                    panic!("invalid character: {}", ch);
                }
            }
        }
        Ok(Board { data, heights })
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

    #[test]
    fn test_clear_board_regression() {
        let mut board = Board::from_str(
            r#"
##########
#########.
#.#..###..
..#..###..
.##...#...
.##...#...
..###.#...
..##..#...
..##..#...
#.##.##...
#.##.##...
###..#.#..
###.##.#..
########..
..######..
.#.#.####.
.#.#...###
##.#..####
##.#...###
##.#....##
    "#,
        )
        .unwrap();
        let cleared = board.clear_full();
        assert_eq!(cleared, vec![0]);
        let expected = Board::from_str(
            r#"
..........
#########.
#.#..###..
..#..###..
.##...#...
.##...#...
..###.#...
..##..#...
..##..#...
#.##.##...
#.##.##...
###..#.#..
###.##.#..
########..
..######..
.#.#.####.
.#.#...###
##.#..####
##.#...###
##.#....##
            "#,
        )
        .unwrap();
        println!("Expected board:{}", expected);
        println!("Actual board:{}", board);
        assert_eq!(board.heights, expected.heights);
        assert_eq!(board.data, expected.data);
    }
}

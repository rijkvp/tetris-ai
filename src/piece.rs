use crate::board::Cell;

type PatternRef<'a> = &'a [&'a [Cell]];

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for row in self.iter_rows() {
            for cell in row {
                write!(f, "{cell}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

/// A rotatable tetromino piece.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece(usize);

impl Piece {
    pub fn from_index(index: usize) -> Self {
        assert!(index < PIECE_DATA.len());
        Piece(index)
    }

    pub fn get_index(&self) -> usize {
        self.0
    }

    pub fn get_rotation(&self, rotation: usize) -> Pattern {
        debug_assert!(rotation < self.num_rotations());
        rot90(PIECE_DATA[self.0].pattern, rotation)
    }

    pub fn num_rotations(&self) -> usize {
        PIECE_DATA[self.0].rotations
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Piece {}", PIECE_DATA[self.0].name)
    }
}

/// Data for each piece.
pub struct PieceData {
    name: char,
    pattern: PatternRef<'static>,
    rotations: usize,
}

pub const N_PIECES: usize = 7;
const PIECE_DATA: [PieceData; N_PIECES] = [
    PieceData {
        name: 'I',
        pattern: &[&[Cell::new(1), Cell::new(1), Cell::new(1), Cell::new(1)]],
        rotations: 2,
    },
    PieceData {
        name: 'T',
        pattern: &[
            &[Cell::new(2), Cell::new(2), Cell::new(2)],
            &[Cell::new(0), Cell::new(2), Cell::new(0)],
        ],
        rotations: 4,
    },
    PieceData {
        name: 'L',
        pattern: &[
            &[Cell::new(3), Cell::new(3), Cell::new(3)],
            &[Cell::new(3), Cell::new(0), Cell::new(0)],
        ],
        rotations: 4,
    },
    PieceData {
        name: 'J',
        pattern: &[
            &[Cell::new(4), Cell::new(4), Cell::new(4)],
            &[Cell::new(0), Cell::new(0), Cell::new(4)],
        ],
        rotations: 4,
    },
    PieceData {
        name: 'O',
        pattern: &[&[Cell::new(5), Cell::new(5)], &[Cell::new(5), Cell::new(5)]],
        rotations: 1,
    },
    PieceData {
        name: 'Z',
        pattern: &[
            &[Cell::new(6), Cell::new(6), Cell::new(0)],
            &[Cell::new(0), Cell::new(6), Cell::new(6)],
        ],
        rotations: 2,
    },
    PieceData {
        name: 'S',
        pattern: &[
            &[Cell::new(0), Cell::new(7), Cell::new(7)],
            &[Cell::new(7), Cell::new(7), Cell::new(0)],
        ],
        rotations: 2,
    },
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pattern {
    data: PatternRef<'static>,
    rev_rows: bool,
    rev_cols: bool,
}

impl Pattern {
    fn new(data: PatternRef<'static>, rev_rows: bool, rev_cols: bool) -> Self {
        Self {
            data,
            rev_rows,
            rev_cols,
        }
    }

    pub fn iter_rows(&self) -> PatternRowIter {
        PatternRowIter::new(
            self.data,
            self.rev_rows,
            self.rev_cols,
            self.rev_cols != self.rev_rows,
        )
    }

    pub fn get_row(&self, row: usize) -> Option<PatternColIter> {
        self.iter_rows().nth(row)
    }

    pub fn rows(&self) -> usize {
        if self.rev_cols == self.rev_rows {
            self.data.len()
        } else {
            self.data[0].len()
        }
    }

    pub fn cols(&self) -> usize {
        if self.rev_cols == self.rev_rows {
            self.data[0].len()
        } else {
            self.data.len()
        }
    }
}

pub struct PatternRowIter {
    data: PatternRef<'static>,
    row: isize,
    rev_rows: bool,
    rev_cols: bool,
    transpose: bool,
}

impl PatternRowIter {
    fn new(data: PatternRef<'static>, rev_rows: bool, rev_cols: bool, transpose: bool) -> Self {
        Self {
            data,
            row: if rev_rows {
                (if transpose { data[0].len() } else { data.len() }) as isize - 1
            } else {
                0
            },
            rev_rows,
            rev_cols,
            transpose,
        }
    }
}

impl Iterator for PatternRowIter {
    type Item = PatternColIter;

    fn next(&mut self) -> Option<Self::Item> {
        let length = if self.transpose {
            self.data[0].len()
        } else {
            self.data.len()
        };
        if self.row >= 0 && (self.row as usize) < length {
            let prev = self.row as usize;
            self.row = if self.rev_rows {
                self.row - 1
            } else {
                self.row + 1
            };
            Some(PatternColIter::new(
                self.data,
                prev,
                self.rev_cols,
                self.transpose,
            ))
        } else {
            None
        }
    }
}

pub struct PatternColIter {
    data: PatternRef<'static>,
    row: usize,
    col: isize,
    rev_cols: bool,
    transpose: bool,
}

impl PatternColIter {
    fn new(data: PatternRef<'static>, row: usize, rev_cols: bool, transpose: bool) -> Self {
        Self {
            data,
            row,
            col: if rev_cols {
                (if transpose { data.len() } else { data[0].len() }) as isize - 1
            } else {
                0
            },
            rev_cols,
            transpose,
        }
    }
}

impl Iterator for PatternColIter {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        let length = if self.transpose {
            self.data.len()
        } else {
            self.data[0].len()
        };
        if self.col >= 0 && (self.col as usize) < length {
            let prev = self.col as usize;
            self.col = if self.rev_cols {
                self.col - 1
            } else {
                self.col + 1
            };
            if self.transpose {
                Some(self.data[prev][self.row]) // for 1 and 3
            } else {
                Some(self.data[self.row][prev]) // for 0 and 2
            }
        } else {
            None
        }
    }
}

fn rot90(pattern: PatternRef<'static>, times: usize) -> Pattern {
    debug_assert!(times < 4); // 4 rotations is the same as 0
    match times {
        0 => Pattern::new(pattern, false, false),
        1 => Pattern::new(pattern, false, true),
        2 => Pattern::new(pattern, true, true),
        3 => Pattern::new(pattern, true, false),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PATTERN: PatternRef<'static> = &[
        &[Cell::new(1), Cell::new(2), Cell::new(3)],
        &[Cell::new(4), Cell::new(5), Cell::new(6)],
        &[Cell::new(7), Cell::new(8), Cell::new(9)],
    ];

    #[test]
    fn test_rot90() {
        fn rot90vec(times: usize) -> Vec<Vec<Cell>> {
            rot90(TEST_PATTERN, times)
                .iter_rows()
                .map(|r| r.collect())
                .collect()
        }
        assert_eq!(
            rot90vec(0),
            vec![
                vec![Cell::new(1), Cell::new(2), Cell::new(3)],
                vec![Cell::new(4), Cell::new(5), Cell::new(6)],
                vec![Cell::new(7), Cell::new(8), Cell::new(9)]
            ]
        );
        assert_eq!(
            rot90vec(1),
            vec![
                vec![Cell::new(7), Cell::new(4), Cell::new(1)],
                vec![Cell::new(8), Cell::new(5), Cell::new(2)],
                vec![Cell::new(9), Cell::new(6), Cell::new(3)]
            ]
        );
        assert_eq!(
            rot90vec(2),
            vec![
                vec![Cell::new(9), Cell::new(8), Cell::new(7)],
                vec![Cell::new(6), Cell::new(5), Cell::new(4)],
                vec![Cell::new(3), Cell::new(2), Cell::new(1)]
            ]
        );
        assert_eq!(
            rot90vec(3),
            vec![
                vec![Cell::new(3), Cell::new(6), Cell::new(9)],
                vec![Cell::new(2), Cell::new(5), Cell::new(8)],
                vec![Cell::new(1), Cell::new(4), Cell::new(7)]
            ]
        );
    }
}

use crate::board::Cell;

type PatternRef<'a> = &'a [&'a [Cell]];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pattern(Vec<Vec<Cell>>);

impl Pattern {
    pub fn iter_rows(&self) -> std::slice::Iter<'_, Vec<Cell>> {
        self.0.iter()
    }

    pub fn get_row(&self, row: usize) -> Option<&[Cell]> {
        self.0.get(row).map(|r| r.as_slice())
    }

    pub fn rows(&self) -> usize {
        self.0.len()
    }

    pub fn cols(&self) -> usize {
        self.0[0].len()
    }
}

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for row in self.iter_rows() {
            for cell in row.iter() {
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

    pub fn rotations(&self) -> Box<impl Iterator<Item = Pattern>> {
        let pattern = PIECE_DATA[self.0].pattern;
        let rots = PIECE_DATA[self.0].rotations;
        Box::new((0..rots).map(|i| rot90(pattern, i)))
    }

    pub fn get_rotation(&self, rotation: usize) -> Pattern {
        rot90(PIECE_DATA[self.0].pattern, rotation)
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

fn rot90(pattern: PatternRef<'_>, times: usize) -> Pattern {
    debug_assert!(times < 4); // 4 rotations is the same as 0

    if times == 0 || times == 2 {
        let mut result = Vec::with_capacity(pattern.len());
        if times == 0 {
            // no rotation
            for row in pattern.iter() {
                result.push(row.iter().copied().collect());
            }
        } else {
            // 180 degree rotation, reverse both rows and columns
            for row in pattern.iter().rev() {
                result.push(row.iter().rev().copied().collect());
            }
        }
        Pattern(result)
    } else {
        let mut result = Vec::with_capacity(pattern[0].len());
        let rows = pattern.len();
        let cols = pattern[0].len();
        if times == 1 {
            // 90 degree rotation
            for c in 0..cols {
                let mut new_row = Vec::with_capacity(rows);
                for r in (0..rows).rev() {
                    new_row.push(pattern[r][c]);
                }
                result.push(new_row);
            }
        } else {
            // 270 degree rotation
            for c in (0..cols).rev() {
                let mut new_row = Vec::with_capacity(rows);
                for r in 0..rows {
                    new_row.push(pattern[r][c]);
                }
                result.push(new_row);
            }
        }
        Pattern(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rot90() {
        let pattern: &[&[Cell]] = &[
            &[Cell::new(1), Cell::new(2), Cell::new(3)],
            &[Cell::new(4), Cell::new(5), Cell::new(6)],
            &[Cell::new(7), Cell::new(8), Cell::new(9)],
        ];
        assert_eq!(
            rot90(pattern, 0),
            Pattern(vec![
                vec![Cell::new(1), Cell::new(2), Cell::new(3)],
                vec![Cell::new(4), Cell::new(5), Cell::new(6)],
                vec![Cell::new(7), Cell::new(8), Cell::new(9)]
            ])
        );
        assert_eq!(
            rot90(pattern, 1),
            Pattern(vec![
                vec![Cell::new(7), Cell::new(4), Cell::new(1)],
                vec![Cell::new(8), Cell::new(5), Cell::new(2)],
                vec![Cell::new(9), Cell::new(6), Cell::new(3)]
            ])
        );
        assert_eq!(
            rot90(pattern, 2),
            Pattern(vec![
                vec![Cell::new(9), Cell::new(8), Cell::new(7)],
                vec![Cell::new(6), Cell::new(5), Cell::new(4)],
                vec![Cell::new(3), Cell::new(2), Cell::new(1)]
            ])
        );
        assert_eq!(
            rot90(pattern, 3),
            Pattern(vec![
                vec![Cell::new(3), Cell::new(6), Cell::new(9)],
                vec![Cell::new(2), Cell::new(5), Cell::new(8)],
                vec![Cell::new(1), Cell::new(4), Cell::new(7)]
            ])
        );
    }
}

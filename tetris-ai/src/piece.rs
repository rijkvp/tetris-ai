use crate::{
    board::{Cell, BOARD_WIDTH},
    r#move::{Move, Position},
};
use serde::Serialize;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

/// A rotatable tetromino piece.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct Piece(usize);

impl Piece {
    pub fn from_index(index: usize) -> Self {
        assert!(index < N_PIECES);
        Piece(index)
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.0
    }

    #[inline]
    pub fn rotation(&self, rotation: usize) -> Pattern {
        debug_assert!(rotation < self.num_rotations());
        Pattern(PIECE_DATA[self.0].patterns[rotation])
    }

    #[inline]
    pub fn num_rotations(&self) -> usize {
        PIECE_DATA[self.0].patterns.len()
    }

    #[inline]
    pub fn cell(&self) -> crate::board::Cell {
        Cell::new(self.0 as u8 + 1)
    }

    pub fn spawn_offset(&self) -> (isize, isize) {
        PIECE_DATA[self.0].spawn_offset
    }

    pub fn into_start_move(self) -> Move {
        let offset = self.spawn_offset();
        Move {
            piece: self,
            pos: Position {
                rot: 0,
                row: 0 - offset.0,
                col: (BOARD_WIDTH / 2) as isize - offset.1,
            },
        }
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Piece {
    #[cfg(feature = "wasm")]
    pub fn get_index(&self) -> usize {
        self.0
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Piece {}", PIECE_DATA[self.0].name)
    }
}

type Patterns<'a> = &'a [&'a [&'a [bool]]];

/// Data for each piece.
pub struct PieceData {
    name: char,
    patterns: Patterns<'static>,
    spawn_offset: (isize, isize),
}

// https://cdn.harddrop.com/0/07/NESTetris-pieces.png
pub const N_PIECES: usize = 7;

#[rustfmt::skip]
const PIECE_DATA: [PieceData; N_PIECES] = [
    PieceData {
        name: 'I',
        patterns: &[
            &[
                &[false, false, false, false],
                &[false, false, false, false],
                &[true,  true,  true,  true],
                &[false, false, false, false],
            ],
            &[
                &[false, false, true, false],
                &[false, false, true, false],
                &[false, false, true, false],
                &[false, false, true, false],
            ],
        ],
        spawn_offset: (2, 2),
    },
    PieceData {
        name: 'T',
        patterns: &[
            &[
                &[false, false, false],
                &[true, true, true],
                &[false, true, false],
            ],
            &[
                &[false, true, false],
                &[true, true, false],
                &[false, true, false],
            ],
            &[
                &[false, true, false],
                &[true, true, true],
                &[false, false, false],
            ],
            &[
                &[false, true, false],
                &[false, true, true],
                &[false, true, false],
            ],
        ],
        spawn_offset: (1, 1),
    },
    PieceData {
        name: 'L',
        patterns: &[
            &[
                &[false, false, false],
                &[true, true, true],
                &[true, false, false],
            ],
            &[
                &[true, true, false],
                &[false, true, false],
                &[false, true, false],
            ],
            &[
                &[false, false, true],
                &[true, true, true],
                &[false, false, false],
            ],
            &[
                &[false, true, false],
                &[false, true, false],
                &[false, true, true],
            ],
        ],
        spawn_offset: (1, 1),
    },
    PieceData {
        name: 'J',
        patterns: &[
            &[
                &[false, false, false],
                &[true, true, true],
                &[false, false, true],
            ],
            &[
                &[false, true, false],
                &[false, true, false],
                &[true, true, false],
            ],
            &[
                &[true, false, false],
                &[true, true, true],
                &[false, false, false],
            ],
            &[
                &[false, true, true],
                &[false, true, false],
                &[false, true, false],
            ],
        ],
        spawn_offset: (1, 1),
    },
    PieceData {
        name: 'O',
        patterns: &[
            &[
                &[true, true],
                &[true, true],
            ],
        ],
        spawn_offset: (0, 1),
    },
    PieceData {
        name: 'Z',
        patterns: &[
            &[
                &[false, false, false],
                &[true, true, false],
                &[false, true, true],
            ],
            &[
                &[false, true, false],
                &[true, true, false],
                &[true, false, false],
            ],
        ],
        spawn_offset: (1, 1),
    },
    PieceData {
        name: 'S',
        patterns: &[
            &[
                &[false, false, false],
                &[false, true, true],
                &[true, true, false],
            ],
            &[
                &[true, false, false],
                &[true, true, false],
                &[false, true, false],
            ],
        ],
        spawn_offset: (1, 1),
    },
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pattern(&'static [&'static [bool]]);

#[cfg_attr(feature = "wasm", wasm_bindgen(getter_with_clone))]
pub struct WasmPattern {
    pub data: Vec<u8>,
    pub size: usize,
}

impl Pattern {
    pub fn iter_rows(&self) -> impl Iterator<Item = &[bool]> {
        self.0.iter().copied()
    }

    pub fn get_row(&self, row: usize) -> Option<&[bool]> {
        self.0.get(row).map(|v| &**v)
    }

    pub fn rows(&self) -> usize {
        self.0.len()
    }

    pub fn cols(&self) -> usize {
        self.0[0].len()
    }

    #[cfg(feature = "wasm")]
    pub fn into_wasm(self) -> WasmPattern {
        let mut data = Vec::with_capacity(self.rows() * self.cols());
        for row in self.iter_rows() {
            for cell in row {
                data.push(if *cell { 1 } else { 0 });
            }
        }

        WasmPattern {
            data,
            size: self.rows(),
        }
    }
}

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for row in self.iter_rows() {
            for cell in row {
                write!(f, "{}", if *cell { 'x' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

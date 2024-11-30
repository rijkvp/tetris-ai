use board::{BOARD_HEIGHT, BOARD_WIDTH};
use piece::Piece;
use state::{Move, State};

pub mod board;
pub mod feature;
pub mod piece;
pub mod state;

/// Returns all possible moves when dropping a piece.
pub fn move_drop(state: &State, piece: Piece) -> Vec<Move> {
    let board = &state.board;
    let mut moves = Vec::new();
    for (rot, pattern) in piece.rotations().enumerate() {
        for col in 0..=(BOARD_WIDTH - pattern.cols()) {
            let highest = (0..pattern.cols())
                .map(|c| board.height(col + c))
                .max()
                .unwrap();
            let mut row = BOARD_HEIGHT - highest - pattern.rows();
            if !board.overlaps(&pattern, row, col) {
                while !board.overlaps(&pattern, row + 1, col) {
                    row += 1;
                }
                moves.push(Move { rot, row, col });
            }
        }
    }
    moves
}

pub fn simulate(
    mut state: State,
    piece_gen: impl Fn() -> Piece,
    move_gen: impl Fn(&State, Piece) -> Vec<Move>,
    move_policy: impl Fn(Vec<State>) -> Option<State>,
) {
    let mut piece;
    let mut i = 0;
    loop {
        println!("Iteration {i}");
        println!("{}", state.board);
        piece = piece_gen(); // generate a new piece

        // TODO: every state is a copy in this pool, optimize it
        let pool = move_gen(&state, piece)
            .into_iter()
            .map(|r#move| state.future(piece, r#move))
            .collect::<Vec<_>>();

        println!("Pool size: {}", pool.len());

        if let Some(next) = move_policy(pool) {
            state = next; // update state
        } else {
            // no possible moves, game over
            break;
        }
        i += 1;
    }
}

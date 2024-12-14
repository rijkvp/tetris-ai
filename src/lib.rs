use piece::Piece;
use state::{Move, State};

pub mod board;
pub mod feature;
pub mod r#move;
pub mod piece;
pub mod state;
#[cfg(test)]
pub mod test;

pub fn simulate(
    mut state: State,
    piece_gen: impl Fn() -> Piece,
    move_gen: impl Fn(&State, Piece) -> Vec<Move>,
    move_policy: impl Fn(Vec<State>) -> Option<State>,
) {
    let mut piece;
    // let start = Instant::now();
    loop {
        // println!("\n\nIteration {i}");
        piece = piece_gen(); // generate a new piece
                             // println!("Piece: {}", piece);
                             // println!("Board: {}", state.board);

        // TODO: every state is a copy in this pool, optimize it
        let pool = move_gen(&state, piece)
            .into_iter()
            .map(|r#move| state.future(piece, r#move))
            .collect::<Vec<_>>();

        // println!("Pool size: {}", pool.len());

        if let Some(next) = move_policy(pool) {
            state = next; // update state
        } else {
            // no possible moves, game over
            break;
        }
        // i += 1;
    }
    // let elapsed = start.elapsed();
    // println!(
    //     "moves: {}, elapsed: {:.2}s, moves/sec: {:.2}",
    //     i,
    //     elapsed.as_secs_f64(),
    //     i as f64 / elapsed.as_secs_f64()
    // );
    // println!("Final board: {}", state.board);
}

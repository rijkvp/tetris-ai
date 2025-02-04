use std::time::Instant;
use tetris_ai::Simulator;

fn main() {
    let start = Instant::now();
    let mut simulator = Simulator::new();
    while simulator.step().is_some() {}
    let elapsed = start.elapsed();
    println!(
        "moves: {}, cleared_rows: {}, score: {}, level: {}, elapsed: {:.2}s, moves/sec: {:.0}",
        simulator.steps,
        simulator.state.cleared_rows,
        simulator.state.score,
        simulator.state.level(),
        elapsed.as_secs_f64(),
        simulator.steps as f64 / elapsed.as_secs_f64()
    );
    println!("{}", simulator.state.board);
}

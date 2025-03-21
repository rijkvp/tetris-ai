use std::time::Instant;
use tetris_ai::simulator::Simulator;

fn main() {
    let start = Instant::now();
    let mut simulator = Simulator::new_with_preset("score");
    simulator.run();
    let elapsed = start.elapsed();
    let stats = simulator.stats();
    println!(
        "moves: {}, cleared_rows: {}, score: {}, level: {}, elapsed: {:.2}s, moves/sec: {:.0}",
        stats.steps,
        stats.lines,
        stats.score,
        stats.level,
        elapsed.as_secs_f64(),
        stats.steps as f64 / elapsed.as_secs_f64()
    );
    println!("{}", simulator.board());
}

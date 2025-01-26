use std::time::Instant;
use tetris_ai::Simulator;

fn main() {
    let start = Instant::now();
    let mut simulator = Simulator::new();
    let mut steps = 0;
    while simulator.step().is_some() && steps < 10000 {
        steps += 1;
    }
    let elapsed = start.elapsed();
    println!(
        "moves: {}, elapsed: {:?}, moves/sec: {:.2}",
        simulator.steps,
        elapsed,
        simulator.steps as f64 / elapsed.as_secs_f64()
    );
    println!("Final board: {}", simulator.state.board);
}

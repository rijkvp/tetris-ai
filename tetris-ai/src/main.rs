use std::time::Instant;
use tetris_ai::{learn::Learner, simulator::Simulator};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() > 1 {
        match args[1].as_str() {
            "learn" => learn(),
            "run" => run(),
            _ => eprintln!("unknown command: {}", args[1]),
        }
    } else {
        run();
    }
}

fn run() {
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

fn learn() {
    let mut learner = Learner::default();
    for i in 0..20 {
        learner.step();
        println!("Iteration {}:", i + 1);
        println!("---------------------");
        let (weights_map, st_dev) = learner.state();
        for (i, (feature, weight)) in weights_map.iter().enumerate() {
            println!("{:<20}\t{:+.4} \t(±{:.4})", feature, weight, st_dev[i]);
        }
    }
}

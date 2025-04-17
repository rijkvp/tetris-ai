use std::time::Instant;
use tetris_ai::{feature::Features, simulator::Simulator, train::Trainer};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() > 1 {
        match args[1].as_str() {
            "train" => train(),
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

fn train() {
    let mut trainer = Trainer::new(Features::from_names(&[
        "row_trans",
        "col_trans",
        "pits",
        "landing_height",
        "eroded_cells",
        "cuml_wells",
    ]));
    let mut i = 1;
    while !trainer.is_stable() {
        let state = trainer.train_gen();
        println!("\ngeneration {i}");
        println!(
            "max: {:.1}, min: {:.1}, mean: {:.1}",
            state.max, state.min, state.mean
        );
        let max_weight = state
            .weights
            .iter()
            .map(|(_, weight)| weight.abs())
            .fold(f64::NEG_INFINITY, f64::max);
        for (i, (feature, weight)) in state.weights.iter().enumerate() {
            let normalized_weight = weight / max_weight * 10.0;
            let normalized_st_dev = state.std_dev[i] / max_weight * 10.0;
            println!(
                "{:<20}\t{:+.1} \t(±{:.1})",
                feature, normalized_weight, normalized_st_dev
            );
        }
        i += 1;
    }
}

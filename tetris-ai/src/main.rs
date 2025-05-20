use std::time::Instant;
use tetris_ai::{
    feature::Features,
    simulator::Simulator,
    train::{TrainCriterion, Trainer},
};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() > 2 {
        let arg2 = args[2].as_str();
        match args[1].as_str() {
            "run" => run(arg2),
            "train" => train(arg2),
            _ => eprintln!("Unknown command: {}", args[1]),
        }
    } else {
        eprintln!("Usage: {} run|train [preset|criterion]", args[0]);
    }
}

fn run(preset: &str) {
    let start = Instant::now();
    let mut simulator = Simulator::new_with_preset(preset);
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

fn train(criterion: &str) {
    // The features to train on
    const FEATURE_NAMES: &[&str] = &[
        "col_trans",
        "row_trans",
        "pits",
        "landing_height",
        "eroded_cells",
        "cuml_wells",
    ];
    let Ok(criterion) = criterion.parse::<TrainCriterion>() else {
        eprintln!("Unknown criterion: '{}'", criterion);
        return;
    };

    let mut trainer = Trainer::new(Features::from_names(FEATURE_NAMES), criterion);
    while !trainer.is_stable() {
        let state = trainer.step();
        println!(
            "Generation {}, Model {}, Score: {:.1}",
            state.gen_index(),
            state.model_index(),
            state.eval_result().score
        );
        if let Some(generation) = state.generation() {
            println!(
                "max: {:.1}, min: {:.1}, mean: {:.1}",
                generation.max, generation.min, generation.mean
            );
            for (i, (weight, feature)) in generation
                .weights
                .iter()
                .zip(FEATURE_NAMES.iter())
                .enumerate()
            {
                println!(
                    "{:<20}\t{:+.1} \t(±{:.1})",
                    feature, weight, generation.std_dev[i]
                );
            }
        }
    }
}

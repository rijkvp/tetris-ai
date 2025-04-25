use crate::{
    feature::{Features, Weights, WeightsMap},
    simulator::Simulator,
};
use rand_distr::{Distribution, Normal};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

const EVAL_ITERATIONS: usize = 100000;
const STABLE_THRESHOLD: f64 = 0.1;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Trainer {
    features: Features,
    weights: Vec<f64>,
    st_dev: Vec<f64>,
    models_per_gen: usize,
    kept_per_gen: usize,
    criteria: TrainCriteria,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct TrainGeneration {
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter_with_clone))]
    pub weights: WeightsMap,
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter_with_clone))]
    pub std_dev: Vec<f64>,
    pub max: f64,
    pub min: f64,
    pub mean: f64,
}

impl Trainer {
    pub fn new(features: Features, criteria: TrainCriteria) -> Self {
        Self {
            weights: vec![0.0; features.len()],
            st_dev: vec![10.0; features.len()],
            features,
            models_per_gen: 100,
            kept_per_gen: 10,
            criteria,
        }
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Trainer {
    #[cfg(feature = "wasm")]
    pub fn from_feature_names(feature_names: Box<[String]>) -> Self {
        let strs = feature_names.iter().map(String::as_str).collect::<Vec<_>>();
        Self::new(Features::from_names(&strs), TrainCriteria::Score)
    }

    pub fn train_gen(&mut self) -> TrainGeneration {
        let mut rng = rand::rng();

        let generation: Vec<Vec<f64>> = (0..self.models_per_gen)
            .map(|_| {
                self.weights
                    .iter()
                    .zip(self.st_dev.iter())
                    .map(|(value, std_dev)| {
                        let dist = Normal::new(*value, *std_dev).unwrap();
                        dist.sample(&mut rng)
                    })
                    .collect()
            })
            .collect();

        // TODO: figure out if multithreading can be used here
        let mut results = generation
            .iter()
            .map(|weights| {
                (
                    weights,
                    self.criteria.eval(self.features.with_weights(weights)),
                )
            })
            .collect::<Vec<_>>();

        // Sort the results by score
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let max = results.first().unwrap().1;
        let min = results.last().unwrap().1;
        let mean = results.iter().map(|(_, x)| *x).sum::<f64>() / results.len() as f64;

        // Get the weights of the top N results
        let mut top_weights: Vec<Vec<f64>> =
            vec![Vec::with_capacity(self.kept_per_gen); self.weights.len()];
        for (weights, _) in results.into_iter().take(self.kept_per_gen) {
            for (i, value) in weights.iter().enumerate() {
                top_weights[i].push(*value);
            }
        }

        // Set the weights to the mean and standard deviation of the top N
        for (i, data) in top_weights.into_iter().enumerate() {
            let mean = data.iter().sum::<f64>() / (data.len() as f64);
            let var = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (data.len() as f64);
            self.weights[i] = mean;
            self.st_dev[i] = var.sqrt();
        }
        TrainGeneration {
            weights: self.features.with_weights(&self.weights).into(),
            std_dev: self.st_dev.clone(),
            max,
            min,
            mean,
        }
    }

    pub fn is_stable(&self) -> bool {
        self.st_dev.iter().all(|&x| x < STABLE_THRESHOLD)
    }
}

pub enum TrainCriteria {
    Score,
    Level,
    Tetrises,
}

impl TrainCriteria {
    fn eval(&self, weights: Weights) -> f64 {
        let mut sim = Simulator::new_with_weights(weights);
        match self {
            TrainCriteria::Score => {
                sim.run_for(EVAL_ITERATIONS);
                sim.stats().score as f64
            }
            TrainCriteria::Level => {
                while sim.step() {
                    if sim.stats().level >= 30 {
                        break;
                    }
                }
                sim.stats().lines as f64
            }
            TrainCriteria::Tetrises => {
                sim.run_for(EVAL_ITERATIONS);
                sim.stats().score as f64 * 10.0 * (sim.stats().tetrises + 1) as f64
            }
        }
    }
}

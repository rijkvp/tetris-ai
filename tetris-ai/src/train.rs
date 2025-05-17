use std::str::FromStr;

use crate::{
    feature::{Features, Weights},
    simulator::Simulator,
};
use rand_distr::{Distribution, Normal};
use serde::Serialize;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

const MODELS_PER_GEN: usize = 100;
const KEPT_PER_GEN: usize = 10;
const EVAL_ITERATIONS: usize = 10000;
const WEIGHT_RANGE: f64 = 10.0;
const STABLE_THRESHOLD: f64 = 0.25;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Trainer {
    features: Features,
    weights: Vec<f64>,
    st_dev: Vec<f64>,
    criteria: TrainCriteria,
    // state
    current_gen: Option<Vec<Vec<f64>>>,
    current_gen_index: usize,
    current_results: Vec<EvalResult>,
    current_model_index: usize,
}

impl Trainer {
    pub fn new(features: Features, criteria: TrainCriteria) -> Self {
        Self {
            weights: vec![0.0; features.len()],
            st_dev: vec![WEIGHT_RANGE; features.len()],
            features,
            criteria,
            current_gen: None,
            current_gen_index: 1,
            current_results: Vec::with_capacity(MODELS_PER_GEN),
            current_model_index: 0,
        }
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl TrainState {
    pub fn into_js(self) -> JsValue {
        serde_wasm_bindgen::to_value(&self).unwrap()
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Trainer {
    #[cfg(feature = "wasm")]
    pub fn from_feature_names(feature_names: Box<[String]>, criteria: String) -> Self {
        let strs = feature_names.iter().map(String::as_str).collect::<Vec<_>>();
        Self::new(Features::from_names(&strs), criteria.parse().unwrap())
    }

    pub fn reset(&mut self) {
        self.weights = vec![0.0; self.features.len()];
        self.st_dev = vec![WEIGHT_RANGE; self.features.len()];
        self.current_gen = None;
        self.current_gen_index = 1;
        self.current_results.clear();
        self.current_model_index = 0;
    }

    pub fn step(&mut self) -> TrainState {
        let mut rng = rand::rng();
        let generation = self.current_gen.get_or_insert_with(|| {
            // New generation
            (0..MODELS_PER_GEN)
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
                .collect()
        });
        // Eval a single model inside the current generation
        let weights = generation[self.current_model_index].clone();
        let score = self.criteria.eval(self.features.with_weights(&weights));
        let result = EvalResult { weights, score };
        self.current_results.push(result.clone());
        self.current_model_index += 1;

        let gen_index = self.current_gen_index;
        let model_index = self.current_model_index;

        let generation_result = if self.current_model_index == MODELS_PER_GEN {
            self.current_gen_index += 1;
            Some(self.finish_generation())
        } else {
            None
        };
        TrainState {
            gen_index,
            model_index,
            eval_result: result,
            generation: generation_result,
        }
    }

    fn finish_generation(&mut self) -> TrainGeneration {
        // Sort the results by score
        self.current_results
            .sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        let max = self.current_results.first().unwrap().score;
        let min = self.current_results.last().unwrap().score;
        let mean = self.current_results.iter().map(|r| r.score).sum::<f64>()
            / self.current_results.len() as f64;

        // Get the weights of the top N results
        let mut top_weights: Vec<Vec<f64>> =
            vec![Vec::with_capacity(KEPT_PER_GEN); self.weights.len()];
        for result in self.current_results.iter().take(KEPT_PER_GEN) {
            for (i, value) in result.weights.iter().enumerate() {
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

        self.normalize_weights();

        // reset
        self.current_gen = None;
        self.current_results.clear();
        self.current_model_index = 0;

        TrainGeneration {
            weights: self.weights.clone(),
            std_dev: self.st_dev.clone(),
            max,
            min,
            mean,
        }
    }

    pub fn is_stable(&self) -> bool {
        self.st_dev.iter().all(|&x| x < STABLE_THRESHOLD)
    }

    fn normalize_weights(&mut self) {
        let max_weight = self
            .weights
            .iter()
            .map(|&x| x.abs())
            .fold(f64::NEG_INFINITY, f64::max);
        for (weight, st_dev) in self.weights.iter_mut().zip(self.st_dev.iter_mut()) {
            *weight = *weight / max_weight * WEIGHT_RANGE;
            *st_dev = *st_dev / max_weight * WEIGHT_RANGE;
        }
    }
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Serialize)]
pub struct TrainGeneration {
    pub weights: Vec<f64>,
    pub std_dev: Vec<f64>,
    pub max: f64,
    pub min: f64,
    pub mean: f64,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, Clone, Serialize)]
pub struct TrainState {
    gen_index: usize,
    model_index: usize,
    eval_result: EvalResult,
    generation: Option<TrainGeneration>,
}

impl TrainState {
    pub fn gen_index(&self) -> usize {
        self.gen_index
    }

    pub fn model_index(&self) -> usize {
        self.model_index
    }

    pub fn eval_result(&self) -> &EvalResult {
        &self.eval_result
    }

    pub fn generation(&self) -> Option<&TrainGeneration> {
        self.generation.as_ref()
    }
}

impl FromStr for TrainCriteria {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "score" => Ok(TrainCriteria::Score),
            "level" => Ok(TrainCriteria::Level),
            "tetrises" => Ok(TrainCriteria::Tetrises),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct EvalResult {
    pub weights: Vec<f64>,
    pub score: f64,
}

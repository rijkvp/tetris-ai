use crate::{
    feature::{Weights, WeightsMap},
    simulator::Simulator,
};
use rand_distr::{Distribution, Normal};

const TEST_ITERATIONS: usize = 50;

pub struct Learner {
    weights: Vec<f64>,
    st_dev: Vec<f64>,
    models_per_gen: usize,
    kept_per_gen: usize,
}

impl Default for Learner {
    fn default() -> Self {
        let weights = Weights::from(WeightsMap::default()).into_values();
        let weights_len = weights.len();
        Learner {
            weights: vec![0.0; weights_len],
            st_dev: vec![10.0; weights_len],
            models_per_gen: 100,
            kept_per_gen: 10,
        }
    }
}

impl Learner {
    pub fn step(&mut self) {
        let mut generation = Vec::with_capacity(self.models_per_gen);

        let mut rng = rand::rng();

        for _ in 0..self.models_per_gen {
            let mut model_weights = Vec::with_capacity(self.weights.len());
            for (value, std_dev) in self.weights.iter().zip(self.st_dev.iter()) {
                let dist = Normal::new(*value, *std_dev).unwrap();
                model_weights.push(dist.sample(&mut rng));
            }
            generation.push(model_weights);
        }

        let weights_idk: Weights = WeightsMap::default().into();

        // Evaluate each model and collect (score, index) pairs
        let mut results = Vec::with_capacity(self.models_per_gen);
        for (i, model_weights) in generation.iter().enumerate() {
            let weights = weights_idk.with_values(model_weights);
            results.push((test(weights), i));
        }

        // Sort the results by score, and keep the top N
        results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        let top_indices = results
            .iter()
            .take(self.kept_per_gen)
            .map(|(_, i)| *i)
            .collect::<Vec<_>>();

        // Get the weights of the top indices
        let mut top_weights: Vec<Vec<f64>> =
            vec![Vec::with_capacity(self.kept_per_gen); self.weights.len()];
        for i in top_indices {
            for j in 0..self.weights.len() {
                top_weights[j].push(generation[i][j]);
            }
        }

        // Compute mean weights and new deviations
        for i in 0..self.weights.len() {
            let data = &top_weights[i];
            let mean = data.iter().sum::<f64>() / (data.len() as f64);
            let var = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (data.len() as f64);
            self.weights[i] = mean;
            self.st_dev[i] = var.sqrt();
        }
    }

    pub fn state(&self) -> (WeightsMap, &[f64]) {
        let default_map = WeightsMap::default();
        let names = default_map.names();
        let weight_map = WeightsMap::from_features_values(&names, &self.weights);
        (weight_map, &self.st_dev)
    }
}

fn test(weights: Weights) -> f64 {
    let mut sim = Simulator::new_with_weights(weights);
    sim.run_for(TEST_ITERATIONS);
    sim.stats().score as f64
}

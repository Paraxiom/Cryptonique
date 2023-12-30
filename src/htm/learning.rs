//learning.rs
use rand::Rng;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

// A learning algorithm with enhancements
pub struct LearningAlgorithm {
    learning_rate: f64,
    weights: Vec<f64>,
    momentum: f64,
    last_weight_update: Vec<f64>,
}

impl LearningAlgorithm {
    // Initialize with random weights and optional momentum
    pub fn new(size: usize, learning_rate: f64, momentum: Option<f64>) -> LearningAlgorithm {
        let mut rng = rand::thread_rng();
        let weights: Vec<f64> = (0..size).map(|_| rng.gen_range(-1.0..1.0)).collect();
        let momentum = momentum.unwrap_or(0.9);

        LearningAlgorithm {
            learning_rate,
            weights,
            momentum,
            last_weight_update: vec![0.0; size],
        }
    }

    // Train with a batch of data points
    pub fn train_batch(&mut self, inputs: &[Vec<f64>], targets: &[f64]) {
        let mut total_error = 0.0;

        for (input, &target) in inputs.iter().zip(targets) {
            let output = self.predict(input);
            let error = target - output;
            total_error += error.powi(2);

            for (i, &input_val) in input.iter().enumerate() {
                let weight_update = self.learning_rate * error * input_val + self.momentum * self.last_weight_update[i];
                self.weights[i] += weight_update;
                self.last_weight_update[i] = weight_update;
            }
        }

        let mse = total_error / inputs.len() as f64;
        println!("Mean Squared Error: {}", mse);
    }

    // Predict the output for a given input
    pub fn predict(&self, input: &[f64]) -> f64 {
        if input.len() != self.weights.len() {
            panic!("Input length does not match weights length");
        }
        
        input.iter().zip(&self.weights).map(|(&i, &w)| i * w).sum()
    }

    // Save the model to a file
    pub fn save_model<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let mut file = File::create(path)?;
        for weight in &self.weights {
            writeln!(file, "{}", weight)?;
        }
        Ok(())
    }

    pub fn load_model(path: &str, learning_rate: f64, momentum: f64) -> Result<Self, std::io::Error> {
        let contents = std::fs::read_to_string(path)?;
        let weights: Vec<f64> = contents.lines().map(|line| line.parse().unwrap()).collect();

        // Clone the weights before moving them
        let weights_clone = weights.clone();

        Ok(Self {
            learning_rate,
            momentum,
            weights,
            last_weight_update: vec![0.0; weights_clone.len()],
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_learning_algorithm() {
        let learning_rate = 0.1;
        let size = 10;
        let algorithm = LearningAlgorithm::new(size, learning_rate, Some(0.9));

        assert_eq!(algorithm.weights.len(), size);
        assert_eq!(algorithm.learning_rate, learning_rate);
    }

    #[test]
    fn test_train_batch() {
        let mut algorithm = LearningAlgorithm::new(3, 0.1, None);
        let inputs = vec![vec![1.0, 0.0, 0.5], vec![0.5, 1.0, 0.2]];
        let targets = vec![1.0, 0.0];

        algorithm.train_batch(&inputs, &targets);

        // Testing for change in weights, assuming learning rate is not zero
        assert_ne!(algorithm.weights, vec![0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_predict() {
        let mut algorithm = LearningAlgorithm::new(2, 0.1, None);
        algorithm.weights = vec![0.5, -0.3];

        let input = vec![1.0, 2.0];
        let output = algorithm.predict(&input);

        let expected_output = input[0] * algorithm.weights[0] + input[1] * algorithm.weights[1];
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_save_and_load_model() {
        let algorithm = LearningAlgorithm::new(3, 0.1, None);
        let path = "test_model.txt";

        algorithm.save_model(path).expect("Failed to save model");

        let loaded_algorithm = LearningAlgorithm::load_model(path, 0.1, 0.9).expect("Failed to load model");

        assert_eq!(algorithm.weights, loaded_algorithm.weights);
        std::fs::remove_file(path).expect("Failed to delete test model file");
    }
}

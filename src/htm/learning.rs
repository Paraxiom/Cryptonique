//learning.rs
use rand::Rng;

// A simple learning algorithm that adapts over time
pub struct LearningAlgorithm {
    learning_rate: f64,
    weights: Vec<f64>,
}

impl LearningAlgorithm {
    // Initialize a new learning algorithm with random weights
    pub fn new(size: usize, learning_rate: f64) -> LearningAlgorithm {
        let mut rng = rand::thread_rng();
        let weights: Vec<f64> = (0..size).map(|_| rng.gen()).collect();

        LearningAlgorithm {
            learning_rate,
            weights,
        }
    }

    // Train the learning algorithm with a new data point
    pub fn train(&mut self, input: &Vec<f64>, target: f64) {
        let output = self.predict(input);
        let error = target - output;

        for i in 0..self.weights.len() {
            self.weights[i] += self.learning_rate * error * input[i];
        }
    }

    // Predict the output for a given input
    pub fn predict(&self, input: &Vec<f64>) -> f64 {
        let mut output = 0.0;

        for i in 0..self.weights.len() {
            output += self.weights[i] * input[i];
        }

        output
    }
}
use ndarray::Array2;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;
use ndarray_npy::read_npy;
use ndarray_npy::write_npy;
use std::fs::File;

pub struct LearningAlgorithm {
    weights: Array2<f64>,
}

impl LearningAlgorithm {
    // Creates a new instance of LearningAlgorithm
    pub fn new() -> Self {
        // Initialize the weights randomly
        let weights = Array2::random((100, 100), Uniform::new(0., 1.));
        LearningAlgorithm {
            weights,
        }
    }

    // Applies the learning algorithm to the given data
    pub fn apply(&mut self, data: &[u8]) -> Vec<u8> {
        // Convert the data to an ndarray
        let data_array = Array2::from_shape_vec((10, 10), data.to_vec()).unwrap();

        // Apply the weights to the data
        let processed_data = self.weights.dot(&data_array);

        // Convert the processed data back to a Vec<u8>
        processed_data.into_raw_vec()
    }

    // Saves the weights to a file
    pub fn save_weights(&self, filename: &str) {
        let file = File::create(filename).unwrap();
        write_npy(file, &self.weights).unwrap();
    }

    // Loads the weights from a file
    pub fn load_weights(&mut self, filename: &str) {
        let file = File::open(filename).unwrap();
        self.weights = read_npy(file).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_learning_algorithm() {
        let mut learning_algorithm = LearningAlgorithm::new();
        let data = vec![1, 2, 3, 4, 5];
        let processed_data = learning_algorithm.apply(&data);
        assert_eq!(processed_data, data);
    }
}
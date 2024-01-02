//htm_model.rs
use crate::htm::columns::column::Column;
use crate::htm::dendrites::dendrite::Dendrite;
use crate::htm::encoders::scalar_encoder::ScalarEncoder;
use crate::htm::temporal_keys::TemporalKey;

use crate::htm::spatial_pooling::adapt_and_learn::adapt_and_learn;
use rand::Rng;
use std::collections::HashMap;
use std::time::SystemTime;
#[derive(Clone)]
pub struct HTMModel {
    // Placeholder fields for the POC
    dendrites: Vec<Dendrite>,
    columns: Vec<Column>,
    encoders: Vec<ScalarEncoder>,
    keys: HashMap<Vec<u8>, SystemTime>,
    learning_rate: f64,
    previous_state: Vec<u8>,
}

impl HTMModel {
    pub fn new() -> Self {
        HTMModel {
            // Initialize placeholder fields
            dendrites: vec![],
            columns: vec![],
            encoders: vec![],
            keys: HashMap::new(),
            learning_rate: 0.1,
            previous_state: vec![],
        }
    }

    pub fn generate_noise_pattern(&self) -> Vec<u8> {
        // For the POC, return a simple noise pattern
        let mut rng = rand::thread_rng();
        let mut noise_pattern = vec![0; 8];
        for byte in noise_pattern.iter_mut() {
            *byte = rng.gen();
        }
        noise_pattern
    }
    // Add the apply_transformation method

    pub fn learn(&mut self, data: &[u8]) {
        // 1. Encode the data
        for encoder in &self.encoders {
            for &byte in data {
                let encoded_data = encoder.encode(byte as f64); // Encode each byte

                // 2. Update columns based on the encoded data
                for (column, &data_bit) in self.columns.iter_mut().zip(&encoded_data) {
                    if data_bit > 0 {
                        column.activate();
                    } else {
                        column.deactivate();
                    }
                    column.update_cells();
                }
            }
        }

        // 3. Update dendrites and their synapses
        for dendrite in &mut self.dendrites {
            for segment in dendrite.get_segments_mut().iter_mut() {
                let active = segment.is_active();
                let strength_delta = self.learning_rate as f32; // Convert to f32
                segment.update_synapses(active, strength_delta);
            }
        }

        // 4. Optionally, use LearningAlgorithm for more complex learning logic
        // This could involve adjusting weights based on the HTM's state and the input data
    }

    pub fn store_key(&mut self, key: Vec<u8>, timestamp: SystemTime) {
        self.keys.insert(key, timestamp);
    }

    pub fn get_key(&self, key: &Vec<u8>) -> Option<&SystemTime> {
        self.keys.get(key)
    }
    pub fn compare_with_expected(&self, key: &TemporalKey) -> f32 {
        // Example implementation
        0.0
    }
    pub fn adapt_and_learn(&mut self, input: &[u8], learning_rate: f32) -> Vec<u8> {
        // Assuming 'some_state' is a mutable Vec<f32> required by adapt_and_learn
        let mut some_state: Vec<f32> = Vec::new();
         adapt_and_learn(input, &mut some_state, learning_rate)
    }
    pub fn apply_transformation(&mut self, input: &[u8]) -> Vec<u8> {
        let spatial_pooling_output: Vec<u8> = input
            .iter()
            .map(|&byte| self.simulate_spatial_pooling(byte))
            .flatten()
            .collect();
        self.simulate_temporal_memory(&spatial_pooling_output)
    }

    fn simulate_spatial_pooling(&self, byte: u8) -> Vec<u8> {
        let mut activated_columns = vec![0; 8];
        // Advanced logic to determine column activation
        // Here you can implement more complex algorithms that determine how each byte activates different columns
        for (i, column) in activated_columns.iter_mut().enumerate() {
            *column = if byte & (1 << i) != 0 { 1 } else { 0 };
        }
        activated_columns
    }

    fn simulate_temporal_memory(&mut self, input: &[u8]) -> Vec<u8> {
        // Implement a more complex temporal memory simulation
        // This could involve comparing the current input with the previous state to determine the output
        let mut temporal_memory_output = vec![];
        for (i, &value) in input.iter().enumerate() {
            // Compare with the previous state and decide on the output
            let previous_value = self.previous_state.get(i).cloned().unwrap_or(0);
            temporal_memory_output.push(self.calculate_temporal_response(value, previous_value));
        }
        // Update the previous state with the current input
        self.previous_state = input.to_vec();
        temporal_memory_output
    }
    fn calculate_temporal_response(&self, current_value: u8, previous_value: u8) -> u8 {
        // Implement logic to calculate the response based on current and previous values
        // For example, you could return a different value if the current and previous values are the same
        if current_value == previous_value {
            // Logic when current and previous values are the same
            0
        } else {
            // Logic for other cases
            current_value
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_htm_model_initialization() {
        let htm_model = HTMModel::new();
        assert!(htm_model.dendrites.is_empty());
        assert!(htm_model.columns.is_empty());
        assert!(htm_model.encoders.is_empty());
        assert!(htm_model.keys.is_empty());
        assert_eq!(htm_model.learning_rate, 0.1);
    }

    #[test]
    fn test_spatial_pooling() {
        let htm_model = HTMModel::new();
        let input_byte: u8 = 0b10101010; // Example input
        let activated_columns = htm_model.simulate_spatial_pooling(input_byte);

        // Validate that the correct columns are activated
        for (i, &column) in activated_columns.iter().enumerate() {
            let expected_state = if input_byte & (1 << i) != 0 { 1 } else { 0 };
            assert_eq!(
                column, expected_state,
                "Column {} activation state mismatch",
                i
            );
        }
    }

    #[test]
    fn test_temporal_memory() {
        let mut htm_model = HTMModel::new();
        let input = vec![0, 1, 1, 0, 1, 0, 0, 1]; // Example input
        let output = htm_model.simulate_temporal_memory(&input);

        // Validate the output
        assert_eq!(output, input, "Temporal memory output mismatch");
    }
}

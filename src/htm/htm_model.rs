//htm_model.rs
use crate::htm::columns::column::Column;
use crate::htm::dendrites::dendrite::Dendrite;
use crate::htm::encoders::scalar_encoder::ScalarEncoder;
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

    /// Apply a simplified transformation to simulate HTM's pattern recognition and temporal memory.
    pub fn apply_transformation(&self, input: &[u8]) -> Vec<u8> {
        let mut transformed = Vec::new();

        // Loop through each byte in the input
        for &byte in input.iter() {
            // Shift the bits of the byte to the left, wrapping around the leftmost bit
            let new_byte = (byte << 1) | (byte >> 7);

            // Append the transformed byte to the output vector
            transformed.push(new_byte);
        }

        transformed
    }

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

    // Additional tests...
}

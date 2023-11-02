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
}

impl HTMModel {
    pub fn new() -> Self {
        HTMModel {
            // Initialize placeholder fields
            dendrites: vec![],
            columns: vec![],
            encoders: vec![],
            keys: HashMap::new(),
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
        // Implement learning logic here
    }
    pub fn store_key(&mut self, key: Vec<u8>, timestamp: SystemTime) {
        self.keys.insert(key, timestamp);
    }

    pub fn get_key(&self, key: &Vec<u8>) -> Option<&SystemTime> {
        self.keys.get(key)
    }
}

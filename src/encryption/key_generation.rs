#![allow(warnings)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(warnings)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate rand;
use crate::encryption::key_generation::rand::prelude::SliceRandom;
use crate::encryption::key_generation::rand::Rng;
use crate::encryption::noise::{cellular_automata, chaotic_maps};
use crate::htm::{htm_model::HTMModel, temporal_keys::TemporalKey};
use std::time::Duration;
const KEY_SIZE: usize = 1000; // Define as per your requirement
const OVERLAP_PERCENTAGE: f32 = 0.775; // 77.5% overlap
const DEFAULT_SIZE: usize = 1000; // Adjust as per your requirement
const DEFAULT_ACTIVE_BIT_PERCENTAGE: f32 = 0.9; // 90% active bits
const DEFAULT_LEARNING_RATE: f32 = 0.9; // 90% active bits

// Define this in the appropriate module (e.g., at the top of key_generation.rs)

// Function to generate an SDR key
pub fn generate_sdr_key() -> (Vec<u8>, Vec<u8>) {
    let htm_model = HTMModel::new();
    let initial_key = vec![0; 256]; // A key of all zeros for simplicity.

    // Create a TemporalKey with the HTMModel instance
    let mut temporal_key = TemporalKey::new(initial_key, htm_model, Duration::from_secs(10));
    temporal_key.evolve_key(1, 1, DEFAULT_LEARNING_RATE); // Replace `some_learning_rate` with an appropriate f32 value
                                                          // Evolve the key once for this example
    let sdr_a = temporal_key.get_key().clone();

    let sdr_b = chaotic_maps::perturb(&sdr_a, 0.5); // Apply chaotic map perturbation

    (sdr_a, sdr_b)
}
fn generate_key_with_overlap(base_key: &[u8]) -> Vec<u8> {
    println!("Generating key with overlap...");
    let mut rng = rand::thread_rng();

    // Calculate the total number of active bits needed
    let total_active_bits = (KEY_SIZE as f32 * DEFAULT_ACTIVE_BIT_PERCENTAGE).round() as usize;

    // Initialize the new key with the base key
    let mut new_key = base_key.to_vec();

    // Count the number of active bits in the base key
    let base_active_bits = base_key.iter().filter(|&&bit| bit == 1).count();
    println!("Base active bits: {}", base_active_bits);

    // Calculate and set additional active bits if needed
    if base_active_bits < total_active_bits {
        let additional_active_bits = total_active_bits - base_active_bits;
        println!("Additional active bits needed: {}", additional_active_bits);

        // Randomly set additional bits to 1
        let mut potential_positions: Vec<usize> =
            (0..KEY_SIZE).filter(|&i| base_key[i] == 0).collect();
        potential_positions.shuffle(&mut rng);

        for &pos in potential_positions.iter().take(additional_active_bits) {
            new_key[pos] = 1;
        }
    }

    // Debug print the final key
    println!("Final key: {:?}", new_key);
    new_key
}

fn calculate_probability_of_one(overlap_size: usize) -> f32 {
    let expected_ones_total = (KEY_SIZE as f32 * 0.90) as usize; // 90% of bits are active
    let ones_needed = expected_ones_total.saturating_sub(overlap_size);
    let positions_remaining = KEY_SIZE - overlap_size;
    ones_needed as f32 / positions_remaining as f32
}

fn set_remaining_positions_with_probability(new_key: &mut Vec<u8>, probability: f32) {
    let mut rng = rand::thread_rng();
    for bit in new_key.iter_mut().filter(|bit| **bit == 0) {
        *bit = if rng.gen::<f32>() < probability { 1 } else { 0 };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn generate_key() -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let mut sdr = vec![0; KEY_SIZE]; // Initialize the key with zeros
        let active_bits = (KEY_SIZE as f32 * DEFAULT_ACTIVE_BIT_PERCENTAGE).round() as usize;

        // Randomly choose positions to set to 1
        let mut positions: Vec<usize> = (0..KEY_SIZE).collect();
        positions.shuffle(&mut rng);
        for pos in positions.into_iter().take(active_bits) {
            sdr[pos] = 1;
        }

        sdr
    }
    fn generate_keys(config: KeyGenConfig) -> (Vec<u8>, Vec<u8>) {
        let key_a = generate_key(); // Generate the first key
        let key_b = generate_key_with_overlap(&key_a); // Generate the second key with overlap to the first
        (key_a, key_b)
    }

    pub struct KeyGenConfig {
        // Add fields and types as per your requirements
        // Example fields:
        pub size: usize,
        pub active_bit_percentage: f32,
        // Add other fields as needed
    }

    impl Default for KeyGenConfig {
        fn default() -> Self {
            KeyGenConfig {
                // Set default values for the fields
                size: 1000, // example default value
                active_bit_percentage: 0.9, // example default value
                            // Set other default values as needed
            }
        }
    }

    #[test]
    fn test_key_generation() {
        let (key_a, key_b) = generate_keys(KeyGenConfig::default());
    
        // Debugging key lengths
        println!("Key A Length: {}, Key B Length: {}", key_a.len(), key_b.len());
    
        assert_eq!(key_a.len(), KEY_SIZE);
        assert_eq!(key_b.len(), KEY_SIZE);
    
        // Calculate overlap
        let overlap = key_a
            .iter()
            .zip(key_b.iter())
            .filter(|(&a, &b)| a == 1 && b == 1)
            .count();
    
        let expected_overlap = (KEY_SIZE as f32 * OVERLAP_PERCENTAGE).round() as usize;
    
        // Debugging overlap
        println!("Calculated Overlap: {}, Expected Overlap: {}", overlap, expected_overlap);
    
        assert_eq!(overlap, expected_overlap);
    }
    
    #[test]
    fn test_overlap() {
        let (sdr_a, sdr_b) = generate_sdr_key(); // Directly use `generate_sdr_key` to get the pair

        let overlap = sdr_a
            .iter()
            .zip(&sdr_b)
            .filter(|(&a, &b)| a == 1 && b == 1)
            .count();
        let overlap_percentage = overlap as f32 / KEY_SIZE as f32; // Use KEY_SIZE for calculation

        println!("Actual overlap percentage: {}", overlap_percentage);

        assert!(
            (overlap_percentage - OVERLAP_PERCENTAGE).abs() <= 0.03,
            "Overlap percentage deviates from expected range"
        );
    }

    #[test]
    fn test_distribution() {
        let config = KeyGenConfig::default();
        let (_, sdr_b) = generate_keys(config);
        let active_bits = sdr_b.iter().filter(|&&bit| bit == 1).count();
        let active_bit_percentage = active_bits as f32 / DEFAULT_SIZE as f32;
        println!("Actual active bit percentage: {}", active_bit_percentage);
        // Check if the active bits are close to the expected percentage
        assert!((active_bit_percentage - DEFAULT_ACTIVE_BIT_PERCENTAGE).abs() <= 0.03);
    }
}

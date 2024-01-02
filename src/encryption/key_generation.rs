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
use rand::thread_rng;
use std::collections::HashSet;
const KEY_SIZE: usize = 256; // Define as per your requirement
const OVERLAP_PERCENTAGE: f32 = 0.3; // 77.5% overlap
const DEFAULT_SIZE: usize = 1000; // Adjust as per your requirement
const DEFAULT_ACTIVE_BIT_PERCENTAGE: f32 = 0.9; // 90% active bits
const DEFAULT_LEARNING_RATE: f32 = 0.9; // 90% active bits

#[derive(Debug)] 
pub struct KeyGenConfig {
    pub key_b: Vec<u8>, // Specify the correct type for key_b
    pub size: usize,
    pub active_bit_percentage: f32,
}

// Default configuration settings
impl Default for KeyGenConfig {
    fn default() -> Self {
        KeyGenConfig {
            key_b: Vec::new(), // Assuming key_b is a Vec<u8>, initialize appropriately
            size: KEY_SIZE,
            active_bit_percentage: DEFAULT_ACTIVE_BIT_PERCENTAGE,
        }
    }
}

fn generate_keys(config: KeyGenConfig) -> (Vec<u8>, Vec<u8>) {
    println!("Generating Key A with configuration: {:?}", config); // Debug
    let key_a = generate_key(); // Generate the first key
    println!("Generated Key A: {:?}", key_a); // Debug

    println!("Generating Key B with overlap to Key A"); // Debug
    let key_b = generate_key_with_overlap(&key_a, OVERLAP_PERCENTAGE);
    println!("Generated Key B: {:?}", key_b); // Debug

    (key_a, key_b)
}

fn generate_key() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut key = vec![0; KEY_SIZE];
    let active_bits_count = (KEY_SIZE as f32 * DEFAULT_ACTIVE_BIT_PERCENTAGE).round() as usize;

    let mut positions: Vec<usize> = (0..KEY_SIZE).collect();
    positions.shuffle(&mut rng);
    for &pos in positions.iter().take(active_bits_count) {
        key[pos] = 1;
    }

    key
}


fn generate_key_with_overlap(key_a: &[u8], overlap_percentage: f32) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let key_length = key_a.len();
    let overlap_count = (key_length as f32 * overlap_percentage).round() as usize;

    // Randomly select indices for overlapping bits
    let mut indices: Vec<usize> = (0..key_length).collect();
    indices.shuffle(&mut rng);
    let overlap_indices: HashSet<usize> = indices.iter().cloned().take(overlap_count).collect();

    // Create Key B with controlled overlap
    let mut key_b = vec![0; key_length];
    for i in 0..key_length {
        if overlap_indices.contains(&i) {
            // Overlapping bit
            key_b[i] = key_a[i];
        } else {
            // Random non-overlapping bit
            key_b[i] = rng.gen_range(0..2);
        }
    }

    key_b
}







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


mod tests {
    use super::*;

    use super::generate_key;
    use super::generate_key_with_overlap;
    use super::KeyGenConfig;
    use super::generate_keys;

    #[test]
fn test_key_generation() {
    let (key_a, key_b) = generate_keys(KeyGenConfig::default());

    // Debugging key lengths
    println!("Key A Length: {}, Key B Length: {}", key_a.len(), key_b.len());
    assert_eq!(key_a.len(), KEY_SIZE);
    assert_eq!(key_b.len(), KEY_SIZE);

    // Calculate and debug overlap
    let overlap = key_a
        .iter()
        .zip(key_b.iter())
        .filter(|(&a, &b)| a == 1 && b == 1)
        .count();
    let expected_overlap = (KEY_SIZE as f32 * OVERLAP_PERCENTAGE).round() as usize;
    println!("Calculated Overlap: {}, Expected Overlap: {}", overlap, expected_overlap);

    // Debug keys if overlap is not as expected
    if overlap != expected_overlap {
        println!("Key A: {:?}", key_a);
        println!("Key B: {:?}", key_b);
    }
    
    assert_eq!(overlap, expected_overlap);
}


#[test]
fn test_overlap() {
    let (key_a, key_b) = generate_keys(KeyGenConfig::default());

    let overlap = key_a
        .iter()
        .zip(&key_b)
        .filter(|(&a, &b)| a == 1 && b == 1)
        .count();
    let overlap_percentage = overlap as f32 / KEY_SIZE as f32;

    println!("Calculated Overlap Percentage: {}, Expected: {}", overlap_percentage, OVERLAP_PERCENTAGE);

    // Debug keys if overlap percentage deviates
    if (overlap_percentage - OVERLAP_PERCENTAGE).abs() > 0.03 {
        println!("Key A: {:?}", key_a);
        println!("Key B: {:?}", key_b);
    }

    assert!(
        (overlap_percentage - OVERLAP_PERCENTAGE).abs() <= 0.03,
        "Overlap percentage deviates from expected range"
    );
}


#[test]
fn test_distribution() {
    let (_, key_b) = generate_keys(KeyGenConfig::default());
    let active_bits = key_b.iter().filter(|&&bit| bit == 1).count();
    let active_bit_percentage = active_bits as f32 / KEY_SIZE as f32;

    // Debug: Print the key and the calculated active bit percentage
    println!("Debug: Key B: {:?}", key_b);
    println!("Debug: Active bit percentage: {}", active_bit_percentage);

    // Check if the active bits are close to the expected percentage
    assert!((active_bit_percentage - DEFAULT_ACTIVE_BIT_PERCENTAGE).abs() <= 0.03);
}
}

#![allow(warnings)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
pub extern crate rand;
use crate::encryption::asymmetric_encryption::keygen::PrivateKey;
use crate::encryption::asymmetric_encryption::keygen::PublicKey;
use crate::encryption::noise::cellular_automata;
use crate::encryption::noise::chaotic_maps;
use crate::htm::htm_model::HTMModel;
use crate::htm::temporal_keys::generate_temporal_key;
use crate::htm::temporal_keys::TemporalKey;
use rand::distributions::Uniform;
use rand::seq::SliceRandom;
extern crate ring;
use ring::rand::SecureRandom;
use ring::rand::SystemRandom;

pub const DEFAULT_SIZE: usize = 1000;
pub const DEFAULT_ACTIVE_BIT_PERCENTAGE: f32 = 0.90; // 90% of bits are active
pub const DEFAULT_PERTURBATION_PERCENTAGE: f32 = 0.09; // 10% of active bits are flipped
const KEY_SIZE: usize = 1000; // Define as per your requirement
const OVERLAP_PERCENTAGE: f32 = 0.775; // 77.5% overlap
use rand::Rng;

pub struct KeyGenConfig {
    size: usize,
    active_bit_percentage: f32,
    perturbation_percentage: f32,
}

impl Default for KeyGenConfig {
    fn default() -> Self {
        KeyGenConfig {
            size: DEFAULT_SIZE,
            active_bit_percentage: DEFAULT_ACTIVE_BIT_PERCENTAGE,
            perturbation_percentage: DEFAULT_PERTURBATION_PERCENTAGE,
        }
    }
}

// Function to generate an SDR key
pub fn generate_sdr_key() -> (Vec<u8>, Vec<u8>) {
    // Create a new TemporalKey
    let htm_model = HTMModel::new();
    let initial_key = vec![0; 256]; // A key of all zeros for simplicity.
    let sdr_a = generate_temporal_key(&htm_model, &initial_key);

    let sdr_b = cellular_automata::evolve(&sdr_a, 5);
    let sdr_b = chaotic_maps::perturb(&sdr_b, 0.5);

    (sdr_a, sdr_b)
}

pub fn generate_sdr_keys() -> (Vec<u8>, Vec<u8>) {
    let htm_model = HTMModel::new();
    let initial_key = vec![0; 256]; // A key of all zeros for simplicity.
    let sdr_a = generate_temporal_key(&htm_model, &initial_key);

    let sdr_b = cellular_automata::evolve(&sdr_a, 5);
    let sdr_b = chaotic_maps::perturb(&sdr_b, 0.5);

    (sdr_a, sdr_b)
}

pub fn generate_keys(config: KeyGenConfig) -> (Vec<u8>, Vec<u8>) {
    let sdr_a = generate_key();
    let sdr_b = generate_key_with_overlap(&sdr_a);
    (sdr_a, sdr_b)
}

fn generate_key() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut sdr = vec![0; KEY_SIZE];
    let overlap_size = (KEY_SIZE as f32 * OVERLAP_PERCENTAGE) as usize;

    // Ensure that at least overlap_size bits are set to 1.
    let mut ones_positions: Vec<usize> = (0..KEY_SIZE).collect();
    ones_positions.shuffle(&mut rng);
    for &pos in ones_positions.iter().take(overlap_size) {
        sdr[pos] = 1;
    }

    sdr
}

fn generate_key_with_overlap(base_key: &[u8]) -> Vec<u8> {
    let overlap_size = (KEY_SIZE as f32 * OVERLAP_PERCENTAGE) as usize;
    let mut new_key = vec![0; KEY_SIZE];

    // Identify positions where base_key has 1s
    let ones_positions: Vec<usize> = base_key
        .iter()
        .enumerate()
        .filter_map(|(i, &bit)| if bit == 1 { Some(i) } else { None })
        .collect();

    // Calculate the adjusted probability for setting 1s for the remaining positions
    let expected_ones_total = (KEY_SIZE as f32 * DEFAULT_ACTIVE_BIT_PERCENTAGE) as usize;
    let ones_needed = expected_ones_total.saturating_sub(overlap_size);
    let positions_remaining = KEY_SIZE - overlap_size;
    let probability_of_one = ones_needed as f32 / positions_remaining as f32;

    // Set the overlap positions in the new key
    for &pos in &ones_positions {
        new_key[pos] = 1;
    }

    // For the positions in the new key that are not part of the overlap, set them with adjusted probability
    let mut rng = rand::thread_rng();
    for i in 0..KEY_SIZE {
        if new_key[i] == 0 {
            new_key[i] = if rng.gen::<f32>() < probability_of_one {
                1
            } else {
                0
            };
        }
    }

    new_key
}

pub fn generate_key_pairs() -> (Vec<u8>, Vec<u8>) {
    let sdr_a = generate_key();
    let sdr_b = generate_key_with_overlap(&sdr_a);
    (sdr_a, sdr_b)
}
pub fn generate_temporal_keys(config: KeyGenConfig) -> (Vec<u8>, Vec<u8>) {
    let initial_key = vec![0; 256]; // Example: a vector of 256 zeros.
    let htm_model = HTMModel::new(); // Assuming HTMModel::new() is a valid constructor.

    let evolution_interval = std::time::Duration::from_secs(10); // 10 seconds, for example
    let mut temporal_key = TemporalKey::new(initial_key, htm_model, evolution_interval);

    let sdr_a = temporal_key.get_key().clone();

    // evolve the key
    temporal_key.evolve_key(1, 1);
    let sdr_b = temporal_key.get_key().clone();

    (sdr_a, sdr_b)
}

pub fn generate_key_pair() -> (PublicKey, PrivateKey) {
    let public_key = PublicKey::default();
    let private_key = PrivateKey {
        exponent: public_key.exponent.clone(),
        modulus: public_key.modulus.clone(),
    };
    (public_key, private_key)
}

// Generate a cryptographic key using chaotic sequences.
pub fn generate_key_with_chaos(initial_conditions: f64) -> String {
    // Generate a chaotic sequence using the logistic map
    let mut x = initial_conditions;
    let r = 3.9; // Parameter for the logistic map

    let mut key = String::new();
    for _ in 0..256 {
        x = r * x * (1.0 - x);
        key.push(((x * 256.0) as u8) as char);
    }

    key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_generation() {
        let (key_a, key_b) = generate_keys(KeyGenConfig::default());

        assert_eq!(key_a.len(), KEY_SIZE);
        assert_eq!(key_b.len(), KEY_SIZE);

        // Calculate overlap
        let overlap = key_a
            .iter()
            .zip(key_b.iter())
            .filter(|(&a, &b)| a == 1 && b == 1)
            .count();

        assert_eq!(overlap, (KEY_SIZE as f32 * OVERLAP_PERCENTAGE) as usize);
    }
    #[test]
    fn test_overlap() {
        let config = KeyGenConfig::default();
        let (sdr_a, sdr_b) = generate_keys(config);
        let overlap = sdr_a
            .iter()
            .zip(sdr_b.iter())
            .filter(|(&a, &b)| a == b && a == 1)
            .count();
        let overlap_percentage = overlap as f32 / DEFAULT_SIZE as f32;
        println!("Actual overlap percentage: {}", overlap_percentage);

        // Check if the overlap is close to the set OVERLAP_PERCENTAGE
        assert!((overlap_percentage - OVERLAP_PERCENTAGE).abs() <= 0.03);
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

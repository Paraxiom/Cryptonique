#![allow(warnings)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::encryption::confusion_diffusion::confusion::apply_confusion;
use crate::encryption::confusion_diffusion::feistel_network::FeistelNetwork;
use crate::encryption::fft_initialize::fft_initialize;
use crate::encryption::hashing;
use crate::encryption::hashing::hash;
use crate::encryption::hashing::HashType::SHA256;
use crate::encryption::key_generation;

use crate::encryption::noise::chaotic_maps;

use crate::htm::htm_model::HTMModel;

use crate::htm::key_evolution::quantum_evolve_key;
use crate::shared_state::SharedState;
extern crate num_complex;
use chrono::prelude::*;
use num_complex::Complex;
use prost::Message;
use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;
extern crate rand; // Assume you're using the 'rand' crate for random numbers
use rand::Rng;
use rand::{rngs::StdRng, SeedableRng};
use rand::{thread_rng, RngCore};
use std::collections::HashSet;

use crate::htm::key_properties::KeyProperties;
const SOME_ENTROPY_THRESHOLD: f64 = 200.0;

const SOME_THRESHOLD: f64 = 4.0;
pub struct TemporalKey {
    current_key: Vec<u8>,
    pub generation_time: SystemTime,
    evolution_interval: Duration,
    htm_model: HTMModel,
    evolution_steps: usize,
}

impl TemporalKey {
    pub fn new(initial_key: Vec<u8>, htm_model: HTMModel, evolution_interval: Duration) -> Self {
        TemporalKey {
            current_key: initial_key,
            generation_time: SystemTime::now(),
            evolution_interval,
            htm_model, // Assigning the passed HTMModel
            evolution_steps: 0,
        }
    }
    fn prepare_htm_input(&self, key: &[u8]) -> Vec<u8> {
        // For demonstration, we'll simply convert the key into a binary representation
        let mut htm_input = Vec::new();

        for &byte in key {
            // Convert each byte to its binary representation
            for i in 0..8 {
                // Extract each bit using bitwise operations
                let bit = (byte >> (7 - i)) & 1;
                htm_input.push(bit);
            }
        }

        htm_input
    }

    pub fn evolve_key(&mut self, num_iterations: usize, steps: usize, learning_rate: f32) {
        let mut rng = rand::thread_rng();

        for _ in 0..num_iterations {
            // Convert part of the key to a format suitable for HTM processing
            let htm_input = self.prepare_htm_input(&self.current_key);

            // Apply the adapt and learn process to the HTM model
            let adapted_result = self.htm_model.adapt_and_learn(&htm_input, learning_rate);

            // Incorporate the adapted result from HTM into the key transformation
            let htm_transformed_key = self.apply_htm_adaptation(&adapted_result);

            // Apply additional complex transformations
            let transformed_key = Self::complex_transformation(
                htm_transformed_key.encode_to_vec(),
                &mut rng,
                &mut self.htm_model,
            );

            // Check the transformed key's properties
            if transformed_key.len() != self.current_key.len() {
                println!("Key transformation altered the key length. Re-evolving...");
                continue;
            }

            if Self::calculate_entropy(&transformed_key) < SOME_ENTROPY_THRESHOLD {
                println!("Key transformation resulted in low entropy. Re-evolving...");
                continue;
            }

            if !resists_differential_cryptanalysis(&transformed_key) {
                println!("Key failed differential cryptanalysis check. Re-evolving...");
                continue;
            }

            // Update the current key if it passes all checks
            self.current_key = transformed_key;
            println!("Key successfully transformed.");

            // Key validated
            break;
        }

        self.evolution_steps += steps;
        if self.validate_key() {
            println!("Key successfully evolved and validated.");
        } else {
            println!("Key evolution failed validation.");
            // Consider logic to regenerate or re-evolve the key here if validation fails
        }
    }

    fn complex_transformation(
        mut key: Vec<u8>,
        rng: &mut impl Rng,
        htm_model: &mut HTMModel,
    ) -> Vec<u8> {
        // Step 1: Apply Chaotic Map Transformation
        key = chaotic_maps::apply_chaotic_map(&key);

        // Step 2: Modify the hashing step to preserve key length
        let hash_part = hashing::hash(&key[0..16], hashing::HashType::SHA256); // Hashing first 16 bytes
        let key_len = key.len(); // Store the length of the key to avoid simultaneous mutable and immutable borrows

        for (i, &byte) in hash_part.iter().enumerate() {
            let index = i % key_len; // Calculate the index using the stored key length
            key[index] ^= byte; // Mixing hashed bytes back into the key
        }

        // Step 3: Apply HTM Model Transformation
        key = htm_model.apply_transformation(&key);

        // Step 4: Final mix
        key = Self::final_mix(&key, rng);
        key
    }

    fn final_mix(key: &[u8], rng: &mut impl Rng) -> Vec<u8> {
        // Implement your logic for the final mix
        key.iter()
            .map(|&x| rng.gen::<u8>().wrapping_add(x))
            .collect()
    }

    // Method to get the number of evolution steps the key has undergone
    pub fn get_evolution_steps(&self) -> usize {
        self.evolution_steps
    }
    fn validate_key(&self) -> bool {
        println!("Starting key validation process...");

        // Length Check
        println!("Checking key length...");
        if self.current_key.len() != 256 {
            println!(
                "Validation failed: Key length is not 256. Actual length: {}",
                self.current_key.len()
            );
            return false;
        } else {
            println!(
                "Key length check passed. Length: {}",
                self.current_key.len()
            );
        }

        // Temporal Check
        println!("Checking key age...");
        match SystemTime::now().duration_since(self.generation_time) {
            Ok(elapsed) => {
                if elapsed > self.evolution_interval {
                    println!(
                        "Validation failed: Key has expired. Elapsed time: {:?}",
                        elapsed
                    );
                    return false;
                } else {
                    println!("Key age check passed. Elapsed time: {:?}", elapsed);
                }
            }
            Err(_) => {
                println!(
                    "System time is set before the key was generated. This should not happen."
                );
                return false;
            }
        }

        // Entropy Check
        println!("Checking key entropy...");
        if !check_key_entropy(&self.current_key) {
            println!("Validation failed: Key entropy is too low.");
            return false;
        } else {
            println!("Key entropy check passed.");
        }

        println!("Key validation successful.");
        true
    }

    // Helper functions

    fn check_key_entropy(key: &[u8]) -> bool {
        Self::calculate_entropy(key) > SOME_ENTROPY_THRESHOLD
    }

    fn calculate_entropy(key: &[u8]) -> f64 {
        let mut occurrences = [0usize; 256]; // Count occurrences of each byte
        let len = key.len();

        for &byte in key {
            occurrences[byte as usize] += 1;
        }

        let entropy: f64 = occurrences
            .iter()
            .filter(|&&count| count > 0)
            .map(|&count| {
                let probability = (count as f64) / (len as f64);
                -probability * probability.log2() // Shannon entropy formula
            })
            .sum();

        entropy
    }

    pub fn get_key(&self) -> &Vec<u8> {
        &self.current_key
    }

    // Additional methods for evolving the key based on an interval
    pub fn evolve(&mut self) {
        let now = Utc::now();
        if now.signed_duration_since(DateTime::<Utc>::from(self.generation_time))
            >= chrono::Duration::from_std(self.evolution_interval).unwrap()
        {
            self.evolve_key(1, 1, 0.1); // Evolve the key once
            self.generation_time = SystemTime::now();
        }
    }

    pub fn quantum_evolve_key(&mut self) {
        // Additional debugging info
        println!("Starting key evolution...");

        let original_key = self.current_key.clone();
        let original_length = original_key.len();
        let mut evolution_attempts = 0;
        const MAX_EVOLUTION_ATTEMPTS: usize = 10;

        while evolution_attempts < MAX_EVOLUTION_ATTEMPTS {
            self.apply_complex_transformations();

            if self.current_key.len() != original_length {
                println!(
                    "Evolution attempt {}: Key length altered.",
                    evolution_attempts + 1
                );
            } else if !self.validate_key() {
                println!(
                    "Evolution attempt {}: Key failed validation.",
                    evolution_attempts + 1
                );
            } else if !Self::is_key_sufficiently_evolved(&original_key, &self.current_key) {
                println!(
                    "Evolution attempt {}: Insufficient evolution.",
                    evolution_attempts + 1
                );
            } else {
                println!("Key evolution successful.");
                self.generation_time = SystemTime::now();
                self.evolution_steps += 1;
                return;
            }

            self.current_key = Self::regenerate_key();
            evolution_attempts += 1;
        }

        println!("Maximum evolution attempts reached. Final key used.");
    }

    // Define the minimum difference threshold for key evolution
    const SOME_MIN_DIFFERENCE_THRESHOLD: usize = 50; // Example value, adjust as needed

    // Additional method to check if the evolved key is sufficiently different from the original
    fn is_key_sufficiently_evolved(original_key: &[u8], evolved_key: &[u8]) -> bool {
        // Implement logic to determine if the key has evolved sufficiently
        original_key
            .iter()
            .zip(evolved_key.iter())
            .filter(|&(a, b)| a != b)
            .count()
            >= Self::SOME_MIN_DIFFERENCE_THRESHOLD
    }

    // Regenerate key method
    fn regenerate_key() -> Vec<u8> {
        // Placeholder logic for new key generation
        let new_key = vec![0; 256];
        new_key
    }
    // Method for applying complex transformations to the key
    fn apply_complex_transformations(&mut self) {
        // Step 1: Apply Classical Transformations
        // Example: Using Feistel networks for encryption
        self.apply_feistel_network();

        // Step 3: Additional Transformations
        // Example: Applying chaotic maps or other non-linear transformations
        self.apply_chaotic_maps();
    }

    fn apply_feistel_network(&mut self) {
        let feistel_network = FeistelNetwork::new();
        self.current_key = feistel_network.encrypt(&self.current_key);
    }

    // Placeholder for applying chaotic maps
    fn apply_chaotic_maps(&mut self) {
        self.current_key = self
            .current_key
            .iter()
            .map(|&x| x.wrapping_add(1))
            .collect();
    }
    // Method to calculate the value of the key based on its entropy
    pub fn value(&self) -> f32 {
        // Converting the entropy (which is in double) to a float value for this method
        Self::calculate_entropy(&self.current_key) as f32
    }

    // Method to calculate an anomaly score for the key
    pub fn anomaly_score(&self) -> f32 {
        // Example: Calculating deviation from expected patterns
        // This is a placeholder logic; you should replace it with your actual calculation.
        let expected_pattern_score = self.calculate_expected_pattern_score();
        1.0 / expected_pattern_score // Higher score for greater deviation
    }
    // Function to calculate a score based on expected patterns of the key
    fn calculate_expected_pattern_score(&self) -> f32 {
        // Calculate the deviation from the uniform distribution
        let deviation = self.calculate_deviation_from_uniform_distribution();
        // Normalize the deviation to a score between 0 and 1
        // This is a basic normalization; you might need a more complex approach based on your requirements
        1.0 - (1.0 / (1.0 + deviation))
    }

    // Function to calculate deviation from uniform distribution of byte values in the key
    fn calculate_deviation_from_uniform_distribution(&self) -> f32 {
        let mut byte_counts = [0usize; 256];
        for &byte in &self.current_key {
            byte_counts[byte as usize] += 1;
        }

        let expected_count = self.current_key.len() / 256;
        byte_counts.iter().fold(0.0, |acc, &count| {
            let diff = (count as isize - expected_count as isize).abs() as f32;
            acc + diff
        }) / self.current_key.len() as f32
    }
    fn apply_htm_adaptation(&mut self, adapted_result: &[u8]) {
        // Implement the adaptation logic here
        // Example:
        // self.current_key = adapted_result.to_vec();
    }
}

fn create_quantum_representation(key: &[u8]) -> Vec<Complex<f32>> {
    key.iter()
        .map(|&byte| {
            // Convert each byte to a quantum state
            // This is a basic example where we map the byte value to the real part of the complex number
            // and set the imaginary part to zero.
            // You can modify this logic to fit the specifics of your quantum representation.
            Complex::new(byte as f32, 0.0)
        })
        .collect()
}

fn determine_function_type_based_on_key(key: &[u8]) -> u8 {
    let sum: u64 = key.iter().map(|&x| x as u64).sum(); // Use u64 to prevent overflow
    if sum % 2 == 0 {
        0 // Constant
    } else {
        1 // Balanced
    }
}

fn determine_function_type() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..2) // Randomly returns 0 or 1
}

pub fn generate_temporal_key(htm_model: &mut HTMModel, initial_key: &[u8]) -> Vec<u8> {
    let mut current_key = initial_key.to_vec();

    // For the purpose of this example, we'll evolve the key 10 times.
    for _ in 0..10 {
        current_key = quantum_evolve_key(htm_model, &current_key);
    }

    current_key
}

fn generate_high_entropy_key(length: usize) -> Vec<u8> {
    let mut rng = StdRng::from_entropy();
    let mut key = vec![0u8; length];
    rng.fill_bytes(&mut key);
    key
}
// fn calculate_entropy(key: &[u8]) -> f64 {
//     let mut byte_counts = [0u32; 256];
//     for &byte in key {
//         byte_counts[byte as usize] += 1;
//     }
//     let len = key.len() as f64;
//     byte_counts.iter().fold(0.0, |entropy, &count| {
//         if count > 0 {
//             let probability = count as f64 / len;
//             entropy - (probability * probability.log2())
//         } else {
//             entropy
//         }
//     })
// }
#[cfg(test)]
mod tests {
    use super::*;
    use crate::htm::htm_model::HTMModel;
    use std::sync::Arc;
    use std::time::Duration;

    use crate::encryption::confusion_diffusion::confusion::apply_confusion;
    use crate::encryption::confusion_diffusion::feistel_network::FeistelNetwork;
    use crate::encryption::fft_initialize::fft_initialize;
    use crate::encryption::hashing::hash;
    use crate::encryption::hashing::HashType::SHA256;

    #[test]
    fn test_resists_differential_cryptanalysis() {
        // Key with no repeated patterns
        let key_no_pattern = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        assert!(
            resists_differential_cryptanalysis(&key_no_pattern),
            "Key with no pattern should pass the check"
        );

        // Key with immediate repetition
        let key_immediate_repetition = vec![1, 1, 3, 4, 5, 6, 7, 8, 9, 10];
        assert!(
            !resists_differential_cryptanalysis(&key_immediate_repetition),
            "Key with immediate repetition should fail the check"
        );

        // Key with a repeated sequence
        let key_repeated_sequence = vec![1, 2, 3, 4, 1, 2, 3, 4, 5, 6];
        assert!(
            !resists_differential_cryptanalysis(&key_repeated_sequence),
            "Key with repeated sequence should fail the check"
        );

        // Key with disproportionate byte occurrence
        let key_high_byte_occurrence = vec![1, 1, 1, 1, 2, 3, 4, 5, 6, 7];
        assert!(
            !resists_differential_cryptanalysis(&key_high_byte_occurrence),
            "Key with high byte occurrence should fail the check"
        );
    }

    // #[test]
    // fn test_quantum_evolution_impact() {
    //     let htm_model = HTMModel::new();
    //     let initial_key = generate_high_entropy_key(256);
    //     let mut temporal_key =
    //         TemporalKey::new(initial_key.clone(), htm_model, Duration::from_secs(10));

    //     // Before evolution
    //     let key_before_evolution = temporal_key.get_key().clone();
    //     // Perform quantum evolution
    //     temporal_key.quantum_evolve_key();

    //     // After evolution
    //     let key_after_evolution = temporal_key.get_key().clone();

    //     // Check entropy of the evolved key
    //     let entropy_after = calculate_entropy(&key_after_evolution);

    //     // Adjusted threshold for testing
    //     let expected_entropy_threshold = 2.0; // Example adjustment for testing

    //     assert!(
    //         entropy_after > expected_entropy_threshold,
    //         "Evolved key should have high entropy. Found entropy: {}",
    //         entropy_after
    //     );
    // }

    use rand::distributions::{Distribution, Uniform};

    // Helper function to generate a high-entropy key of a given length
    fn generate_high_entropy_key(length: usize) -> Vec<u8> {
        // Using a cryptographically secure random number generator
        let mut rng = rand::thread_rng();

        // Uniform distribution ensures that each byte value is equally likely
        let between = Uniform::from(0..=255);

        // Generate a vector of random bytes with uniform distribution
        let key: Vec<u8> = (0..length).map(|_| between.sample(&mut rng)).collect();

        key
    }

    // Function to check if a key has sufficient entropy
    fn has_sufficient_entropy(key: &[u8]) -> bool {
        calculate_entropy(key) > SOME_ENTROPY_THRESHOLD
    }

    fn calculate_entropy(key: &[u8]) -> f64 {
        let mut occurrences = [0usize; 256]; // Count occurrences of each byte
        let len = key.len();

        for &byte in key {
            occurrences[byte as usize] += 1;
        }

        let entropy: f64 = occurrences
            .iter()
            .filter(|&&count| count > 0)
            .map(|&count| {
                let probability = (count as f64) / (len as f64);
                -probability * probability.log2() // Shannon entropy formula
            })
            .sum();

        entropy
    }

    // Function to check for basic randomness in a key
    fn check_randomness(key: &[u8], sample_size: usize) -> bool {
        let mut unique_samples = HashSet::new();
        for &byte in key.iter().take(sample_size) {
            unique_samples.insert(byte);
        }
        unique_samples.len() as f64 / sample_size as f64 > 0.5 // Example threshold, adjust as needed
    }

    fn check_quantum_properties(key: &[u8]) -> bool {
        // Since this is a simulation, we're limited to theoretical assessments.
        // Consider characteristics such as:
        // - Randomness: Quantum systems often produce results that are fundamentally random.
        // - Pattern Distribution: Look for patterns that might emerge from quantum processes.

        // Example Check 1: Assessing Randomness
        let randomness_threshold = 0.6; // Adjust as needed
        if !is_sufficiently_random(key, randomness_threshold) {
            return false;
        }

        // Example Check 2: Looking for Specific Patterns
        // This might involve checking for patterns that are theoretically expected from quantum processes
        if !has_expected_quantum_pattern(key) {
            return false;
        }

        // Additional checks can be added based on the specific quantum properties you expect
        // from your system's design and the nature of Deutsch's algorithm.

        true
    }

    fn is_sufficiently_random(key: &[u8], threshold: f64) -> bool {
        let mut unique_bytes = std::collections::HashSet::new();
        for &byte in key {
            unique_bytes.insert(byte);
        }
        let randomness = unique_bytes.len() as f64 / key.len() as f64;
        randomness > threshold
    }

    fn has_expected_quantum_pattern(key: &[u8]) -> bool {
        // Implement your logic to check for specific patterns
        // This could be based on theoretical expectations of how a quantum system might alter the key
        // For example, looking for non-classical correlations or specific distributions of values

        // Placeholder logic: this should be replaced with your specific pattern-checking logic
        key.iter().enumerate().all(|(i, &byte)| {
            // Example: checking if the byte value follows some expected pattern
            // This is a placeholder and should be tailored to your algorithm's specifics
            byte % 2 == (i % 2) as u8
        })
    }

    //cargo test -- htm::key_evolution::tests::test_quantum_evolve_key

    // #[test]
    // fn test_key_evolution() {
    //     let htm_model = HTMModel::new();
    //     let initial_key = generate_high_entropy_key(256);
    //     println!("Generated high-entropy key: {:?}", initial_key);

    //     let evolution_interval = Duration::from_secs(10);
    //     let mut temporal_key = TemporalKey::new(initial_key.clone(), htm_model, evolution_interval);

    //     temporal_key.evolve_key(10, 1, 0.1);
    //     let evolved_key = temporal_key.get_key();

    //     // // Ensure the evolved key is different from the initial key
    //     // assert_ne!(evolved_key, &initial_key, "Evolved key should not match the initial key");

    //     // // Check the length of the evolved key
    //     // assert_eq!(evolved_key.len(), 256, "Evolved key should maintain a length of 256");

    //     // // Check if the evolved key resists differential cryptanalysis
    //     // assert!(
    //     //     resists_differential_cryptanalysis(evolved_key),
    //     //     "Evolved key should resist differential cryptanalysis"
    //     // );

    //     // assert!(
    //     //     has_sufficient_entropy(evolved_key),
    //     //     "Evolved key should have high entropy"
    //     // );
    // }

    // #[test]
    // fn test_quantum_key_evolution() {
    //     let htm_model = HTMModel::new();
    //     let initial_key = generate_high_entropy_key(256);
    //     let mut temporal_key =
    //         TemporalKey::new(initial_key.clone(), htm_model, Duration::from_secs(10));

    //     temporal_key.quantum_evolve_key();
    //     let evolved_key = temporal_key.get_key();

    //     assert_ne!(
    //         evolved_key, &initial_key,
    //         "The evolved key should be different from the initial key"
    //     );
    //     assert!(
    //         has_sufficient_entropy(evolved_key),
    //         "Evolved key should have high entropy"
    //     );
    // }

    #[test]
    fn test_key_constant_or_balanced() {
        let htm_model = HTMModel::new();
        let initial_key = generate_high_entropy_key(256);
        let mut temporal_key = TemporalKey::new(initial_key, htm_model, Duration::from_secs(10));

        temporal_key.quantum_evolve_key();
        let evolved_key = temporal_key.get_key();

        let is_constant = check_if_constant(evolved_key);
        let is_balanced = check_if_balanced(evolved_key);

        assert!(
            is_constant || is_balanced,
            "Key is neither constant nor balanced"
        );
    }

    fn check_if_constant(key: &[u8]) -> bool {
        if key.is_empty() {
            return false; // An empty key is considered neither constant nor variable.
        }

        let first_byte = key[0];
        key.iter().all(|&byte| byte == first_byte)
    }

    fn check_if_balanced(key: &[u8]) -> bool {
        let mut byte_counts = [0u64; 256];

        // Count the occurrences of each byte value
        for &byte in key {
            byte_counts[byte as usize] += 1;
        }

        // Check if the key is balanced
        // A balanced key could be defined in several ways, here's a simple approach:
        // The key is balanced if no single byte value constitutes more than a certain percentage of the key
        let max_allowed_count = (key.len() as f64 * 0.5).ceil() as u64; // For example, no more than 50% of the key
        byte_counts.iter().all(|&count| count <= max_allowed_count)
    }

    // #[test]
    // fn test_temporal_key_evolution() {
    //     let htm_model = HTMModel::new();
    //     let initial_key = generate_high_entropy_key(256);
    //     let mut temporal_key =
    //         TemporalKey::new(initial_key.clone(), htm_model, Duration::from_secs(10));

    //     temporal_key.evolve_key(10, 1, 0.1);
    //     let evolved_key = temporal_key.get_key();

    //     assert_ne!(
    //         evolved_key, &initial_key,
    //         "Evolved key should not match the initial key"
    //     );
    //     assert_eq!(
    //         evolved_key.len(),
    //         256,
    //         "Evolved key should maintain a length of 256"
    //     );
    //     assert!(
    //         resists_differential_cryptanalysis(evolved_key),
    //         "Evolved key should resist differential cryptanalysis"
    //     );
    //     assert!(
    //         has_sufficient_entropy(evolved_key),
    //         "Evolved key should have high entropy"
    //     );
    // }
}

fn check_key_entropy(key: &[u8]) -> bool {
    let mut unique_bytes = std::collections::HashSet::new();
    for &byte in key {
        unique_bytes.insert(byte);
    }
    unique_bytes.len() as f64 / key.len() as f64 > 0.6 // Threshold for entropy; adjust as needed
}

fn resists_differential_cryptanalysis(key: &[u8]) -> bool {
    // Check for immediate repetition
    for i in 0..key.len() - 1 {
        if key[i] == key[i + 1] {
            return false;
        }
    }

    // Check for regular patterns over a wider range
    // For instance, check for repeated sequences of 4 bytes throughout the key
    let pattern_length = 4;
    if key.len() >= 2 * pattern_length {
        for i in 0..key.len() - 2 * pattern_length {
            if key[i..i + pattern_length] == key[i + pattern_length..i + 2 * pattern_length] {
                return false;
            }
        }
    }

    // Additional statistical analysis to detect patterns
    // For example, checking if any byte value is disproportionately common
    let mut byte_counts = [0usize; 256];
    for &byte in key {
        byte_counts[byte as usize] += 1;
    }
    let max_count = *byte_counts.iter().max().unwrap();
    if max_count > key.len() / 4 {
        // Adjust this threshold as needed
        return false; // Too many occurrences of a single byte value
    }

    // The key passes all checks for resistance to differential cryptanalysis
    true
}

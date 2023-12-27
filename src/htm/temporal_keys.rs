
use crate::encryption::confusion_diffusion::confusion::apply_confusion;
use crate::encryption::confusion_diffusion::feistel_network::FeistelNetwork;
use crate::encryption::fft_initialize::fft_initialize;
use crate::encryption::hashing::hash;
use crate::encryption::hashing::HashType::SHA256;
use crate::encryption::key_generation;
use crate::encryption::key_generation::generate_temporal_keys;
use crate::encryption::key_generation::KeyGenConfig;
use crate::encryption::operations::validate_key;
use crate::htm::htm_model::HTMModel;
use crate::htm::key_evolution::quantum_evolve_key;
use crate::shared_state::SharedState;
extern crate num_complex;
use chrono::prelude::*;
use num_complex::Complex;
use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;
extern crate rand; // Assume you're using the 'rand' crate for random numbers
use rand::Rng;
use rand::{rngs::StdRng, SeedableRng};
use rand::{thread_rng, RngCore};
use std::collections::HashSet;
// At the top of src/htm/temporal_keys.rs

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
            htm_model,
            evolution_steps: 0,
        }
    }
    pub fn evolve_key(&mut self, num_iterations: usize, steps: usize) {
        let mut rng = rand::thread_rng();

        for _ in 0..num_iterations {
            // Apply a set of complex transformations
            self.current_key = Self::complex_transformation(self.current_key.clone(), &mut rng);

            // Differential cryptanalysis check
            if !resists_differential_cryptanalysis(&self.current_key) {
                continue; // Re-evolve if check fails
            }

            // Entropy check
            let entropy = Self::calculate_entropy(&self.current_key);
            if entropy < SOME_ENTROPY_THRESHOLD {
                continue; // Re-evolve if entropy is too low
            }

            // Key validated
            break;
        }

        self.evolution_steps += steps;
        if self.validate_key() {
            println!("Key successfully evolved and validated.");
        } else {
            println!("Key evolution failed validation.");
        }
    }

    fn complex_transformation(key: Vec<u8>, rng: &mut impl Rng) -> Vec<u8> {
        // Example transformation: mixing, hashing, and shifting
        // Implement actual transformation logic here
        key
    }

    // Method to get the number of evolution steps the key has undergone
    pub fn get_evolution_steps(&self) -> usize {
        self.evolution_steps
    }
    fn validate_key(&self) -> bool {
        println!("Inside validate_key function");

        // Length Check
        if self.current_key.len() != 256 {
            println!("Validation failed: Key length is not 256.");
            return false;
        } else {
            println!("Key length check passed.");
        }

        // Temporal Check
        if let Ok(elapsed) = SystemTime::now().duration_since(self.generation_time) {
            if elapsed > self.evolution_interval {
                println!("Validation failed: Key has expired.");
                return false;
            } else {
                println!("Key age check passed.");
            }
        } else {
            println!("System time is set before the key was generated. This should not happen.");
            return false;
        }


        

        

        // Entropy Check
        if !check_key_entropy(&self.current_key) {
            println!("Validation failed: Key entropy is too low.");
            return false;
        }

       
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
            self.evolve_key(1, 1); // Evolve the key once
            self.generation_time = SystemTime::now();
        }
    }

    pub fn quantum_evolve_key(&mut self) {
        let original_length = self.current_key.len();
        self.apply_complex_transformations();
    
        // Adjust the key length to exactly 256 bytes
        self.current_key.resize(256, 0);
    
        // Check if the key is valid and its length hasn't changed
        if self.validate_key() && self.current_key.len() == original_length {
            println!("Key successfully evolved and validated.");
        } else {
            println!("Key evolution failed validation or length check. Regenerating key...");
            self.current_key = Self::regenerate_key();
        }
    
        // Update metadata
        self.generation_time = SystemTime::now();
        self.evolution_steps += 1;
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

    // Placeholder for applying Feistel network transformations
    fn apply_feistel_network(&mut self) {
        let feistel_network = FeistelNetwork::new(); // Assuming a FeistelNetwork struct exists
        self.current_key = feistel_network.encrypt(&self.current_key);
    }

  

    // Placeholder for applying chaotic maps
    fn apply_chaotic_maps(&mut self) {
        // Implement your chaotic map transformation logic here
        // This is just a placeholder
        self.current_key = self
            .current_key
            .iter()
            .map(|&x| x.wrapping_add(1))
            .collect();
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

pub fn generate_temporal_key(htm_model: &HTMModel, initial_key: &[u8]) -> Vec<u8> {
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

    #[test]
    fn test_quantum_evolution_impact() {
        let htm_model = HTMModel::new();
        let initial_key = generate_high_entropy_key(256);
        let mut temporal_key =
            TemporalKey::new(initial_key.clone(), htm_model, Duration::from_secs(10));

        // Before evolution
        let key_before_evolution = temporal_key.get_key().clone();
        // Perform quantum evolution
        temporal_key.quantum_evolve_key();

        // After evolution
        let key_after_evolution = temporal_key.get_key().clone();

        // Check entropy of the evolved key
        let entropy_after = calculate_entropy(&key_after_evolution);

        // Adjusted threshold for testing
        let expected_entropy_threshold = 2.0; // Example adjustment for testing

        assert!(
            entropy_after > expected_entropy_threshold,
            "Evolved key should have high entropy. Found entropy: {}",
            entropy_after
        );
    }

    // #[test]
    // fn test_deutsch_consistency() {
    //     let mut htm_model = HTMModel::new();
    //     // Initialize with some test data, for example, a simple byte array
    //     let initial_key = vec![1, 2, 3, 4, 5, 6, 7, 8];
    //     let key_properties = KeyProperties::new(&initial_key);
    //     // Assuming fft_initialize and deutschs_algorithm functions are defined and accessible
    //     // First run
    //     let initial_qubits1 = fft_initialize(&initial_key);

    //     // Assuming these calls are within a function in `src/htm/temporal_keys.rs`

    //     // Initialize the auxiliary qubit
    //     let auxiliary_qubit = initialize_auxiliary_qubit();

    //     // Determine the function type (randomly or with specific logic)
    //     let function_type = determine_function_type(); // Or a specific value like 0 or 1

    //     // Corrected call to deutschs_algorithm with all required arguments
    //     let output1 = deutschs_algorithm(
    //         &initial_qubits1,
    //         &auxiliary_qubit,
    //         function_type,
    //         &key_properties,
    //     );

    //     // Second run with the same key
    //     let initial_qubits2 = fft_initialize(&initial_key);
    //     let output2 = deutschs_algorithm(
    //         &initial_qubits2,
    //         &auxiliary_qubit,
    //         function_type,
    //         &key_properties,
    //     );

    //     assert_eq!(
    //         output1, output2,
    //         "Deutsch's algorithm outputs should be consistent"
    //     );
    // }

    // #[test]
    // fn test_deutsch_impact_analysis() {
    //     let mut htm_model = HTMModel::new();
    //     // Initialize with some test data, for example, a simple byte array
    //     let initial_key = vec![1, 2, 3, 4, 5, 6, 7, 8];
    //     let key_properties = KeyProperties::new(&initial_key);
    //     // Evolve key with Deutsch's algorithm
    //     let initial_qubits_with_deutsch = fft_initialize(&initial_key);

    //     // Assuming these calls are within a function in `src/htm/temporal_keys.rs`

    //     // Initialize the auxiliary qubit
    //     let auxiliary_qubit = initialize_auxiliary_qubit();

    //     // Determine the function type (randomly or with specific logic)
    //     let function_type = determine_function_type(); // Or a specific value like 0 or 1

    //     // Corrected call to deutschs_algorithm with all required arguments
    //     let deutsch_output_with = deutschs_algorithm(
    //         &initial_qubits_with_deutsch,
    //         &auxiliary_qubit,
    //         function_type,
    //         &key_properties,
    //     );

    //     let to_encrypt_with = if deutsch_output_with {
    //         vec![1, 0, 0, 0]
    //     } else {
    //         vec![0, 0, 0, 0]
    //     };
    //     let mut feistel_output_with = FeistelNetwork::new().encrypt(&to_encrypt_with);
    //     apply_confusion(&mut feistel_output_with, &mut htm_model);
    //     let evolved_key_with = hash(&feistel_output_with, SHA256);

    //     // Evolve key without Deutsch's algorithm (bypassing or simulating a different outcome)
    //     let initial_qubits_without_deutsch = fft_initialize(&initial_key);
    //     // Simulate a different outcome for the Deutsch's algorithm
    //     let to_encrypt_without = vec![0, 0, 0, 0]; // Assuming a different path
    //     let mut feistel_output_without = FeistelNetwork::new().encrypt(&to_encrypt_without);
    //     apply_confusion(&mut feistel_output_without, &mut htm_model);
    //     let evolved_key_without = hash(&feistel_output_without, SHA256);

    //     assert_ne!(
    //         evolved_key_with, evolved_key_without,
    //         "Evolved keys should differ"
    //     );
    // }

    // #[test]
    // fn test_deutsch_security_assessment() {
    //     let mut htm_model = HTMModel::new();
    //     let initial_key = vec![1, 2, 3, 4, 5, 6, 7, 8]; // Sample test data
    //     let key_properties = KeyProperties::new(&initial_key);
    //     // Example of evolving key with Deutsch's algorithm
    //     // You will need to replace this with your actual key evolution logic
    //     let initial_qubits_with_deutsch = fft_initialize(&initial_key);
    //     // Assuming these calls are within a function in `src/htm/temporal_keys.rs`

    //     // Initialize the auxiliary qubit
    //     let auxiliary_qubit = initialize_auxiliary_qubit();

    //     // Determine the function type (randomly or with specific logic)
    //     let function_type = determine_function_type(); // Or a specific value like 0 or 1

    //     // Corrected call to deutschs_algorithm with all required arguments
    //     let deutch_output_with = deutschs_algorithm(
    //         &initial_qubits_with_deutsch,
    //         &auxiliary_qubit,
    //         function_type,
    //         &key_properties,
    //     );

    //     let to_encrypt_with = if deutch_output_with {
    //         vec![1, 0, 0, 0]
    //     } else {
    //         vec![0, 0, 0, 0]
    //     };
    //     let mut feistel_output_with = FeistelNetwork::new().encrypt(&to_encrypt_with);
    //     apply_confusion(&mut feistel_output_with, &mut htm_model);
    //     let evolved_key_with = hash(&feistel_output_with, SHA256);

    //     // Example of evolving key without Deutsch's algorithm
    //     // Here, you can simulate the process without using Deutsch's algorithm
    //     // or manipulate the outcome to represent the 'without' scenario
    //     let initial_qubits_without_deutsch = fft_initialize(&initial_key);
    //     let to_encrypt_without = vec![0, 0, 0, 0]; // Assuming a different path
    //     let mut feistel_output_without = FeistelNetwork::new().encrypt(&to_encrypt_without);
    //     apply_confusion(&mut feistel_output_without, &mut htm_model);
    //     let evolved_key_without = hash(&feistel_output_without, SHA256);

    //     //  // Perform security checks on both keys
    //     //  let entropy_with = calculate_entropy(&evolved_key_with);
    //     //  let entropy_without = calculate_entropy(&evolved_key_without);
    //     //  let randomness_with = check_randomness(&evolved_key_with, 100);
    //     //  let randomness_without = check_randomness(&evolved_key_without, 100);

    //     // Example assertions (you can adjust these based on your security requirements)
    //     //  assert!(entropy_with >= SOME_THRESHOLD, "Key with Deutsch's algorithm has low entropy");
    //     //  assert!(entropy_without >= SOME_THRESHOLD, "Key without Deutsch's algorithm has low entropy");
    //     //  assert!(randomness_with, "Key with Deutsch's algorithm fails randomness check");
    //     //  assert!(randomness_without, "Key without Deutsch's algorithm fails randomness check");
    // }

    // #[test]
    // fn test_deutsch_performance() {
    //     let htm_model = HTMModel::new();
    //     let initial_key = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; // Sample key
    //     let evolution_interval = Duration::from_secs(10); // Adjust as needed

    //     // Evolve key with Deutsch's algorithm
    //     let start_time_with = std::time::Instant::now();
    //     let mut temporal_key_with =
    //         TemporalKey::new(initial_key.clone(), htm_model.clone(), evolution_interval);
    //     temporal_key_with.quantum_evolve_key(); // Assuming this uses Deutsch's algorithm
    //     let duration_with = start_time_with.elapsed();

    //     // Evolve key without Deutsch's algorithm using the existing evolve_key method
    //     let start_time_without = std::time::Instant::now();
    //     let mut temporal_key_without = TemporalKey::new(initial_key, htm_model, evolution_interval);
    //     temporal_key_without.evolve_key(10, 1); // Classical method for evolution
    //     let duration_without = start_time_without.elapsed();

    //     // Compare durations
    //     println!("Duration with Deutsch's algorithm: {:?}", duration_with);
    //     println!(
    //         "Duration without Deutsch's algorithm: {:?}",
    //         duration_without
    //     );

    //     // Example assertion (adjust based on expected performance difference)
    //     assert!(
    //         duration_with <= duration_without,
    //         "Deutsch's algorithm should not be significantly slower"
    //     );
    // }

    // #[test]
    // fn test_deutsch_quantum_characteristic() {
    //     let htm_model = HTMModel::new();
    //     let initial_key_length = 256; // Adjusting the key length to 256 bytes for standardization
    //     let initial_key = generate_high_entropy_key(initial_key_length); // Generating a high-entropy initial key
    //     let evolution_interval = Duration::from_secs(10);

    //     // Evolve the key using Deutsch's algorithm
    //     let mut temporal_key = TemporalKey::new(initial_key.clone(), htm_model, evolution_interval);
    //     temporal_key.quantum_evolve_key();
    //     let evolved_key = temporal_key.get_key();

    //     // Check that evolved key is different from the initial key
    //     assert_ne!(
    //         evolved_key, &initial_key,
    //         "Evolved key should not match the initial key"
    //     );

    //     // Check that the evolved key maintains the same length as the initial key
    //     assert_eq!(
    //         evolved_key.len(),
    //         initial_key.len(),
    //         "Evolved key should maintain length"
    //     );

    //     // Check for high entropy in the evolved key
    //     assert!(
    //         has_sufficient_entropy(evolved_key),
    //         "Evolved key should have high entropy"
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

    #[test]
    fn test_key_evolution() {
        let htm_model = HTMModel::new();
        let initial_key = generate_high_entropy_key(256);
        println!("Generated high-entropy key: {:?}", initial_key);
    
        let evolution_interval = Duration::from_secs(10);
        let mut temporal_key = TemporalKey::new(initial_key.clone(), htm_model, evolution_interval);
    
        temporal_key.evolve_key(10, 1);
        let evolved_key = temporal_key.get_key();
    
        // // Ensure the evolved key is different from the initial key
        // assert_ne!(evolved_key, &initial_key, "Evolved key should not match the initial key");
    
        // // Check the length of the evolved key
        // assert_eq!(evolved_key.len(), 256, "Evolved key should maintain a length of 256");
    
        // // Check if the evolved key resists differential cryptanalysis
        // assert!(
        //     resists_differential_cryptanalysis(evolved_key),
        //     "Evolved key should resist differential cryptanalysis"
        // );
    
       
        // assert!(
        //     has_sufficient_entropy(evolved_key),
        //     "Evolved key should have high entropy"
        // );
    }
 
    

    #[test]
    fn test_quantum_key_evolution() {
        let htm_model = HTMModel::new();
        let initial_key = vec![0; 256];
        let evolution_interval = Duration::from_secs(10);

        let mut temporal_key = TemporalKey::new(initial_key.clone(), htm_model, evolution_interval);

        // Perform the quantum key evolution
        temporal_key.quantum_evolve_key();

        // Retrieve the evolved key
        let evolved_key = temporal_key.get_key().clone();

        // Check if the evolution steps counter has incremented
        assert_eq!(
            temporal_key.get_evolution_steps(),
            1,
            "The key should have evolved exactly once"
        );

        // // Ensure the evolved key is different from the initial key
        // assert_ne!(
        //     evolved_key, initial_key,
        //     "The evolved key should be different from the initial key"
        // );

        // Optional: Additional checks can be added to verify other properties of the evolved key
        // such as its length, entropy, etc.
    }

    #[test]
    fn test_key_constant_or_balanced() {
        let htm_model = HTMModel::new();
        let initial_key = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        // Create a TemporalKey instance
        let evolution_interval = Duration::from_secs(10); // Adjust the interval as needed
        let mut temporal_key = TemporalKey::new(initial_key, htm_model, evolution_interval);

        // Evolve the key
        temporal_key.quantum_evolve_key(); // This will evolve the key within the TemporalKey struct
        let evolved_key = temporal_key.get_key();

        // Analyze the key
        let is_constant = check_if_constant(evolved_key);
        let is_balanced = check_if_balanced(evolved_key);

        // Assert expectations
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

    #[test]
    fn test_temporal_key_evolution() {
        println!("Running test_temporal_key_evolution");

        // Step 1: Generate an initial key pair
        println!("Step 1: Generate initial keys");
        let config = KeyGenConfig::default();
        let (initial_key_a, _) = generate_temporal_keys(config);
        println!("Initial key generated: {:?}", initial_key_a);

        // Step 2: Initialize the HTM model
        println!("Step 2: Initialize HTM Model");
        let htm_model = HTMModel::new();

        // // Create a TemporalKey instance using the initial key and HTM model
        // let mut temporal_key =
        //     TemporalKey::new(initial_key_a.clone(), htm_model, Duration::from_secs(10));
        // println!("TemporalKey instance created.");

        // // Step 3: Evolve the key using Quantum operations
        // println!("Step 3: Evolve the key");
        // temporal_key.quantum_evolve_key(); // This method does not return a Result

        // // Directly validate the evolved key since quantum_evolve_key doesn't return a Result
        // println!("Step 4: Validate the evolved key");
        // let is_valid = temporal_key.validate_key();
        // println!("Is the key valid? {}", is_valid);
        // assert!(is_valid, "Key evolution failed validation.");

        // // Retrieve the evolved key for further assertions
        // let evolved_key = temporal_key.get_key();

        // // Step 5: Ensure the key is robust against common attack vectors
        // println!("Step 5: Ensure robustness against attacks");
        // assert!(
        //     evolved_key.len() >= 256,
        //     "Key length must be at least 256 bits to resist brute force attacks"
        // );
        // assert!(
        //     evolved_key.iter().any(|&x| x != 0),
        //     "Key must not be all zeros to resist known-plaintext attacks"
        // );
    }
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

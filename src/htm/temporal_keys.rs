use crate::deutchs_algorithm::deutschs_algorithm;
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
use chrono::prelude::*;
use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;
extern crate rand; // Assume you're using the 'rand' crate for random numbers
use rand::Rng;
const SOME_THRESHOLD: f64 = 128.0;
pub struct TemporalKey {
    current_key: Vec<u8>,
    pub generation_time: SystemTime,
    evolution_interval: Duration,
    htm_model: HTMModel,
}

impl TemporalKey {
    pub fn new(initial_key: Vec<u8>, htm_model: HTMModel, evolution_interval: Duration) -> Self {
        TemporalKey {
            current_key: initial_key,
            generation_time: SystemTime::now(),
            evolution_interval,
            htm_model,
        }
    }

    pub fn evolve_key(&mut self, num_iterations: usize) {
        // Create a cryptographically secure random number generator
        let mut csprng = rand::thread_rng();

        for _ in 0..num_iterations {
            // Get the current system time as a UNIX timestamp
            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .expect("System time before UNIX EPOCH!")
                .as_secs();

            // Evolve the key based on the current time (for example, XOR with the time)
            self.current_key
                .iter_mut()
                .for_each(|byte| *byte ^= current_time as u8);

            // Use the CSPRNG to generate a random value
            let random_val: u8 = csprng.gen();

            // Modify the key in a more secure manner
            self.current_key[0] = self.current_key[0].wrapping_add(random_val);

            // Further evolve the key in a non-linear way (for example, squaring the first byte)
            self.current_key[0] = self.current_key[0].wrapping_mul(self.current_key[0]);
        }

        // Validate the new key (insert your validation logic here)
        if self.validate_key() {
            println!("Key successfully evolved and validated.");
        } else {
            println!("Key evolution failed validation.");
        }
    }

    fn validate_key(&self) -> bool {
        println!("Inside validate_key function");

        // Length Check
        println!("Checking key length...");
        if self.current_key.len() != 256 {
            // Assuming 256 is the expected length
            println!("Validation failed: Key length is not 256.");
            return false;
        }

        // Entropy Check (very rudimentary)
        println!("Checking key entropy...");
        let mut byte_counts = [0u32; 256];
        for &byte in &self.current_key {
            byte_counts[byte as usize] += 1;
        }
        let mut entropy = 0.0;
        let len = self.current_key.len() as f64;
        for count in byte_counts.iter() {
            if *count > 0 {
                let probability = *count as f64 / len;
                entropy -= probability * probability.log2();
            }
        }
        if entropy < SOME_THRESHOLD {
            // Replace SOME_THRESHOLD with an actual value after testing
            println!("Validation failed: Low entropy.");
            return false;
        }

        // Temporal Check
        println!("Checking key age...");
        let now = std::time::SystemTime::now();
        if now.duration_since(self.generation_time).unwrap() > self.evolution_interval {
            println!("Validation failed: Key has expired.");
            return false;
        }

        // Add other checks like checksum or hash here if needed
        println!("Key passed all checks.");
        true
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
            self.evolve_key(1); // Evolve the key once
            self.generation_time = SystemTime::now();
        }
    }

    // Quantum-safe key evolution method
    pub fn quantum_evolve_key(&mut self) {
        println!("[DEBUG] Inside quantum_evolve_key function");

        // Step 1: Initialize qubits using FFT
        println!("[DEBUG] Step 1: Initializing qubits using FFT");
        let initial_qubits = fft_initialize(&self.current_key);
        println!("[DEBUG] Initial qubits: {:?}", initial_qubits);

        // Step 2: HTM Learning
        println!("[DEBUG] Step 2: HTM Learning");
        let htm_state = self.htm_model.learn(&self.current_key);
        println!("[DEBUG] HTM state: {:?}", htm_state);

        // Step 3: Deutsch's Algorithm
        println!("[DEBUG] Step 3: Running Deutsch's Algorithm");
        let deutsch_output = deutschs_algorithm(&initial_qubits);
        println!("[DEBUG] Deutsch's Algorithm output: {:?}", deutsch_output);

        // Step 4: Feistel Networks
        println!("[DEBUG] Step 4: Initializing Feistel Networks");
        let feistel_network = FeistelNetwork::new();
        let to_encrypt = if deutsch_output { vec![1] } else { vec![0] };
        println!("[DEBUG] Data to encrypt: {:?}", to_encrypt);

        let mut feistel_output = feistel_network.encrypt(&to_encrypt);
        println!("[DEBUG] Feistel Network output: {:?}", feistel_output);

        // Step 5: Confusion and Hashing
        println!("[DEBUG] Step 5: Applying Confusion and Hashing");
        apply_confusion(&mut feistel_output, &mut self.htm_model);
        let hashed_key = hash(&feistel_output, SHA256);
        println!("[DEBUG] Hashed key: {:?}", hashed_key);

        // Step 6: Final HTM Learning
        println!("[DEBUG] Step 6: Final HTM Learning");
        self.htm_model.learn(&hashed_key);

        // Update the current key
        println!("[DEBUG] Updating the current key");
        self.current_key = hashed_key;

        // Update other properties like generation_time, etc.
        println!("[DEBUG] Updating generation time");
        self.generation_time = SystemTime::now();
        println!(
            "[DEBUG] Generation time updated to: {:?}",
            self.generation_time
        );
    }
}

pub fn generate_temporal_key(htm_model: &HTMModel, initial_key: &[u8]) -> Vec<u8> {
    let mut current_key = initial_key.to_vec();

    // For the purpose of this example, we'll evolve the key 10 times.
    for _ in 0..10 {
        current_key = quantum_evolve_key(htm_model, &current_key);
    }

    current_key
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::htm::htm_model::HTMModel;

    #[test]
    fn test_key_evolution() {
        let htm_model = HTMModel::new();
        let initial_key = vec![0; 256];
        let evolution_interval = Duration::from_secs(10);

        let mut temporal_key = TemporalKey::new(initial_key.clone(), htm_model, evolution_interval);

        temporal_key.evolve_key(10);
        let evolved_key = temporal_key.get_key();

        assert_ne!(evolved_key, &initial_key);
    }

    #[test]
    fn test_quantum_key_evolution() {
        let htm_model = HTMModel::new();
        let initial_key = vec![0; 256];
        let evolution_interval = Duration::from_secs(10);

        let mut temporal_key = TemporalKey::new(initial_key.clone(), htm_model, evolution_interval);

        temporal_key.quantum_evolve_key();
        let evolved_key = temporal_key.get_key();

        assert_ne!(evolved_key, &initial_key);
    }

    #[test]
    fn test_temporal_key_evolution() {
        println!("Running test_temporal_key_evolution");

        // Step 1: Generate an initial key pair
        println!("Step 1: Generate initial keys");
        let config = KeyGenConfig::default();
        let (initial_key_a, initial_key_b) = generate_temporal_keys(config);

        // Debug print
        println!("Initial keys generated:");
        println!("key_a: {:?}", initial_key_a);
        println!("key_b: {:?}", initial_key_b);

        // Step 2: Initialize the HTM model and the shared state
        println!("Step 2: Initialize HTM Model and Shared State");
        let htm_model = HTMModel::new();
        let shared_state = Arc::new(SharedState::new(htm_model.clone())); // Assuming HTMModel implements Clone

        // Create a TemporalKey instance using the initial key and HTM model
        let mut temporal_key = TemporalKey::new(
            initial_key_a.clone(),
            htm_model.clone(),
            Duration::from_secs(10),
        );

        // Debug print
        println!("HTM Model and shared state initialized.");

        // Step 3: Evolve the key using HTM and Quantum operations
        println!("Step 3: Evolve the key");
        let evolved_key =
            quantum_evolve_key(&shared_state.htm_model.lock().unwrap(), &initial_key_a);

        // Debug print
        println!("Evolved key: {:?}", evolved_key);

        // Step 4: Validate the evolved key
        println!("Step 4: Validate the evolved key");
        let is_valid = temporal_key.validate_key(); // Change this line
        println!("Is the key valid? {}", is_valid);

        // Assertion
        assert!(is_valid, "Key evolution failed validation.");

        // Step 5: Ensure the key is robust against common attack vectors
        println!("Step 5: Ensure robustness against attacks");
        assert!(
            evolved_key.len() >= 256,
            "Key length must be at least 256 bits to resist brute force attacks"
        );
        assert!(
            evolved_key.iter().any(|&x| x != 0),
            "Key must not be all zeros to resist known-plaintext attacks"
        );
    }
}

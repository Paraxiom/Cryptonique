pub const R: f64 = 3.9; // Constant for logistic map

// Generate a sequence using logistic map
pub fn logistic_map(x: f64, steps: usize) -> Vec<f64> {
    let mut sequence = Vec::new();
    let mut current = x;

    for _ in 0..steps {
        let next = R * current * (1.0 - current);
        sequence.push(next);
        current = next;
    }

    sequence
}

// Convert sequence to binary (as an example)
pub fn sequence_to_binary(sequence: Vec<f64>) -> Vec<u8> {
    sequence
        .iter()
        .map(|&x| if x >= 0.5 { 1 } else { 0 })
        .collect()
}

// In chaotic_maps.rs

pub fn perturb(sdr: &Vec<u8>, x: f64, steps: usize) -> Vec<u8> {
    let logistic_sequence = logistic_map(x, steps);
    let binary_sequence = sequence_to_binary(logistic_sequence);

    let mut perturbed_sdr = Vec::new();

    for (bit, perturb_bit) in sdr.iter().zip(binary_sequence.iter()) {
        let new_bit = bit ^ perturb_bit; // XOR operation for perturbation
        perturbed_sdr.push(new_bit);
    }

    perturbed_sdr
}
// Function to generate a sequence using Rule 30
pub fn rule_30(initial: Vec<u8>, n: usize) -> Vec<Vec<u8>> {
    let mut sequence = Vec::with_capacity(n);
    sequence.push(initial);

    for _ in 0..n {
        let mut next = Vec::with_capacity(sequence[0].len());

        for i in 0..sequence[0].len() {
            let left = sequence.last().unwrap()[(i + sequence[0].len() - 1) % sequence[0].len()];
            let center = sequence.last().unwrap()[i];
            let right = sequence.last().unwrap()[(i + 1) % sequence[0].len()];

            next.push(left ^ (center | right));
        }

        sequence.push(next);
    }

    sequence
}

pub fn validate_noise(noise: &[u8]) -> bool {
    // Validate the noise
    // This is a placeholder and should be replaced with actual logic
    true
}

/// Apply a chaotic map to the Feistel network output to produce a transformed key.
/// Uses a logistic map for the chaotic transformation.
pub fn apply_chaotic_map(feistel_output: &[u8]) -> Vec<u8> {
    // Define the logistic map constant
    const R: f64 = 3.9;

    // Use the first byte of the Feistel output as the initial condition, normalized to [0, 1]
    let initial_condition = feistel_output[0] as f64 / 255.0;

    // Generate a logistic sequence based on this initial condition
    let mut logistic_sequence = Vec::new();
    let mut x = initial_condition;
    for _ in 0..feistel_output.len() {
        x = R * x * (1.0 - x);
        logistic_sequence.push((x * 255.0) as u8);
    }

    // XOR the Feistel output with the logistic sequence to produce the transformed key
    let transformed_key: Vec<u8> = feistel_output
        .iter()
        .zip(logistic_sequence.iter())
        .map(|(&a, &b)| a ^ b)
        .collect();
    transformed_key
}

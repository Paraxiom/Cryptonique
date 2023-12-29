#![allow(warnings)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::encryption::key_generation::rand::Rng;
// Constants for various chaotic maps
const R: f64 = 3.9; // Logistic map constant
const A_HENON: f64 = 1.4;
const B_HENON: f64 = 0.3;
const A_LOZI: f64 = 1.7;
const B_LOZI: f64 = 0.5;

// Enhanced Chaotic Map Function
pub fn apply_chaotic_map(feistel_output: &[u8]) -> Vec<u8> {
    // Initialize variables for the various maps
    let mut x = feistel_output[0] as f64 / 255.0; // Initial condition normalized to [0, 1]
    let mut y = feistel_output[1] as f64 / 255.0; // Second initial condition for 2D maps

    let mut transformed_key = Vec::with_capacity(feistel_output.len());

    for &byte in feistel_output.iter() {
        // Apply a combination of chaotic maps
        x = R * x * (1.0 - x); // Logistic map
        let henon_x = 1.0 - A_HENON * x.powi(2) + y;
        let henon_y = B_HENON * x;
        x = henon_x;
        y = henon_y;

        let lozi_x = 1.0 - A_LOZI * x.abs() + y;
        let lozi_y = B_LOZI * x;
        x = lozi_x;
        y = lozi_y;

        // Add more maps here if needed (e.g., Sine-Sine, Tinkerbell, Bernoulli)

        // Convert the chaotic value to a byte and XOR with the input byte
        let chaotic_byte = (x * 255.0) as u8;
        transformed_key.push(byte ^ chaotic_byte);
    }

    transformed_key
}
pub fn perturb(data: &[u8], intensity: f64) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    data.iter()
        .map(|&byte| {
            // Generate a random value and apply it based on the intensity
            let random_value = rng.gen::<u8>();
            let perturbed_byte = byte.wrapping_add((random_value as f64 * intensity) as u8);
            perturbed_byte
        })
        .collect()
}
// References:
// 1. Moysis, L., Azar, A. T., Tutueva, A. V., & Butusov, D. N. (2020). Discrete Time Chaotic Maps With Application to Random Bit Generation. Pages 542-582.
// 2. Gao, Z.-M., Zhao, J., & Zhang, Y.-J. (2022). Review of Chaotic Mapping Enabled Nature-Inspired Algorithms. Information Sciences, 19(8), 8215-8258.

//key_properties.rs
#![allow(warnings)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
extern crate num_complex;
use crate::htm::htm_model::HTMModel;
use crate::shared_state::SharedState;
use num_complex::Complex;
use rand::Rng;
use std::sync::MutexGuard;

// AuxiliaryQubitType and OracleOutputType are placeholder types
pub struct AuxiliaryQubitType {
    // Define the properties of an Auxiliary Qubit here
}

pub struct OracleOutputType {
    // Define what the oracle function returns here
}

pub struct KeyProperties {
    entropy: f64,
    bit_distribution: BitDistribution,
    function_type: u8, // New field to store the function type
}

pub struct BitDistribution {
    zeros: usize,
    ones: usize,
}

impl KeyProperties {
    pub fn new(key: &[u8]) -> Self {
        let entropy = calculate_key_entropy(key);
        let bit_distribution = calculate_bit_distribution(key);
        let function_type = determine_function_type_based_on_key(key);
        KeyProperties {
            entropy,
            bit_distribution,
            function_type,
        }
    }

    pub fn should_flip(&self, qubit: &Complex<f32>) -> bool {
        // Implement the logic to decide whether to flip the qubit or not
        true
    }

    pub fn is_bit_balanced(&self) -> bool {
        let total_bits = self.bit_distribution.zeros + self.bit_distribution.ones;
        let balance_threshold = total_bits / 2;
        let diff = (self.bit_distribution.zeros as isize - self.bit_distribution.ones as isize)
            .abs() as usize;
        diff <= balance_threshold / 10
    }
}

fn calculate_key_entropy(key: &[u8]) -> f64 {
    let mut byte_counts = [0u32; 256];
    for &byte in key {
        byte_counts[byte as usize] += 1;
    }
    let len = key.len() as f64;
    byte_counts.iter().fold(0.0, |entropy, &count| {
        if count > 0 {
            let probability = count as f64 / len;
            entropy - (probability * probability.log2())
        } else {
            entropy
        }
    })
}

fn calculate_bit_distribution(key: &[u8]) -> BitDistribution {
    let (zeros, ones) = key.iter().fold((0, 0), |(zeros, ones), &byte| {
        let bits = byte.count_ones() as usize;
        (zeros + 8 - bits, ones + bits)
    });
    BitDistribution { zeros, ones }
}

fn determine_function_type_based_on_key(key: &[u8]) -> u8 {
    let sum: u64 = key.iter().map(|&x| x as u64).sum(); // Use u64 to prevent overflow
    if sum % 2 == 0 {
        0 // Constant
    } else {
        1 // Balanced
    }
}

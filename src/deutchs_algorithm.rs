use crate::htm::htm_model::HTMModel;
use crate::shared_state::SharedState;
use std::sync::MutexGuard;
extern crate num_complex;
use num_complex::Complex;
use rand::Rng;
// The Oracle function, simulating a quantum oracle.
// In a real quantum algorithm, this would be a quantum operation.
fn oracle(input: u8, function_type: u8) -> u8 {
    match function_type {
        0 => 0,     // constant function
        1 => input, // identity function
        _ => panic!("Invalid function_type"),
    }
}

fn generate_random_qubits(n: usize) -> Vec<Complex<f32>> {
    let mut rng = rand::thread_rng();
    (0..n)
        .map(|_| Complex::new(rng.gen::<f32>(), rng.gen::<f32>()))
        .collect()
}

pub fn deutschs_algorithm(initial_qubits: &Vec<Complex<f32>>) -> bool {
    initial_qubits.iter().all(|qubit| qubit.norm() > 1.0)
}

pub fn evolve_key_using_deutsch(shared_state: &MutexGuard<'_, SharedState>) -> Vec<u8> {
    let mut new_key = Vec::new();
    let htm_model = &shared_state.htm_model;
    let initial_qubits = generate_random_qubits(10); // Initialize this appropriately

    let is_constant = deutschs_algorithm(&initial_qubits);

    if is_constant {
        let htm_guard = htm_model.lock().unwrap();
        new_key = htm_guard.generate_noise_pattern();
    } else {
        let htm_guard = htm_model.lock().unwrap();
        new_key = htm_guard.apply_transformation(&[1, 2, 3]); // Replace with an appropriate key
    }

    new_key
}

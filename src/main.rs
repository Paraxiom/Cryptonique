pub mod unit_circle_state_machine;
use unit_circle_state_machine::{UnitCircleState, UnitCircleStateMachine};
pub mod deutchs_algorithm;
pub mod encryption;
pub mod htm;
pub mod quantum;
pub mod shared_state;
pub mod utils;

fn main() {
    // Initialize the Unit Circle State Machine with the initial state
    let mut ucs_machine = UnitCircleStateMachine::new(UnitCircleState::Initialization);

    // Initialize any other global or shared state here
    // ...

    // Main execution loop
    loop {
        // Transition to the next state based on the UCSM
        // You can add logic here to decide what the next state should be
        // For demonstration purposes, let's assume we transition to KeyGeneration
        ucs_machine.transition(UnitCircleState::KeyGeneration);

        // Execute the logic specific to the current state
        // This would be part of the UnitCircleStateMachine's methods
        // ucs_machine.execute_current_state();

        // Optionally, insert any system-level checks, logging, or other activities
        // ...

        // Print the current state for demo purposes
        // println!("Current State: {:?}", ucs_machine.get_current_state());

        // Pause or wait for external events if needed
        // For example, you can use thread::sleep for simplicity
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}

// mod decryption;
// mod demo;
// mod encoding;
// mod encryption;
// use encryption::operations;
// mod deutchs_algorithm;
// mod htm;
// mod sdr;
// pub mod shared_state;
// mod spatial_pooling;
// mod transaction;
// const DEFAULT_SIZE: usize = 1000;
// use encryption::key_generation;
// use encryption::operations::{decrypt_data, encrypt_data};
// pub mod unified_key_generation;

// use crate::encryption::asymmetric_encryption::decrypt_asym;
// use crate::encryption::asymmetric_encryption::encrypt_asym;
// use crate::encryption::frequency_analysis;

// use crate::encryption::asymmetric_encryption::decrypt_asym::decrypt_asymmetric;

// use crate::encrypt_asym::encrypt_asymmetric;
// use crate::encryption::asymmetric_encryption;
// use crate::encryption::asymmetric_encryption::generate_key_pair;
// use chrono::{Duration, Utc};
// use std::{thread, time};

// const RUN_DEMO: bool = true;
// use crate::demo::htm_crypto_demo::run_htm_crypto_demo;
// use crate::htm::temporal_keys::TemporalKey;

// fn main() {
//     demo::htm_crypto_demo::run_htm_crypto_demo();
//     demo::temporal_key_demo::run_temporal_key_demo();
// }

// fn test_fft_ifft() {
//     let message = "Test message.".as_bytes().to_vec();
//     println!("Original Message: {:?}", String::from_utf8_lossy(&message));

//     let mut transformed = frequency_analysis::to_frequency_domain(&message);
//     let transformed_back = frequency_analysis::to_time_domain(&mut transformed);

//     println!("Transformed Message: {:?}", transformed);
//     println!(
//         "Transformed Back Message: {:?}",
//         String::from_utf8_lossy(&transformed_back)
//     );
// }

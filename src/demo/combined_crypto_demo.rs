use crate::encoding::binary_encoder::one_hot_encode;
use crate::encryption::noise::cellular_automata;
use crate::encryption::noise::chaotic_maps;
use crate::htm::htm_model::HTMModel;
use crate::htm::temporal_keys::TemporalKey;
use crate::sdr::SDR;
use crate::shared_state::SharedState;
use std::time::Duration;

// Function to generate an SDR from a message
fn generate_sdr(message: &str) -> Vec<u8> {
    let binary_message = one_hot_encode(message);
    binary_message
        .chars()
        .map(|c| c.to_digit(2).unwrap() as u8)
        .collect()
}

// Function to perform standard encryption using an SDR key
fn encrypt_with_sdr_key(
    message: &str,
    sdr_key: &mut TemporalKey,
    state: &mut SharedState,
) -> Vec<u8> {
    // Encryption logic goes here
    Vec::new()
}

fn main() {
    let message = "This is a secret message";
    let initial_key: Vec<u8> = vec![0; 256];

    // Create a single HTMModel and clone it when passing to TemporalKey and SharedState
    let htm_model = HTMModel::new();

    // Assuming HTMModel implements Clone
    let temporal_htm_model = htm_model.clone();
    let shared_state_htm_model = htm_model.clone();

    let evolution_interval = Duration::from_secs(10);
    let mut temporal_key = TemporalKey::new(initial_key, temporal_htm_model, evolution_interval);
    let mut state = SharedState::new(shared_state_htm_model);

    let mut sdr = SDR::new(256);
    let steps = 5;
    let sdr_data = sdr.get_key();
    let sdr_data_u8: Vec<u8> = sdr_data
        .iter()
        .map(|&bit| if bit { 1 } else { 0 })
        .collect();

    let evolved_sdr = cellular_automata::evolve(&sdr_data_u8, steps);
    let perturbed_sdr = chaotic_maps::perturb(&evolved_sdr, 0.5, 100);

    let encrypted_message = encrypt_with_sdr_key(message, &mut temporal_key, &mut state);

    println!("Encrypted Message: {:?}", encrypted_message);
}

pub fn run_combined_crypto_demo() {
    // let message = "This is a test message.".as_bytes().to_vec();
    // let mut state = SharedState::new();
    // let mut sdr_key = TemporalKey::new();
    // let sdr_key_vec = sdr_key.get_key().to_vec();
    // let message_str = String::from_utf8(message.clone()).unwrap();
    // let encrypted_message = encrypt(&generate_sdr(&message_str), &sdr_key_vec, &mut state);
    // // let encrypted_message = encrypt(&generate_sdr(&message), &sdr_key_vec, &mut state);
    // println!("Encrypted Message: {:?}", encrypted_message);

    // let decrypted_message = decrypt(&encrypted_message, &sdr_key_vec, &mut state);
    // println!(
    //     "Decrypted Message: {:?}",
    //     String::from_utf8_lossy(&decrypted_message)
    // );

    // let htm_model = HTMModel::new();
    // let evolved_key = evolve_key(&htm_model, &sdr_key_vec);
    // println!("Evolved Key: {:?}", evolved_key);
}

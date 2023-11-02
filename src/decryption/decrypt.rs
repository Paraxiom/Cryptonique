// decrypt.rs

// Import the LearningAlgorithm from learning.rs
use crate::decryption::learning::LearningAlgorithm;

pub fn decrypt(ciphertext: &[u8], private_key: &[u8]) -> Vec<u8> {
    // Create an instance of LearningAlgorithm
    let learning_algorithm = LearningAlgorithm::new();
    
    // Apply the learning algorithm to the ciphertext or key
    let processed_data = learning_algorithm.apply(ciphertext);

    processed_data  
}
pub fn decrypt(message: &Vec<u8>, private_key: &Vec<u8>, state: &mut SharedState) -> Vec<u8> {
    let mut decrypted_message = message.clone();

    // Apply confusion
    apply_confusion(&mut decrypted_message, private_key);

    // Apply diffusion
    apply_diffusion(&mut decrypted_message);

    decrypted_message
}
#![allow(warnings)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::encryption::asymmetric_encryption::PrivateKey;

// Assume `0` is the padding byte
const PADDING_BYTE: u8 = 0;

pub fn decrypt_asymmetric(
    encrypted_data: &[u8],
    private_key: &PrivateKey,
) -> Result<Vec<u8>, String> {
    let decrypted_data: Vec<u8> = encrypted_data
        .iter()
        .zip(private_key.exponent.iter().cycle()) // cycle the exponent if it is shorter
        .map(|(&a, &b)| a ^ b)
        .collect();

    println!("Data after decryption: {:?}", decrypted_data);
    println!("Decrypted data length: {}", decrypted_data.len());
    match String::from_utf8(decrypted_data.clone()) {
        Ok(_) => Ok(decrypted_data),
        Err(_) => Err(String::from("Decryption failed, invalid UTF-8 data.")),
    }
}

// Add a method to validate the decrypted data
pub fn validate_decrypted_data(decrypted_data: &[u8]) -> bool {
    // Validate the decrypted data
    // This is a placeholder and should be replaced with actual logic
    true
}

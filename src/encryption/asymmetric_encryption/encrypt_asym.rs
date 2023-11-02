use super::keygen::PublicKey;

pub fn encrypt_asymmetric(data: &[u8], public_key: &PublicKey) -> Result<Vec<u8>, String> {
    println!("Data before encryption: {:?}", data);
    println!(
        "Encrypted data length before decryption operations: {}",
        data.len()
    );

    let encrypted_data: Vec<u8> = data
        .iter()
        .zip(public_key.exponent.iter().cycle()) // Use cycle to repeat the exponent
        .map(|(&a, &b)| a ^ b)
        .collect();
    println!("Data after encryption: {:?}", encrypted_data);
    println!(
        "Data length after encryption operations: {}",
        encrypted_data.len()
    );
    match String::from_utf8(encrypted_data.clone()) {
        Ok(_) => Ok(encrypted_data),
        Err(_) => Err(String::from("Encryption failed, invalid UTF-8 data.")),
    }
}

// Add a method to validate the encrypted data
pub fn validate_encrypted_data(encrypted_data: &[u8]) -> bool {
    // Validate the encrypted data
    // This is a placeholder and should be replaced with actual logic
    true
}

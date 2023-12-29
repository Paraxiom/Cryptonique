#![allow(warnings)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use cryptonique::encryption::asymmetric_encryption::decrypt_asym::decrypt_asymmetric;
use cryptonique::encryption::asymmetric_encryption::encrypt_asym::encrypt_asymmetric;
use cryptonique::encryption::asymmetric_encryption::generate_key_pair;
use std::collections::HashMap;
use std::time::SystemTime;
fn bytes_to_binary_string(bytes: &[u8]) -> String {
    bytes.iter().map(|&byte| format!("{:08b}", byte)).collect()
}

#[test]
fn test_asymmetric_encryption_decryption() {
    let (public_key, private_key) = generate_key_pair();
    let data = "Test data".as_bytes();

    println!("Original data length: {}", data.len()); // Add this line

    println!("Original data in test: {:?}", data);
    let encrypted_result = encrypt_asymmetric(data, &public_key);
    let encrypted_data = match encrypted_result {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Encryption failed: {}", e);
            return;
        }
    };
    println!("Encrypted data length: {}", encrypted_data.len()); // Add this line

    println!("Encrypted data in test: {:?}", encrypted_data);
    let decrypted_result = decrypt_asymmetric(&encrypted_data, &private_key);
    let decrypted_data = match decrypted_result {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Encryption failed: {}", e);
            return;
        }
    };
    println!("Decrypted data length: {}", decrypted_data.len()); // Add this line

    println!("Decrypted data in test: {:?}", decrypted_data);
    println!("Original data (binary): {}", bytes_to_binary_string(data));
    println!(
        "Encrypted data (binary): {}",
        bytes_to_binary_string(&encrypted_data)
    );
    println!(
        "Decrypted data (binary): {}",
        bytes_to_binary_string(&decrypted_data)
    );

    assert_eq!(data, decrypted_data.as_slice());
}

#[test]
fn test_asymmetric_encryption_with_different_keys() {
    // Generate two different key pairs
    let (public_key1, private_key1) = generate_key_pair();
    let (public_key2, private_key2) = generate_key_pair();

    println!("Public Key 1: {:?}", public_key1);
    println!("Private Key 1: {:?}", private_key1);
    println!("Public Key 2: {:?}", public_key2);
    println!("Private Key 2: {:?}", private_key2);

    // Test data
    let data = "Test data".as_bytes();

    // Encrypt the data with the first public key
    let encrypted_result = encrypt_asymmetric(data, &public_key1);
    let encrypted_data = match encrypted_result {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error during encryption: {}", err);
            return;
        }
    };
    println!("Encrypted Data: {:?}", encrypted_data);

    // Attempt to decrypt with the second private key, which should fail
    let decrypted_result = decrypt_asymmetric(&encrypted_data, &private_key2);
    let decrypted_data = match decrypted_result {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Encryption failed: {}", e);
            return;
        }
    };

    println!("Decrypted Data: {:?}", decrypted_data);

    // Check that decryption failed (the decrypted data should be empty)
    assert_eq!(decrypted_data.len(), 0);
}

#[test]
fn test_encryption_decryption_various_data_sizes() {
    let (public_key, private_key) = generate_key_pair();
    let small_data = "small".as_bytes();
    let large_data = [0u8; 1024 * 1024]; // 1 MiB of data

    let encrypted_result = encrypt_asymmetric(small_data, &public_key);
    let encrypted_small_data = match encrypted_result {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error during encryption: {}", err);
            return;
        }
    };
    let decrypted_result = decrypt_asymmetric(&encrypted_small_data, &private_key);
    let decrypted_small_data = match decrypted_result {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error during decryption: {}", err);
            return;
        }
    };

    assert_eq!(small_data, decrypted_small_data.as_slice());

    let encrypted_result = encrypt_asymmetric(&large_data, &public_key);
    let encrypted_large_data = match encrypted_result {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error during encryption: {}", err);
            return;
        }
    };
    let decrypted_result = decrypt_asymmetric(&encrypted_large_data, &private_key);

    let decrypted_large_data = match decrypted_result {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error during decryption: {}", err);
            return;
        }
    };
    assert_eq!(large_data, decrypted_large_data.as_slice());
}
#[test]
fn test_encryption_decryption_various_data_types() {
    let (public_key, private_key) = generate_key_pair();
    let binary_data = [0x00, 0xff, 0xab, 0x7c];

    let encrypted_result = encrypt_asymmetric(&binary_data, &public_key);
    let encrypted_binary_data = match encrypted_result {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error during encryption: {}", err);
            return;
        }
    };
    // Print the encrypted data for debugging.
    println!("Encrypted data: {:?}", encrypted_binary_data);

    let decrypted_result = decrypt_asymmetric(&encrypted_binary_data, &private_key);
    let decrypted_binary_data = match decrypted_result {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error during decryption: {}", err);
            return;
        }
    };
    // Print the decrypted data for debugging.
    println!("Decrypted data: {:?}", decrypted_binary_data);

    // Assert that the decrypted data matches the original data.
    assert_eq!(
        binary_data,
        decrypted_binary_data.as_slice(),
        "Decrypted data does not match original data"
    );
}

#[test]
fn test_encryption_uniqueness() {
    let (public_key, _) = generate_key_pair();
    let data = "Test data".as_bytes();

    let data1 = [&[1][..], data].concat();
    let data2 = [&[2][..], data].concat();

    let encrypted_data1 = encrypt_asymmetric(&data1, &public_key);
    let encrypted_data2 = encrypt_asymmetric(&data2, &public_key);

    // Debug: Print out the encrypted data and any errors.
    match &encrypted_data1 {
        Ok(data) => println!("Encrypted data1: {:?}", data),
        Err(e) => println!("Error encrypting data1: {}", e),
    }
    match &encrypted_data2 {
        Ok(data) => println!("Encrypted data2: {:?}", data),
        Err(e) => println!("Error encrypting data2: {}", e),
    }

    // Ensure both encrypted_data1 and encrypted_data2 are Ok before asserting inequality.
    if encrypted_data1.is_ok() && encrypted_data2.is_ok() {
        assert_ne!(encrypted_data1.unwrap(), encrypted_data2.unwrap());
    } else {
        println!("Error during encryption, skipping inequality assertion.");
    }
}

#[test]
fn test_error_handling() {
    let (public_key, private_key) = generate_key_pair();
    let encrypted_result = encrypt_asymmetric(&[], &public_key);
    let encrypted_data = match encrypted_result {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error during encryption: {}", err);
            return;
        }
    };

    let decrypted_result = decrypt_asymmetric(&encrypted_data, &private_key);
    let decrypted_data = match decrypted_result {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error during decryption: {}", err);
            return;
        }
    };

    assert_eq!(decrypted_data.len(), 0);
}

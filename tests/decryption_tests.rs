use cryptonique::encryption::operations::{decrypt_data, encrypt_data};
use cryptonique::htm::htm_model::HTMModel;
use cryptonique::htm::temporal_keys::TemporalKey;
use cryptonique::shared_state::SharedState;
use std::{thread, time};
#[test]
fn test_decryption() {
    let data = vec![1, 2, 3, 4, 5];
    let encrypted_data = encrypt_data(&data);
    let decrypted_data = decrypt_data(&encrypted_data);
    assert_eq!(data, decrypted_data);
}

#[test]
fn test_encryption() {
    let data = vec![1, 2, 3, 4, 5];
    let encrypted_data = encrypt_data(&data);
    let decrypted_data = decrypt_data(&encrypted_data);
    assert_eq!(data, decrypted_data);
}

#[test]
#[test]
fn test_encryption_decryption() {
    // Initialize test data and keys
    let data = vec![1, 2, 3, 4, 5];
    let initial_key: Vec<u8> = vec![0; 256];
    let htm_model = HTMModel::new();

    #[test]
    fn test_encryption_decryption() {
        // Initialize test data and keys
        let data = vec![1, 2, 3, 4, 5];
        let initial_key: Vec<u8> = vec![0; 256];
        let htm_model = HTMModel::new();
        let evolution_interval = std::time::Duration::from_secs(10); // 10 seconds

        // Create public and private TemporalKey instances
        let public_key =
            TemporalKey::new(initial_key.clone(), htm_model.clone(), evolution_interval);
        let private_key = TemporalKey::new(initial_key, htm_model, evolution_interval);

        // Perform encryption and decryption
        let encrypted_data = encrypt_data(&data);
        let decrypted_data = decrypt_data(&encrypted_data);

        // Output for debugging and comparison
        println!("Original data: {:?}", data);
        println!("Decrypted data: {:?}", decrypted_data);

        // Assertions to validate the test
        assert_eq!(
            data, decrypted_data,
            "Original and decrypted data do not match"
        );
        assert_eq!(
            data.len(),
            decrypted_data.len(),
            "Lengths of original and decrypted data do not match"
        );
    }

    // Perform encryption and decryption
    let encrypted_data = encrypt_data(&data);
    let decrypted_data = decrypt_data(&encrypted_data);

    // Output for debugging and comparison
    println!("Original data: {:?}", data);
    println!("Decrypted data: {:?}", decrypted_data);

    // Assertions to validate the test
    assert_eq!(
        data, decrypted_data,
        "Original and decrypted data do not match"
    );
    assert_eq!(
        data.len(),
        decrypted_data.len(),
        "Lengths of original and decrypted data do not match"
    );
}

// Other test functions should follow the similar pattern.

// #[test]
// fn test_various_data_sizes() {
//     let public_key = vec![1, 2, 3];
//     let private_key = vec![4, 5, 6];
//     let mut shared_state = SharedState::new();

//     for &size in &[0, 1, 10, 100, 1000, 10000] {
//         let data: Vec<u8> = (0..size).map(|_| rand::random::<u8>()).collect();
//         println!("Size: {}", size);
//         println!("Original Data: {:?}", data);

//         let encrypted_data = encrypt(&data, &public_key, &mut shared_state);
//         println!("Encrypted Data: {:?}", encrypted_data);

//         let decrypted_data = decrypt(&encrypted_data, &private_key, &mut shared_state);
//         println!("Decrypted Data: {:?}", decrypted_data);

//         if data != decrypted_data {
//             println!("Failure Details:");
//             println!("Expected: {:?}", data);
//             println!("Got:      {:?}", decrypted_data);
//         }

//         // Assert equality, this will panic and print a message if the assertion fails.
//         assert_eq!(data, decrypted_data, "Failure at size: {}", size);

//         // Sleep for a short time to allow reading the debug info for each size.
//         thread::sleep(time::Duration::from_millis(500));
//     }
// }

// #[test]
// fn test_wrong_keys() {
//     let data = vec![1, 2, 3, 4, 5];
//     let public_key1 = vec![1, 2, 3];
//     let public_key2 = vec![4, 5, 6];
//     let private_key = vec![7, 8, 9];
//     let mut shared_state = SharedState::new();

//     let encrypted_data = encrypt(&data, &public_key1, &mut shared_state);
//     let decrypted_data = decrypt(&encrypted_data, &private_key, &mut shared_state);

//     assert_ne!(data, decrypted_data);
// }

// #[test]
// fn test_random_data() {
//     let data: Vec<u8> = (0..100).map(|_| rand::random::<u8>()).collect();
//     let public_key = vec![1, 2, 3];
//     let private_key = vec![4, 5, 6];
//     let mut shared_state = SharedState::new();

//     let encrypted_data = encrypt(&data, &public_key, &mut shared_state);
//     let decrypted_data = decrypt(&encrypted_data, &private_key, &mut shared_state);

//     assert_eq!(data, decrypted_data);
// }

fn test_error_handling() {
    let data = vec![1, 2, 3, 4, 5];

    // Initialize public_key and invalid_private_key as TemporalKey
    let initial_key: Vec<u8> = vec![0; 256];

    // Creating an instance of HTMModel.
    // Ensure HTMModel::new() is a valid call and add arguments if needed.
    let htm_model = HTMModel::new();

    // Creating a TemporalKey instance with initial key and HTMModel.
    #[test]
    fn test_encryption_decryption() {
        // Initialize test data and keys
        let data = vec![1, 2, 3, 4, 5];
        let initial_key: Vec<u8> = vec![0; 256];
        let htm_model = HTMModel::new();
        let evolution_interval = std::time::Duration::from_secs(10); // 10 seconds

        // Create public and private TemporalKey instances
        let public_key =
            TemporalKey::new(initial_key.clone(), htm_model.clone(), evolution_interval);
        let private_key = TemporalKey::new(initial_key, htm_model, evolution_interval);

        // Perform encryption and decryption
        let encrypted_data = encrypt_data(&data);
        let decrypted_data = decrypt_data(&encrypted_data);

        // Output for debugging and comparison
        println!("Original data: {:?}", data);
        println!("Decrypted data: {:?}", decrypted_data);

        // Assertions to validate the test
        assert_eq!(
            data, decrypted_data,
            "Original and decrypted data do not match"
        );
        assert_eq!(
            data.len(),
            decrypted_data.len(),
            "Lengths of original and decrypted data do not match"
        );
    }

    let shared_state_htm_model = htm_model.clone();

    let mut state = SharedState::new(shared_state_htm_model);

    let encrypted_data = encrypt_data(&data);
    let decrypted_data = decrypt_data(&encrypted_data);

    assert_ne!(data, decrypted_data);
}

// #[test]
// fn test_encryption_uniqueness() {
//     let data = vec![1, 2, 3, 4, 5];
//     let public_key = vec![1, 2, 3];
//     let mut shared_state = SharedState::new();

//     let encrypted_data1 = encrypt(&data, &public_key, &mut shared_state);
//     let encrypted_data2 = encrypt(&data, &public_key, &mut shared_state);

//     assert_ne!(encrypted_data1, encrypted_data2);
// }

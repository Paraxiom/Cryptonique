//src/main.rs
#![allow(warnings)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod quantumtimesandwich {
    tonic::include_proto!("quantumtimesandwich");
}
use crate::quantumtimesandwich::DecryptionRequest;
use crate::quantumtimesandwich::EncryptionRequest;
use quantumtimesandwich::quantum_encryption_service_client::QuantumEncryptionServiceClient;
use quantumtimesandwich::{GenerateKeyRequest, MeasureStateRequest, PrepareStateRequest};
use crate::quantumtimesandwich::GetKeyRequest;

use log::error;
use log::info;
use log::warn;

use std::env;
use tokio::runtime::Runtime;
use tonic::transport::Channel;
pub mod encryption;
pub mod htm;
pub mod shared_state;
pub mod utils;

use std::time::Duration;
use uuid::Uuid;

pub mod quantum_algorithms;
use quantum_algorithms::*;
use crate::quantum_algorithms::quantum_resistant_algos::{AlgorithmType, quantum_resistant_algorithm_factory};


async fn run_client() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to QuantumTimeSandwich gRPC server
    let mut client = QuantumEncryptionServiceClient::connect("http://0.0.0.0:50051").await?;
    let session_id = 1.to_string(); //Uuid::new_v4().to_string();
                                    // Request for key generation
    let key_gen_response = client
        .generate_key(GenerateKeyRequest {
            session_id: session_id.clone(),
        })
        .await?;
    println!("Generated Key: {:?}", key_gen_response.into_inner().key);

    // Prepare quantum states (Alice's role)
    let num_qubits = 10; // Specify the number of qubits
    let prepare_state_request = PrepareStateRequest {
        num_qubits,
        session_id: session_id.clone(),
    };
    let prepare_state_response = client.prepare_quantum_state(prepare_state_request).await?;
    println!(
        "Prepared Quantum States Response: {:?}",
        prepare_state_response.into_inner()
    );

    // Measure quantum states (Bob's role)
    // Populate the MeasureStateRequest fields as needed
    let measure_state_request = MeasureStateRequest { session_id };
    let measure_state_response = client.measure_quantum_state(measure_state_request).await?;
    println!(
        "Measured Quantum States Response: {:?}",
        measure_state_response.into_inner()
    );

    // Additional QKD steps like sifting, error correction, privacy amplification, etc.
    // ...

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // Command-line argument parsing
    match args.get(1).map(String::as_str) {
        Some("generate-keypair") => {
            let algorithm = match args.get(2).map(String::as_str) {
                Some("kyber") => AlgorithmType::Kyber,
                Some("bike") => AlgorithmType::Bike,
                Some("classicmceliece") => AlgorithmType::ClassicMcEliece,
                Some("frodo") => AlgorithmType::Frodo,
                Some("hqc") => AlgorithmType::Hqc,
                Some("ntruprime") => AlgorithmType::NtruPrime,
                Some("dilithium") => AlgorithmType::Dilithium,
                Some("falcon") => AlgorithmType::Falcon,
                Some("sphincs") => AlgorithmType::Sphincs,
                _ => return Err("Unsupported algorithm".into()),
            };
    
            let algorithm_instance = quantum_resistant_algorithm_factory(algorithm);
            let (public_key, secret_key) = algorithm_instance.generate_keypair()?;
            println!("Public Key: {:?}\nSecret Key: {:?}", public_key, secret_key);
        },
        Some("encrypt") => {
            let algorithm = match args.get(2).map(String::as_str) {
                Some("kyber") => AlgorithmType::Kyber,
                Some("bike") => AlgorithmType::Bike,
                Some("classicmceliece") => AlgorithmType::ClassicMcEliece,
                Some("frodo") => AlgorithmType::Frodo,
                Some("hqc") => AlgorithmType::Hqc,
                Some("ntruprime") => AlgorithmType::NtruPrime,
                Some("dilithium") => AlgorithmType::Dilithium,
                Some("falcon") => AlgorithmType::Falcon,
                Some("sphincs") => AlgorithmType::Sphincs,
                _ => return Err("Unsupported algorithm".into()),
            };
        
            let algorithm_instance = quantum_resistant_algorithm_factory(algorithm);
        
            
            if let (Some(public_key_arg), Some(message_arg)) = (args.get(3), args.get(4)) {
                let public_key = hex::decode(public_key_arg)?;
                let message = message_arg.as_bytes();
        
                let ciphertext = algorithm_instance.encrypt(&public_key, message)?;
                println!("Encrypted Message: {:?}", ciphertext);
            } else {
                return Err("Missing public key or message".into());
            }
        },
        Some("decrypt") => {
            let args: Vec<String> = env::args().collect();
            let secret_key_arg = args.get(2).expect("No secret key provided");
            let ciphertext_arg = args.get(3).expect("No ciphertext provided");
        
            // Assuming the secret key and ciphertext are provided as hexadecimal strings
            let secret_key = hex::decode(secret_key_arg)?; // Replace with appropriate decoding if not hex
            let ciphertext = hex::decode(ciphertext_arg)?; // Replace with appropriate decoding if not hex
        
            let algorithm = match args.get(2).map(String::as_str) {
                Some("kyber") => AlgorithmType::Kyber,
                Some("bike") => AlgorithmType::Bike,
                Some("classicmceliece") => AlgorithmType::ClassicMcEliece,
                Some("frodo") => AlgorithmType::Frodo,
                Some("hqc") => AlgorithmType::Hqc,
                Some("ntruprime") => AlgorithmType::NtruPrime,
                Some("dilithium") => AlgorithmType::Dilithium,
                Some("falcon") => AlgorithmType::Falcon,
                Some("sphincs") => AlgorithmType::Sphincs,
                _ => return Err("Unsupported algorithm".into()),
            };
        
            // Create an instance of the algorithm
            let algorithm_instance = quantum_resistant_algorithm_factory(algorithm);
        
            // Perform the decryption
            match algorithm_instance.decrypt(&secret_key, &ciphertext) {
                Ok(plaintext) => println!("Decrypted plaintext: {:?}", plaintext),
                Err(e) => println!("Error during decryption: {:?}", e),
            }
        },
        Some("sign") => { /* Sign implementation */ },
        Some("verify") => { /* Verify implementation */ },
        Some("hash") => { /* Hash implementation */ },
        Some("symmetric-encrypt") => { /* Symmetric-Encrypt implementation */ },
        Some("symmetric-decrypt") => { /* Symmetric-Decrypt implementation */ },
        Some("derive-key") => { /* Derive-Key implementation */ },
        Some("encode") => { /* Encode implementation */ },
        Some("decode") => { /* Decode implementation */ },
        Some("key-exchange") => { /* Key-Exchange implementation */ },
        Some("random-bytes") => { /* Random-Bytes implementation */ },
        // Add other command cases here (sign, verify, etc.)
        _ => {
            println!("Usage: cryptonique [command] [algorithm]");
        }
    }

    Ok(())
}
async fn run_as_alice(session_id: String) -> Result<(), Box<dyn std::error::Error>> {
    info!("Running as Alice...");
    let session_id = 1.to_string(); //Uuid::new_v4().to_string();

    let get_key_request = GetKeyRequest {
        session_id: session_id.clone(),
    };
    // Configure the gRPC client
    let mut client = QuantumEncryptionServiceClient::connect("http://127.0.0.1:50051").await?;
    info!("Alice connected to QuantumTimeSandwich server.");

    // Define the number of qubits for the simulation
    let num_qubits = 10; // Example: 10 qubits
    info!(
        "Alice preparing to send quantum state preparation request with {} qubits.",
        num_qubits
    );

    let prepare_state_request = PrepareStateRequest {
        num_qubits,
        session_id: session_id.clone(), // Include session_id
    };
    let prepare_state_response = client.prepare_quantum_state(prepare_state_request).await?;
    info!(
        "Response from server to Alice: {:?}",
        prepare_state_response.into_inner().message
    );

    loop {
        let request_clone = get_key_request.clone(); // Clone the request for each loop iteration
        match client.get_key(request_clone).await {
            Ok(response) => {
                let key = response.into_inner().key;
                println!("Received Key: {:?}", key);
                break;
            }
            Err(e) => {
                warn!("Waiting for key generation. Error: {:?}", e);
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }

    Ok(())
}

async fn run_as_bob(session_id: String) -> Result<(), Box<dyn std::error::Error>> {
    info!("Running as Bob...");
    let session_id = 1.to_string(); //Uuid::new_v4().to_string();
                                    // Configure the gRPC client
    let mut client = QuantumEncryptionServiceClient::connect("http://127.0.0.1:50051").await?;
    info!("Bob connected to QuantumTimeSandwich server.");
    let get_key_request = GetKeyRequest {
        session_id: session_id.clone(),
    };
    let measure_state_request = MeasureStateRequest {
        session_id: session_id.clone(), // Include session_id
                                        // ... [populate other fields as needed] ...
    };
    let measure_state_response = client.measure_quantum_state(measure_state_request).await?;
    info!(
        "Response from server to Bob: {:?}",
        measure_state_response.into_inner().message
    );

    // Poll for the key
    loop {
        let request_clone = get_key_request.clone(); // Clone the request for each loop iteration
        match client.get_key(request_clone).await {
            Ok(response) => {
                let key = response.into_inner().key;
                println!("Received Key: {:?}", key);
                break;
            }
            Err(e) => {
                warn!("Waiting for key generation. Error: {:?}", e);
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }

    Ok(())
}

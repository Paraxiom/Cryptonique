use std::env;
use tonic::transport::Channel;
use quantumtimesandwich::quantum_encryption_service_client::QuantumEncryptionServiceClient;
use quantumtimesandwich::{GenerateKeyRequest, PrepareStateRequest, MeasureStateRequest};
use crate::quantumtimesandwich::EncryptionRequest;
use crate::quantumtimesandwich::DecryptionRequest;
use tokio::runtime::Runtime;
use log::warn;
use log::info;
use log::error;
pub mod encryption;
pub mod htm;
pub mod shared_state;
pub mod utils;
use std::time::Duration;
use uuid::Uuid;
use log::debug;
use crate::quantumtimesandwich::GetKeyRequest;
pub mod quantumtimesandwich {
    tonic::include_proto!("quantumtimesandwich");
}
use tonic::transport::{ClientTlsConfig};


async fn run_client() -> Result<(), Box<dyn std::error::Error>> {
    // Read the CA certificate
    println!("Running client...");
    let ca_cert = tokio::fs::read("./newserver.pem").await?;
    
    println!("{:?}", ca_cert);
    // Create a TlsConfig with the CA certificate
    let tls_config = ClientTlsConfig::new()
        .ca_certificate(tonic::transport::Certificate::from_pem(&ca_cert))
        .domain_name("localhost"); // Use the appropriate domain name of the server
    println!("TLS configuration created.");
    // Connect to QuantumTimeSandwich gRPC server with TLS configuration
    let endpoint = "https://127.0.0.1:50052";
    println!("Connecting to endpoint: {}", endpoint);
    debug!("Connecting to server at: {}", endpoint);
    let channel = Channel::from_static(endpoint)
    .tls_config(tls_config)?
    .connect()
    .await?;
println!("Connected to server.");
let mut client = QuantumEncryptionServiceClient::new(channel);

let session_id = 1.to_string(); // Or use Uuid::new_v4(
    let session_id = 1.to_string();//Uuid::new_v4().to_string();
    debug!("Generating key for session_id: {}", session_id);
    // Request for key generation
    let key_gen_response = client.generate_key(GenerateKeyRequest {
        session_id: session_id.clone(),
    }).await?;
    println!("Generated Key: {:?}", key_gen_response.into_inner().key);

    // Prepare quantum states (Alice's role)
    let num_qubits = 10; // Specify the number of qubits
    let prepare_state_request = PrepareStateRequest {
        num_qubits,
        session_id: session_id.clone(),
    };
    let prepare_state_response = client.prepare_quantum_state(prepare_state_request).await?;
    println!("Prepared Quantum States Response: {:?}", prepare_state_response.into_inner());

    // Measure quantum states (Bob's role)
    // Populate the MeasureStateRequest fields as needed
    let measure_state_request = MeasureStateRequest { session_id };
    let measure_state_response = client.measure_quantum_state(measure_state_request).await?;
    println!("Measured Quantum States Response: {:?}", measure_state_response.into_inner());

    // Additional QKD steps like sifting, error correction, privacy amplification, etc.
    // ...

    Ok(())
}







#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting main function...");
    let session_id = 1.to_string();//Uuid::new_v4().to_string();
    
    println!("Session ID: {}", session_id);
    let args: Vec<String> = env::args().collect();
    let session_id = 1.to_string();//"your_session_id_here"; // Replace with actual session_id

    match args.get(1).map(String::as_str) {
        Some("alice") => {
            println!("Running as Alice...");
            run_as_alice(session_id.to_string()).await
        },
        Some("bob") => {
            println!("Running as Bob...");
            run_as_bob(session_id.to_string()).await
        },
        _ => {
            println!("Usage: cryptonique [alice|bob]");
            Err("Invalid argument".into())
        }
    }
}
async fn run_as_alice(session_id: String) -> Result<(), Box<dyn std::error::Error>> {
    // Read the CA certificate for TLS
    let ca_cert = tokio::fs::read("./newserver.pem").await?;
    let tls_config = ClientTlsConfig::new()
        .ca_certificate(tonic::transport::Certificate::from_pem(&ca_cert))
        .domain_name("localhost");

    // Configure the gRPC client with TLS configuration
    let endpoint = "https://127.0.0.1:50052";
    let channel = Channel::from_static(endpoint)
        .tls_config(tls_config)?
        .connect()
        .await?;

    let mut client = QuantumEncryptionServiceClient::new(channel);

    // Define the number of qubits for the simulation
    let num_qubits = 10; // Example: 10 qubits
    info!("Alice preparing to send quantum state preparation request with {} qubits.", num_qubits);

    let prepare_state_request = PrepareStateRequest {
        num_qubits,
        session_id: session_id.clone(),
    };
    let prepare_state_response = client.prepare_quantum_state(prepare_state_request).await?;
    info!("Response from server to Alice: {:?}", prepare_state_response.into_inner().message);
    
    // Loop to receive the key
    let get_key_request = GetKeyRequest {
        session_id: session_id.clone(),
    };

    loop {
        let request_clone = get_key_request.clone();
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
    // Read the CA certificate for TLS
    let ca_cert = tokio::fs::read("./newserver.pem").await?;
    let tls_config = ClientTlsConfig::new()
        .ca_certificate(tonic::transport::Certificate::from_pem(&ca_cert))
        .domain_name("localhost");

    // Configure the gRPC client with TLS configuration
    let endpoint = "https://127.0.0.1:50052";
    let channel = Channel::from_static(endpoint)
        .tls_config(tls_config)?
        .connect()
        .await?;

    let mut client = QuantumEncryptionServiceClient::new(channel);

    // Logic for Bob's role in the protocol
    let measure_state_request = MeasureStateRequest {
        session_id: session_id.clone(),
        // ... [populate other fields as needed] ...
    };
    let measure_state_response = client.measure_quantum_state(measure_state_request).await?;
    info!("Response from server to Bob: {:?}", measure_state_response.into_inner().message);

    // Loop to receive the key
    let get_key_request = GetKeyRequest {
        session_id: session_id.clone(),
    };

    loop {
        let request_clone = get_key_request.clone();
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

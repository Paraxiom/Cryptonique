// src/quantum_communication/mod.rs
use tonic::transport::Channel;
pub mod quantumtimesandwich {
    tonic::include_proto!("quantumtimesandwich");
}
use quantumtimesandwich::DecryptionRequest;
use quantumtimesandwich::EncryptionRequest;
use quantumtimesandwich::quantum_encryption_service_client::QuantumEncryptionServiceClient;
use quantumtimesandwich::{GenerateKeyRequest, MeasureStateRequest, PrepareStateRequest, GenerateKeyResponse};
use quantumtimesandwich::GetKeyRequest;
use quantumtimesandwich::KeyExchangeRequest;
use quantumtimesandwich::KeyExchangeResponse;



pub use oqs_kex_rpc::client::QuantumSafeKeyExchangeClient;
pub mod oqs_kex_rpc;

pub struct QuantumClient {
    client: QuantumEncryptionServiceClient<Channel>,
}

impl QuantumClient {
   // Existing connection method
   pub async fn connect(addr: String) -> Result<Self, Box<dyn std::error::Error>> {
    let client = QuantumEncryptionServiceClient::connect(addr).await?;
    Ok(QuantumClient { client })
}

// Existing generate_key method
pub async fn generate_key(&mut self) -> Result<GenerateKeyResponse, Box<dyn std::error::Error>> {
    let request = tonic::Request::new(GenerateKeyRequest {
        session_id: "your_session_id".to_string(),
    });
    
    let response = self.client.generate_key(request).await?;
    Ok(response.into_inner())
}

// New method for sending key exchange request
pub async fn send_key_exchange_request(&mut self, public_key: Vec<u8>) -> Result<KeyExchangeResponse, Box<dyn std::error::Error>> {
    let request = tonic::Request::new(KeyExchangeRequest {
        public_key,
    });
    
    let response = self.client.key_exchange(request).await?;
    Ok(response.into_inner())
}

// New method for key exchange
pub async fn key_exchange(&mut self, public_key: Vec<u8>) -> Result<KeyExchangeResponse, Box<dyn std::error::Error>> {
    let request = tonic::Request::new(KeyExchangeRequest {
        public_key,
    });
    
    let response = self.client.key_exchange(request).await?;
    Ok(response.into_inner())
}
    // ... other methods for EncryptMessage, DecryptMessage, etc.
}

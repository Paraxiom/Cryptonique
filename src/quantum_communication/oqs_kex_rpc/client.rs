use oqs::kem::{Kem, Algorithm, CiphertextRef};
extern crate oqs; 

// client.rs
pub mod quantumtimesandwich {
    tonic::include_proto!("quantumtimesandwich");
}

use quantumtimesandwich::quantum_encryption_service_client::QuantumEncryptionServiceClient;
use quantumtimesandwich::{KeyExchangeRequest, KeyExchangeResponse};
use super::QuantumSafeKey;
use tonic::transport::Channel;

// This struct represents your quantum-safe key exchange client
pub struct QuantumSafeKeyExchangeClient {
    server_address: String,
    kem: Kem,
}

impl QuantumSafeKeyExchangeClient {
    // Constructor for the client
    pub fn new(server_address: String, kem_algorithm: &str) -> QuantumSafeKeyExchangeClient {
        let algorithm = str_to_algorithm(kem_algorithm).expect("Invalid KEM algorithm");
        let kem = Kem::new(algorithm).expect("Failed to initialize KEM");

        QuantumSafeKeyExchangeClient {
            server_address,
            kem,
        }
    }

    pub async fn initiate_key_exchange(&mut self) -> Result<QuantumSafeKey, Box<dyn std::error::Error>> {
        let (public_key, secret_key) = self.kem.keypair()?;

        let public_key_bytes: &[u8] = public_key.as_ref(); // Convert PublicKey to &[u8]
        let encapsulated_key = self.send_key_exchange_request(public_key_bytes).await?;

        // Use Kem's ciphertext_from_bytes to convert &[u8] to CiphertextRef
        let ciphertext_ref = self.kem.ciphertext_from_bytes(&encapsulated_key)
            .ok_or("Failed to convert to CiphertextRef")?;

        // Use the CiphertextRef for decapsulation
        let shared_secret = self.kem.decapsulate(&secret_key, ciphertext_ref)?;

        Ok(QuantumSafeKey {
            key: shared_secret.into_vec(),
            algorithm: self.kem.algorithm().to_string(),
        })
    }

    // Function to send key exchange request
    async fn send_key_exchange_request(&self, public_key: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut client = QuantumEncryptionServiceClient::connect(self.server_address.clone()).await?;
        let request = tonic::Request::new(KeyExchangeRequest {
            public_key: public_key.to_vec(),
        });
        let response = client.key_exchange(request).await?;
        let encapsulated_key = response.into_inner().encapsulated_key;
        Ok(encapsulated_key)
    }
    
}

// Helper function for Algorithm mapping
fn str_to_algorithm(algo_str: &str) -> Result<Algorithm, &'static str> {
    match algo_str {
        "kyber512" => Ok(Algorithm::Kyber512),
        // Add other algorithms as needed
        _ => Err("Unsupported algorithm"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oqs::kem::Algorithm;

    #[tokio::test]
    async fn test_key_generation() {
        let mut client = QuantumSafeKeyExchangeClient::new("http://mockserver:50051".to_string(), "kyber512");
        let result = client.initiate_key_exchange().await;
        assert!(result.is_ok());

        let key = result.unwrap();
        assert!(!key.key.is_empty());
        assert_eq!(key.algorithm, Algorithm::Kyber512.to_string());
    }
}

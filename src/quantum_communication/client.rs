// Assume this is in `src/quantum_communication/mod.rs` or similar
use tonic::transport::Channel;
use quantumtimesandwich::quantum_encryption_service_client::QuantumEncryptionServiceClient;
use quantumtimesandwich::{GenerateKeyRequest, EncryptionRequest, DecryptionRequest, PrepareStateRequest, MeasureStateRequest};

pub mod quantumtimesandwich {
    tonic::include_proto!("quantumtimesandwich"); // The string here must match the package name in the .proto file
}

pub struct QuantumClient {
    client: QuantumEncryptionServiceClient<Channel>,
}

impl QuantumClient {
    pub async fn connect(addr: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = QuantumEncryptionServiceClient::connect(addr).await?;
        Ok(QuantumClient { client })
    }

    // Implement methods for each gRPC call, for example:
    pub async fn generate_key(&mut self) -> Result<GenerateKeyResponse, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(GenerateKeyRequest {});
        let response = self.client.generate_key(request).await?;
        Ok(response.into_inner())
    }

    // ... other methods for EncryptMessage, DecryptMessage, etc.
}

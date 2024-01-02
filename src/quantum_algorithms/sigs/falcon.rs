use oqs::sig::{Algorithm, Sig};

pub struct FalconSignature {
    sig: Sig,
}

impl FalconSignature {
    // Constructs a new FalconSignature instance
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let sig = Sig::new(Algorithm::Falcon512)
            .map_err(|e| format!("Failed to initialize Falcon signature: {:?}", e))?;
        Ok(FalconSignature { sig })
    }

    // Generates a keypair and returns them as byte vectors
    pub fn keypair(&self) -> Result<(Vec<u8>, Vec<u8>), Box<dyn std::error::Error>> {
        let (public_key, secret_key) = self.sig.keypair()
            .map_err(|e| format!("Failed to generate keypair: {:?}", e))?;
        Ok((public_key.into_vec(), secret_key.into_vec()))
    }

    // Signs a message with a secret key
    pub fn sign(&self, secret_key: &[u8], message: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let secret_key_ref = self.sig.secret_key_from_bytes(secret_key)
            .ok_or("Failed to create secret key ref from bytes")?;
        let signature = self.sig.sign(message, &secret_key_ref)
            .map_err(|e| format!("Failed to sign message: {:?}", e))?;
        Ok(signature.into_vec())
    }

    pub fn verify(&self, public_key: &[u8], message: &[u8], signature: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let public_key_ref = self.sig.public_key_from_bytes(public_key)
            .ok_or("Failed to create public key ref from bytes")?;
        let signature_ref = self.sig.signature_from_bytes(signature)
            .ok_or("Failed to create signature ref from bytes")?;
        
        self.sig.verify(message, &signature_ref, &public_key_ref)
            .map_err(|e| format!("Failed to verify signature: {:?}", e).into()) // Changed here
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_falcon_signature() -> Result<(), Box<dyn std::error::Error>> {
        let falcon = FalconSignature::new()?;
        let (public_key, secret_key) = falcon.keypair()?;
        let message = b"Hello, world!";
        let signature = falcon.sign(&secret_key, message)?;
        falcon.verify(&public_key, message, &signature)?;

        Ok(())
    }
}

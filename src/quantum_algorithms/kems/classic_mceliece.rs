use oqs::kem::{Algorithm, Kem};

pub struct ClassicMcElieceKem {
    kem: Kem,
}

impl ClassicMcElieceKem {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let kem = Kem::new(Algorithm::ClassicMcEliece348864)
            .map_err(|e| format!("Failed to initialize Classic McEliece KEM: {:?}", e))?;
        Ok(ClassicMcElieceKem { kem })
    }

    pub fn keypair(&self) -> Result<(Vec<u8>, Vec<u8>), Box<dyn std::error::Error>> {
        let (public_key, secret_key) = self.kem.keypair()
            .map_err(|e| format!("Failed to generate keypair: {:?}", e))?;
        Ok((public_key.into_vec(), secret_key.into_vec()))
    }

    pub fn encapsulate(&self, public_key_bytes: &[u8]) -> Result<(Vec<u8>, Vec<u8>), Box<dyn std::error::Error>> {
        let public_key = self.kem.public_key_from_bytes(public_key_bytes)
            .ok_or("Failed to create public key from bytes")?;
        let (ciphertext, shared_secret) = self.kem.encapsulate(&public_key)
            .map_err(|e| format!("Failed to encapsulate: {:?}", e))?;
        Ok((ciphertext.into_vec(), shared_secret.into_vec()))
    }

    pub fn decapsulate(&self, secret_key_bytes: &[u8], ciphertext_bytes: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let secret_key = self.kem.secret_key_from_bytes(secret_key_bytes)
            .ok_or("Failed to create secret key from bytes")?;
        let ciphertext = self.kem.ciphertext_from_bytes(ciphertext_bytes)
            .ok_or("Failed to create ciphertext from bytes")?;
        let shared_secret = self.kem.decapsulate(&secret_key, &ciphertext)
            .map_err(|e| format!("Failed to decapsulate: {:?}", e))?;
        Ok(shared_secret.into_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classic_mceliece_kem() -> Result<(), Box<dyn std::error::Error>> {
        let mceliece = ClassicMcElieceKem::new()?;
        let (public_key, secret_key) = mceliece.keypair()?;
        let (ciphertext, shared_secret_enc) = mceliece.encapsulate(&public_key)?;
        let shared_secret_dec = mceliece.decapsulate(&secret_key, &ciphertext)?;

        assert_eq!(shared_secret_enc, shared_secret_dec);
        Ok(())
    }
}

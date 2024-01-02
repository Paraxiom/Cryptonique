use oqs::kem::{Kem, Algorithm};
use std::error::Error;

pub struct BikeKem {
    kem: Kem,
}

impl BikeKem {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let kem = Kem::new(Algorithm::BikeL1)?;
        Ok(BikeKem { kem })
    }

    pub fn keypair(&self) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
        let (public_key, secret_key) = self.kem.keypair()?;
        Ok((public_key.into_vec(), secret_key.into_vec()))
    }

    pub fn encapsulate(&self, public_key: &[u8]) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
        let public_key_ref = self.kem.public_key_from_bytes(public_key)
            .ok_or("Failed to create PublicKeyRef from bytes")?;
        let (ciphertext, shared_secret) = self.kem.encapsulate(&public_key_ref)?;
        Ok((ciphertext.into_vec(), shared_secret.into_vec()))
    }

    pub fn decapsulate(&self, secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let secret_key_ref = self.kem.secret_key_from_bytes(secret_key)
            .ok_or("Failed to create SecretKeyRef from bytes")?;
        let ciphertext_ref = self.kem.ciphertext_from_bytes(ciphertext)
            .ok_or("Failed to create CiphertextRef from bytes")?;
        let shared_secret = self.kem.decapsulate(&secret_key_ref, &ciphertext_ref)?;
        Ok(shared_secret.into_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bike_kem() -> Result<(), Box<dyn Error>> {
        let kem = BikeKem::new()?;
        let (public_key, secret_key) = kem.keypair()?;
        let (ciphertext, shared_secret_encapsulate) = kem.encapsulate(&public_key)?;
        let shared_secret_decapsulate = kem.decapsulate(&secret_key, &ciphertext)?;
        assert_eq!(shared_secret_encapsulate, shared_secret_decapsulate);
        Ok(())
    }
}

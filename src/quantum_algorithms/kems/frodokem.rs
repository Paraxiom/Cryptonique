use oqs::kem::{Algorithm, Kem};

pub struct FrodoKEM {
    kem: Kem,
}

impl FrodoKEM {
    pub fn new() -> Self {
        let kem = Kem::new(Algorithm::FrodoKem640Aes)
            .expect("Failed to initialize FrodoKEM");
        FrodoKEM { kem }
    }

    pub fn keypair(&self) -> (Vec<u8>, Vec<u8>) {
        let (public_key, secret_key) = self.kem.keypair()
            .expect("Failed to generate keypair for FrodoKEM");
        (public_key.into_vec(), secret_key.into_vec())
    }

    pub fn encapsulate(&self, public_key: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let public_key_ref = self.kem.public_key_from_bytes(public_key)
            .expect("Invalid public key format");
        let (ciphertext, shared_secret) = self.kem.encapsulate(&public_key_ref)
            .expect("Failed to encapsulate secret");
        (ciphertext.into_vec(), shared_secret.into_vec())
    }

    pub fn decapsulate(&self, secret_key: &[u8], ciphertext: &[u8]) -> Vec<u8> {
        let secret_key_ref = self.kem.secret_key_from_bytes(secret_key)
            .expect("Invalid secret key format");
        let ciphertext_ref = self.kem.ciphertext_from_bytes(ciphertext)
            .expect("Invalid ciphertext format");
        self.kem.decapsulate(&secret_key_ref, &ciphertext_ref)
            .expect("Failed to decapsulate secret")
            .into_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frodokem_encapsulation_decapsulation() {
        let frodokem = FrodoKEM::new();
        let (public_key, secret_key) = frodokem.keypair();
        let (ciphertext, shared_secret_encapsulated) = frodokem.encapsulate(&public_key);
        let shared_secret_decapsulated = frodokem.decapsulate(&secret_key, &ciphertext);

        assert_eq!(shared_secret_encapsulated, shared_secret_decapsulated, "Shared secrets should match after encapsulation and decapsulation");
    }
}

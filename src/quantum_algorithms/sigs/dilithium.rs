use oqs::sig::{Algorithm, Sig, SecretKeyRef, PublicKeyRef, SignatureRef};

pub struct DilithiumSignature {
    sig: Sig,
}

impl DilithiumSignature {
    pub fn new() -> Self {
        let sig = Sig::new(Algorithm::Dilithium3)
            .expect("Failed to create Dilithium instance");
        DilithiumSignature { sig }
    }

    pub fn keypair(&self) -> (Vec<u8>, Vec<u8>) {
        let (public_key, secret_key) = self.sig.keypair()
            .expect("Failed to generate keypair");
        (public_key.into_vec(), secret_key.into_vec())
    }

    pub fn sign(&self, secret_key: &[u8], message: &[u8]) -> Vec<u8> {
        let secret_key_ref = self.sig.secret_key_from_bytes(secret_key)
            .expect("Failed to create SecretKeyRef from bytes");
        self.sig.sign(message, secret_key_ref)
            .expect("Failed to sign message")
            .into_vec()
    }

    pub fn verify(&self, message: &[u8], public_key: &[u8], signature: &[u8]) -> bool {
        let public_key_ref = self.sig.public_key_from_bytes(public_key)
            .expect("Failed to create PublicKeyRef from bytes");
        let signature_ref = self.sig.signature_from_bytes(signature)
            .expect("Failed to create SignatureRef from bytes");
        self.sig.verify(message, signature_ref, public_key_ref).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dilithium_signature() {
        let dilithium = DilithiumSignature::new();
        let (public_key, secret_key) = dilithium.keypair();
        let message = b"Hello, world!";
        let signature = dilithium.sign(&secret_key, message);
        assert!(dilithium.verify(message, &public_key, &signature));
    }
}



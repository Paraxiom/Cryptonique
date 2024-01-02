use oqs::sig::{Algorithm, Sig, SecretKeyRef, PublicKeyRef, SignatureRef};

pub struct SphincsSignature {
    sig: Sig,
}

impl SphincsSignature {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let sig = Sig::new(Algorithm::SphincsSha2128fSimple)
            .map_err(|e| e.into_boxed())?; // Convert to boxed error directly
        Ok(SphincsSignature { sig })
    }

    pub fn keypair(&self) -> Result<(Vec<u8>, Vec<u8>), Box<dyn std::error::Error>> {
        let (public_key, secret_key) = self.sig.keypair()
            .map_err(|e| e.into_boxed())?; // Convert to boxed error directly
        Ok((public_key.into_vec(), secret_key.into_vec()))
    }

    pub fn sign(&self, secret_key: &[u8], message: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let secret_key_ref = SecretKeyRef::from(self.sig.secret_key_from_bytes(secret_key)
            .ok_or_else(|| "Failed to create SecretKeyRef from bytes")?); // Convert Option to Result first
        self.sig.sign(message, secret_key_ref)
            .map_err(|e| e.into_boxed()) // Convert to boxed error directly
            .map(|signature| signature.into_vec())
    }

    pub fn verify(&self, message: &[u8], public_key: &[u8], signature: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let public_key_ref = PublicKeyRef::from(self.sig.public_key_from_bytes(public_key)
            .ok_or_else(|| "Failed to create PublicKeyRef from bytes")?); // Convert Option to Result first
        let signature_ref = SignatureRef::from(self.sig.signature_from_bytes(signature)
            .ok_or_else(|| "Failed to create SignatureRef from bytes")?); // Convert Option to Result first
        self.sig.verify(message, signature_ref, public_key_ref)
            .map_err(|e| e.into_boxed()) // Convert to boxed error directly
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sphincs_signature() -> Result<(), Box<dyn std::error::Error>> {
        let sphincs = SphincsSignature::new()?;
        let (public_key, secret_key) = sphincs.keypair()?;
        let message = b"Test message";
        let signature = sphincs.sign(&secret_key, message)?;
        sphincs.verify(message, &public_key, &signature)?;
        Ok(())
    }
}

// Helper function to convert errors to Box<dyn std::error::Error>
trait IntoBoxedError {
    fn into_boxed(self) -> Box<dyn std::error::Error>;
}

impl IntoBoxedError for oqs::Error {
    fn into_boxed(self) -> Box<dyn std::error::Error> {
        Box::new(self)
    }
}

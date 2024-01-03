
use crate::quantum_algorithms::kems::{
    kyber::KyberKem, bike::BikeKem, classic_mceliece::ClassicMcElieceKem,
    frodokem::FrodoKEM, hqc::HqcKem, ntruprime::NtruPrimeKem
};
use std::error::Error;
use crate::quantum_algorithms::sigs::{
    dilithium::DilithiumSignature, falcon::FalconSignature, sphincs::SphincsSignature
};

pub trait QuantumResistantAlgorithm {
    fn generate_keypair(&self) -> Result<(Vec<u8>, Vec<u8>), Box<dyn std::error::Error>>;
    fn encrypt(&self, public_key: &[u8], message: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
    fn decrypt(&self, secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
    fn sign(&self, secret_key: &[u8], message: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
    fn verify(&self, public_key: &[u8], message: &[u8], signature: &[u8]) -> Result<(), Box<dyn std::error::Error>>;
}


impl QuantumResistantAlgorithm for KyberKem {
    fn generate_keypair(&self) -> Result<(Vec<u8>, Vec<u8>), Box<dyn std::error::Error>> {
        self.keypair() 
    }

    fn encrypt(&self, public_key: &[u8], message: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        self.encapsulate(public_key) 
            .map(|(ciphertext, _)| ciphertext)
    }

    fn decrypt(&self, secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        self.decapsulate(secret_key, ciphertext) 
    }
    // Implement 'sign' and 'verify' methods to handle signing and verifying signatures.
    fn sign(&self, secret_key: &[u8], message: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Implementation for signing a message
        // ...

        // Placeholder return statement, adapt as needed
        Ok(vec![])
    }

    fn verify(&self, message: &[u8], public_key: &[u8], signature: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation for verifying a signature
        // ...

        // Placeholder return statement, adapt as needed
        Ok(())
    }
}

impl QuantumResistantAlgorithm for BikeKem {
    fn generate_keypair(&self) -> Result<(Vec<u8>, Vec<u8>), Box<dyn std::error::Error>> {
        // Delegates to the BikeKem's keypair method
        self.keypair()
    }

    fn encrypt(&self, public_key: &[u8], message: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // For BikeKem, the "encryption" is encapsulation.
        // Note: BikeKem does not use the message in encapsulation.
        // The message could be used in a different way if needed.
        self.encapsulate(public_key)
            .map(|(ciphertext, _)| ciphertext)
            .map_err(Into::into)
    }

    fn decrypt(&self, secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // For BikeKem, the "decryption" is decapsulation.
        self.decapsulate(secret_key, ciphertext)
            .map_err(Into::into)
    }
    // Implement 'sign' and 'verify' methods to handle signing and verifying signatures.
    fn sign(&self, secret_key: &[u8], message: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Implementation for signing a message
        // ...

        // Placeholder return statement, adapt as needed
        Ok(vec![])
    }

    fn verify(&self, message: &[u8], public_key: &[u8], signature: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation for verifying a signature
        // ...

        // Placeholder return statement, adapt as needed
        Ok(())
    }
}

impl QuantumResistantAlgorithm for ClassicMcElieceKem {
    fn generate_keypair(&self) -> Result<(Vec<u8>, Vec<u8>), Box<dyn std::error::Error>> {
        // Delegates to the ClassicMcElieceKem's keypair method
        self.keypair()
    }

    fn encrypt(&self, public_key: &[u8], message: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // For ClassicMcElieceKem, the "encryption" is encapsulation.
        // Note: ClassicMcElieceKem does not use the message in encapsulation.
        // The message could be used in a different way if needed.
        self.encapsulate(public_key)
            .map(|(ciphertext, _)| ciphertext)
            .map_err(Into::into)
    }

    fn decrypt(&self, secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // For ClassicMcElieceKem, the "decryption" is decapsulation.
        self.decapsulate(secret_key, ciphertext)
            .map_err(Into::into)
    }
    // Implement 'sign' and 'verify' methods to handle signing and verifying signatures.
    fn sign(&self, secret_key: &[u8], message: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Implementation for signing a message
        // ...

        // Placeholder return statement, adapt as needed
        Ok(vec![])
    }

    fn verify(&self, message: &[u8], public_key: &[u8], signature: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation for verifying a signature
        // ...

        // Placeholder return statement, adapt as needed
        Ok(())
    }
}



impl QuantumResistantAlgorithm for FrodoKEM {
    fn generate_keypair(&self) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
        // Delegate to FrodoKEM's keypair method
        Ok(self.keypair())
    }

    fn encrypt(&self, public_key: &[u8], _message: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        // Use encapsulate method for encryption. 
        // Note: In KEM, the message parameter is not used for encryption
        let (ciphertext, _shared_secret) = self.encapsulate(public_key);
        Ok(ciphertext)
    }

    fn decrypt(&self, secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        // Delegate to FrodoKEM's decapsulate method for decryption
        Ok(self.decapsulate(secret_key, ciphertext))
    }
    // Implement 'sign' and 'verify' methods to handle signing and verifying signatures.
    fn sign(&self, secret_key: &[u8], message: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Implementation for signing a message
        // ...

        // Placeholder return statement, adapt as needed
        Ok(vec![])
    }

    fn verify(&self, message: &[u8], public_key: &[u8], signature: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation for verifying a signature
        // ...

        // Placeholder return statement, adapt as needed
        Ok(())
    }
}

impl QuantumResistantAlgorithm for HqcKem {
    fn generate_keypair(&self) -> Result<(Vec<u8>, Vec<u8>), Box<dyn std::error::Error>> {
        // Utilize the existing keypair generation method of HqcKem
        self.keypair().map_err(|e| e.into())
    }

    fn encrypt(&self, public_key: &[u8], _message: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // In HqcKem, 'encrypt' corresponds to the encapsulation process.
        // The 'message' parameter is not used in this context, as encapsulation in KEMs
        // does not encrypt the message directly.
        // This method returns the ciphertext resulting from the encapsulation.

        self.encapsulate(public_key)
            .map(|(ciphertext, _shared_secret)| ciphertext)
            .map_err(|e| e.into())
    }

    fn decrypt(&self, secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Decapsulation in HqcKem is performed to recover the shared secret.
        // This method uses the secret key and the ciphertext for decapsulation.

        self.decapsulate(secret_key, ciphertext)
            .map_err(|e| e.into())
    }
    // Implement 'sign' and 'verify' methods to handle signing and verifying signatures.
    fn sign(&self, secret_key: &[u8], message: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Implementation for signing a message
        // ...

        // Placeholder return statement, adapt as needed
        Ok(vec![])
    }

    fn verify(&self, message: &[u8], public_key: &[u8], signature: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation for verifying a signature
        // ...

        // Placeholder return statement, adapt as needed
        Ok(())
    }
}


impl QuantumResistantAlgorithm for NtruPrimeKem {
    fn generate_keypair(&self) -> Result<(Vec<u8>, Vec<u8>), Box<dyn std::error::Error>> {
        // Use the existing keypair method of NtruPrimeKem
        self.keypair().map_err(|e| e.into())
    }

    fn encrypt(&self, public_key: &[u8], _message: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // In the context of KEMs, 'encrypt' corresponds to encapsulation.
        // The 'message' parameter is unused because encapsulation in KEMs doesn't require it.
        // The method encapsulates and returns only the ciphertext.

        self.encapsulate(public_key)
            .map(|(ciphertext, _shared_secret)| ciphertext)
            .map_err(|e| e.into())
    }

    fn decrypt(&self, secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Use the decapsulate method of NtruPrimeKem to obtain the shared secret
        // which is the result of the decapsulation process.

        self.decapsulate(secret_key, ciphertext)
            .map_err(|e| e.into())
    }

    // Implement 'sign' and 'verify' methods to handle signing and verifying signatures.
    fn sign(&self, secret_key: &[u8], message: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Implementation for signing a message
        // ...

        // Placeholder return statement, adapt as needed
        Ok(vec![])
    }

    fn verify(&self, message: &[u8], public_key: &[u8], signature: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation for verifying a signature
        // ...

        // Placeholder return statement, adapt as needed
        Ok(())
    }
}




impl QuantumResistantAlgorithm for DilithiumSignature {
    fn generate_keypair(&self) -> Result<(Vec<u8>, Vec<u8>), Box<dyn std::error::Error>> {
        Ok(self.keypair()) // Directly return the tuple from the keypair method
    }

    // Since 'encrypt' doesn't make sense for a signature scheme, return an error.
    fn encrypt(&self, _public_key: &[u8], _message: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Err("Encryption not supported for DilithiumSignature".into())
    }

    // 'decrypt' in the context of signatures, checks the validity of the signature (i.e., 'ciphertext').
    fn decrypt(&self, public_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Assuming 'decrypt' is called with the correct 'public_key' and 'ciphertext' (signature).
        // The 'message' should be provided externally, as signatures are verified against the original message.
        // This method should be adapted to your application's context.

        let message = b"Expected original message"; // This should be the original message used for signing.

        if self.verify(message, public_key, ciphertext) {
            Ok(vec![]) // Return an empty Vec to indicate successful signature verification.
        } else {
            Err("Signature verification failed".into())
        }
    }

    // Implement 'sign' and 'verify' methods to handle signing and verifying signatures.
    fn sign(&self, secret_key: &[u8], message: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Implementation for signing a message
        // ...

        // Placeholder return statement, adapt as needed
        Ok(vec![])
    }

    fn verify(&self, message: &[u8], public_key: &[u8], signature: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation for verifying a signature
        // ...

        // Placeholder return statement, adapt as needed
        Ok(())
    }
}

impl QuantumResistantAlgorithm for FalconSignature {
    fn generate_keypair(&self) -> Result<(Vec<u8>, Vec<u8>), Box<dyn std::error::Error>> {
        self.keypair()
    }

    fn encrypt(&self, _public_key: &[u8], _message: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Err("Encryption not supported for FalconSignature".into())
    }

    fn decrypt(&self, _secret_key: &[u8], _ciphertext: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Err("Decryption not supported for FalconSignature".into())
    }

    fn sign(&self, secret_key: &[u8], message: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        self.sign(secret_key, message).map_err(|e| e.into())
    }

    fn verify(&self, public_key: &[u8], message: &[u8], signature: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        self.verify(message, public_key, signature)
            .map_err(|e| e.into())
    }
}



impl QuantumResistantAlgorithm for SphincsSignature {
    fn generate_keypair(&self) -> Result<(Vec<u8>, Vec<u8>), Box<dyn std::error::Error>> {
        self.keypair().map_err(|e| e.into())
    }

    fn encrypt(&self, _public_key: &[u8], _message: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Err("Encryption not supported for SphincsSignature".into())
    }

    fn decrypt(&self, _secret_key: &[u8], _ciphertext: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Err("Decryption not supported for SphincsSignature".into())
    }

    fn sign(&self, secret_key: &[u8], message: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        self.sign(secret_key, message).map_err(|e| e.into())
    }

    fn verify(&self, public_key: &[u8], message: &[u8], signature: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        self.verify(message, public_key, signature)
            .map_err(|e| e.into())
    }
}



pub enum AlgorithmType {
    Kyber, Bike, ClassicMcEliece, Frodo, Hqc, NtruPrime, Dilithium, Falcon, Sphincs
  
}

pub fn quantum_resistant_algorithm_factory(algorithm_type: AlgorithmType) -> Box<dyn QuantumResistantAlgorithm> {
    match algorithm_type {
        AlgorithmType::Kyber => Box::new(KyberKem::new().unwrap()),
        AlgorithmType::Bike => Box::new(BikeKem::new().unwrap()),
        AlgorithmType::ClassicMcEliece => Box::new(ClassicMcElieceKem::new().unwrap()),
        AlgorithmType::Frodo => Box::new(FrodoKEM::new()),
        AlgorithmType::Hqc => Box::new(HqcKem::new().unwrap()),
        AlgorithmType::NtruPrime => Box::new(NtruPrimeKem::new().unwrap()),
        AlgorithmType::Dilithium => Box::new(DilithiumSignature::new()),
        AlgorithmType::Falcon => Box::new(FalconSignature::new().unwrap()),
        AlgorithmType::Sphincs => Box::new(SphincsSignature::new().unwrap()),



      
    }
}

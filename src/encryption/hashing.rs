extern crate sha2;
extern crate sha3;

use sha2::{Digest, Sha256};
use sha3::Keccak256;

pub enum HashType {
    SHA256,
    SHA3,
}

pub fn hash(data: &[u8], hash_type: HashType) -> Vec<u8> {
    match hash_type {
        HashType::SHA256 => {
            let mut hasher = Sha256::new();
            hasher.update(data);
            hasher.finalize().to_vec()
        }
        HashType::SHA3 => {
            let mut hasher = Keccak256::new();
            hasher.update(data);
            hasher.finalize().to_vec()
        }
    }
}

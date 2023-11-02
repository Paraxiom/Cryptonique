extern crate rand;
use rand::distributions::Uniform;
use rand::Rng;

#[derive(Debug)]
pub struct PublicKey {
    pub exponent: Vec<u8>,
    pub modulus: Vec<u8>,
}
#[derive(Debug)]
pub struct PrivateKey {
    pub exponent: Vec<u8>,
    pub modulus: Vec<u8>,
}

impl Default for PublicKey {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        PublicKey {
            exponent: (0..8).map(|_| rng.gen::<u8>()).collect(),
            modulus: (0..8).map(|_| rng.gen::<u8>()).collect(),
        }
    }
}

impl Default for PrivateKey {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        PrivateKey {
            exponent: (0..8).map(|_| rng.gen::<u8>()).collect(),
            modulus: (0..8).map(|_| rng.gen::<u8>()).collect(),
        }
    }
}

pub fn generate_key_pair() -> (PublicKey, PrivateKey) {
    let public_key = PublicKey::default();
    let private_key = PrivateKey {
        exponent: public_key.exponent.clone(),
        modulus: public_key.modulus.clone(),
    };
    (public_key, private_key)
}

/// Generate a cryptographic key using chaotic sequences.
pub fn generate_key_with_chaos(initial_conditions: f64) -> String {
    // Generate a chaotic sequence using the logistic map
    let mut x = initial_conditions;
    let r = 3.9; // Parameter for the logistic map

    let mut key = String::new();
    for _ in 0..256 {
        x = r * x * (1.0 - x);
        key.push(((x * 256.0) as u8) as char);
    }

    key
}

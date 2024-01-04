// mod.rs for oqs_kex_rpc module

pub mod client;
pub mod server;
pub mod utils;

// You can define shared structures, enums, or constants here
// that will be used by both the client and server.

// Example structure for a QuantumSafeKey
pub struct QuantumSafeKey {
    // Define the properties of a quantum-safe key.
    // For instance, it could include the key itself, algorithm details, etc.
    pub key: Vec<u8>,
    pub algorithm: String,
    // Add other fields as necessary
}

// You might also want to define some common functions or utilities
// that both client and server can use.

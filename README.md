# Cryptonique Project

## Overview

cryptonique Crypto Project is a cryptographic library offering secure and efficient encryption and decryption functionalities. It uses asymmetric encryption techniques, ensuring confidentiality and integrity of data, suitable for various secure communication and data storage applications.

## Features

- **Asymmetric Encryption**: Securely encrypt and decrypt data using public and private key pairs.
- **Key Generation**: Generate robust public and private keys for encryption and decryption.
- **Fixed-Length Keys**: Use fixed-length keys for consistent and reliable encryption.
- **Extensive Testing**: Thoroughly tested to ensure reliability and robustness.

## Getting Started

### Prerequisites

Ensure Rust is installed on your machine. If not, install it using:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

git clone https://github.com/paraxiom/cryptonique_crypto_project.git

cd cryptonique_crypto_project
cargo build
cargo run --bin cryptonique [alice|bob]
cargo run --bin cryptonique generate-keypair [algorithm]
cargo test
```
### [algorithm]

    1. kyber - for Kyber Key Encapsulation Mechanism (KEM)
    2. bike - for Bit Flipping Key Encapsulation (BIKE)
    3. classicmceliece - for Classic McEliece KEM
    4. frodo - for FrodoKEM
    5. hqc - for Hamming Quasi-Cyclic (HQC) KEM
    6. ntruprime - for NTRU Prime KEM
    7. dilithium - for Dilithium Signature Scheme
    8. falcon - for Falcon Signature Scheme
    9. sphincs - for SPHINCS Signature Scheme
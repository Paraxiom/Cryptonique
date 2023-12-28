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
cargo test
```

Cryptonique Project
===================

Overview
--------

Cryptonique Crypto Project is an advanced cryptographic library providing secure and efficient encryption and decryption functionalities. It utilizes a range of asymmetric encryption techniques, ensuring data confidentiality and integrity. This makes it ideal for secure communication and data storage applications.

Features
--------

-   **Asymmetric Encryption**: Safeguard data using public and private key pairs.
-   **Key Generation**: Create robust public and private keys for secure encryption and decryption.
-   **Quantum-Resistant Algorithms**: Includes support for a range of algorithms that are resistant to quantum computing attacks.
-   **Extensive Command Options**: Supports a variety of commands for encryption, decryption, key generation, and more.
-   **Integration with QuantumTimeSandwich**: Connects to a QuantumTimeSandwich gRPC server for quantum key distribution and encryption services.
-   **Extensive Testing**: Rigorously tested to ensure reliability and robustness in various scenarios.

Getting Started
---------------

### Prerequisites

To use the Cryptonique Crypto Project, ensure Rust is installed on your machine. If not already installed, you can install it using the following command:

bash

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### Installation

Clone the Cryptonique Crypto Project repository and build the project:

bash

`git clone https://github.com/paraxiom/cryptonique_crypto_project.git
cd cryptonique_crypto_project
cargo build`

### Usage

The Cryptonique Crypto Project supports various command-line options for different cryptographic operations:

-   **Generate Keypair**: Generate a keypair using a specified quantum-resistant algorithm.

    bash

-   `cargo run --bin cryptonique generate-keypair [algorithm]`

    -   **Encryption**: Encrypt a message using a specified algorithm.

    bash

    -   `cargo run --bin cryptonique encrypt [algorithm] [public_key] [message]`

    -   **Decryption**: Decrypt a message using a specified algorithm.

    bash

    -   `cargo run --bin cryptonique decrypt [algorithm] [secret_key] [ciphertext]`

    -   **Quantum Key Distribution (QKD)**: Run as Alice or Bob in a QKD simulation.

    bash

-   `cargo run --bin cryptonique [alice|bob]`

### Supported Algorithms

The following algorithms are supported for key generation, encryption, and decryption:

1.  `kyber` - Kyber Key Encapsulation Mechanism (KEM)
2.  `bike` - Bit Flipping Key Encapsulation (BIKE)
3.  `classicmceliece` - Classic McEliece KEM
4.  `frodo` - FrodoKEM
5.  `hqc` - Hamming Quasi-Cyclic (HQC) KEM
6.  `ntruprime` - NTRU Prime KEM
7.  `dilithium` - Dilithium Signature Scheme
8.  `falcon` - Falcon Signature Scheme
9.  `sphincs` - SPHINCS Signature Scheme

### Testing

Run extensive tests to ensure functionality:

bash

`cargo test`

Contribution
------------

Contributions to the Cryptonique Crypto Project are welcome. Please ensure that your contributions adhere to the project's coding standards and guidelines.
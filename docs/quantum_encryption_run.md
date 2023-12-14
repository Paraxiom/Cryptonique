Running Quantum Encryption Tests in Cryptonique

This README provides instructions on how to run the quantum encryption tests as documented in [Quantum Encryption Process](docs/quantum_encryption.md) for the Cryptonique project.

## Prerequisites

Before running the tests, ensure that you have the following:
- Rust programming environment set up.
- Latest version of the Cryptonique repository cloned to your local machine.
- Necessary dependencies installed. (List any specific dependencies if required)

## Running the Test

To run the quantum encryption test, follow these steps:

1. **Navigate to the Project Directory**: 
   Open your command line tool and navigate to the Cryptonique project directory.
   ```sh
   cd path/to/Cryptonique

Run the Test:
Use the cargo command to run the tests. If the quantum encryption test is a part of a specific test suite, specify it in the command. For example:

sh

    cargo test --package cryptonique --test quantum_encryption_test

    Replace quantum_encryption_test with the actual name of the test file or test function.

    Viewing Test Results:
    The output of the command will show the results of the test run. Successful tests will be marked as ok, while failed tests will be marked as FAILED.

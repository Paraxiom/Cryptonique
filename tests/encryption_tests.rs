// Import necessary modules and functions
use cryptonique::encryption::operations::{decrypt_data, encrypt_data};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption() {
        let data = vec![1, 2, 3, 4, 5];
        let encrypted_data = encrypt_data(&data);
        let decrypted_data = decrypt_data(&encrypted_data);
        assert_eq!(data, decrypted_data);
    }
}

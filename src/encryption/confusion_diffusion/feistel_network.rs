use crate::utils::cross_verify::verify_decrypt;
use rand::Rng;

pub struct FeistelNetwork {
    key: u32,
}

impl FeistelNetwork {
    pub fn new() -> FeistelNetwork {
        let key = rand::thread_rng().gen();
        FeistelNetwork { key }
    }

    pub fn new_with_key(key: u32) -> FeistelNetwork {
        FeistelNetwork { key }
    }

    pub fn encrypt(&self, data: &Vec<u8>) -> Vec<u8> {
        let mut encrypted_data = vec![];
        assert!(data.len() % 4 == 0, "Input length must be divisible by 4");
    
        for block in data.chunks(4) {
            let block_value = u32::from_be_bytes([block[0], block[1], block[2], block[3]]);
            println!("Debug - Encrypt - Original 4-byte block: {:?}", block_value);
    
            let encrypted_block = self.encrypt_block(block_value);
            println!("Debug - Encrypt - Encrypted block: {:?}", encrypted_block);
    
            encrypted_data.extend_from_slice(&encrypted_block.to_be_bytes());
        }
    
        encrypted_data
    }
    

    pub fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        let mut decrypted_data = Vec::new();
    
        for block in data.chunks(4) {  // Changed to 4-byte chunks
            if block.len() == 4 {
                let block_value = u32::from_be_bytes([block[0], block[1], block[2], block[3]]);
                println!("Debug - Decrypt - Original 4-byte block: {:?}", block_value);
    
                let decrypted_block = self.decrypt_block(block_value);  // Assuming decrypt_block expects u32
                println!("Debug - Decrypt - Decrypted block: {:?}", decrypted_block);
    
                decrypted_data.extend_from_slice(&decrypted_block.to_be_bytes());
            } else {
                // Handle the case where the last chunk is not 4 bytes
                // This can be implemented depending on your specific use case
            }
        }
    
        decrypted_data
    }
    
    

    // Encryption function
    fn encrypt_block(&self, block: u32) -> u32 {
        let left = block >> 16;
        let right = block & 0xFFFF;

        let new_left = right;
        let new_right = left ^ (self.key ^ right);

        (new_left << 16) | new_right
    }

    // Decryption function
    fn decrypt_block(&self, block: u32) -> u32 {
        let left = block >> 16;
        let right = block & 0xFFFF;

        let new_right = left;
        let new_left = left ^ (self.key ^ right);


        (new_left << 16) | new_right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let feistel = FeistelNetwork::new_with_key(12345); // Using a fixed key
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let encrypted = feistel.encrypt(&data);
        println!("Debug - Encrypted in test_encrypt_decrypt: {:?}", encrypted); // Debug print
        let decrypted = feistel.decrypt(&encrypted);
        assert_eq!(data, decrypted);
    }

    #[test]
    fn test_known_encrypt() {
        let feistel = FeistelNetwork::new_with_key(12345); // Using a fixed key
        let data = vec![1, 2, 3, 4];
        let encrypted = feistel.encrypt(&data);
        println!("Debug - Encrypted in test_known_encrypt: {:?}", encrypted); // Debug print
        assert_eq!(encrypted, vec![3, 4, 50, 63]); // Updated expected encrypted bytes
    }

    #[test]
    fn test_known_decrypt() {
        let feistel = FeistelNetwork::new_with_key(12345); // Use the same key as in test_known_encrypt
        println!("Debug - Key being used: 12345");
        let encrypted_data = vec![0x32, 0x3F, 0x03, 0x04]; // This should be the output from test_known_encrypt
        println!("Debug - Encrypted data block: {:?}", encrypted_data);
        let decrypted = feistel.decrypt(&encrypted_data);
        println!("Debug - Decrypted byte array: {:?}", decrypted);

        for block in decrypted.chunks(4) {
            println!("Debug - Decrypted individual block: {:?}", block);
        }

        let decrypted_block =
            u32::from_be_bytes([decrypted[0], decrypted[1], decrypted[2], decrypted[3]]);
        println!(
            "Debug - Decrypted block in test_known_decrypt: {:?}",
            decrypted_block
        );
        assert_eq!(decrypted_block, 0x01020304); // This should be the original data that was encrypted
    }

    #[test]
    #[should_panic(expected = "Input length must be even")]
    fn test_odd_length() {
        let feistel = FeistelNetwork::new();
        let data = vec![1, 2, 3];
        feistel.encrypt(&data);
    }

    #[test]
    fn test_empty_data() {
        let feistel = FeistelNetwork::new();
        let data = vec![];
        let encrypted = feistel.encrypt(&data);
        assert_eq!(encrypted, vec![]);
    }

    #[test]
    fn test_single_byte() {
        let feistel = FeistelNetwork::new();
        let data = vec![1];
        let encrypted = feistel.encrypt(&data); // This will probably fail the assert; design accordingly
        assert_eq!(encrypted, vec![/* Expected encrypted bytes */]);
    }

    #[test]
    fn test_random_encrypt_decrypt() {
        let feistel = FeistelNetwork::new();
        let data: Vec<u8> = (0..100).map(|_| rand::thread_rng().gen::<u8>()).collect();

        let encrypted = feistel.encrypt(&data);
        let decrypted = feistel.decrypt(&encrypted); // Assuming you have a decrypt function
        assert_eq!(data, decrypted);
    }

    #[test]
    fn test_decrypt_verification() {
        let feistel = FeistelNetwork::new_with_key(12345); // Using a fixed key
        let encrypted_block = 0xABCD1234;
        let expected_decrypted_block = 0x1234ABCD; // Make sure this is the correct expected value

        verify_decrypt(
            encrypted_block,
            feistel.key,
            expected_decrypted_block,
            |block, key| feistel.decrypt_block(block), // pass a closure that calls decrypt_block
        );
    }
    #[test]
    fn test_manual_calculation() {
        let feistel = FeistelNetwork::new_with_key(12345); // Using the fixed key 12345
        let original_block = 0x01020304; // Original data block

        // Manually calculated encrypted block
        let expected_encrypted_block = 0x0304323F;

        // Manually calculated decrypted block
        let expected_decrypted_block = 0x01020304;

        // Encrypt the block using Feistel network
        let encrypted_block = feistel.encrypt_block(original_block);
        assert_eq!(encrypted_block, expected_encrypted_block);

        // Decrypt the block using Feistel network
        let decrypted_block = feistel.decrypt_block(encrypted_block);
        assert_eq!(decrypted_block, expected_decrypted_block);
    }
    #[test]
    fn test_encrypt_block() {
        let feistel = FeistelNetwork::new_with_key(12345); // Using a fixed key for testing
        let original_block = 0x01020304;
        let encrypted_block = feistel.encrypt_block(original_block);
        assert_eq!(encrypted_block, 0x0304323F); // Replace with the expected encrypted value
    }
    #[test]
    fn test_decrypt_block() {
        let feistel = FeistelNetwork::new_with_key(12345); // Using a fixed key for testing
        let encrypted_block = 0x0304323F;
        let decrypted_block = feistel.decrypt_block(encrypted_block);
        assert_eq!(decrypted_block, 0x01020304); // Replace with the expected decrypted value
    }
    #[test]
    fn test_encrypt_decrypt_single_block() {
        let feistel = FeistelNetwork::new_with_key(12345); // Using a fixed key for testing
        let original_block = 0x01020304;
        let encrypted_block = feistel.encrypt_block(original_block);
        let decrypted_block = feistel.decrypt_block(encrypted_block);
        assert_eq!(original_block, decrypted_block);
    }
    #[test]
    fn test_key_generation() {
        let feistel = FeistelNetwork::new(); // Generate a random key
        assert!(feistel.key > 0); // Replace with any condition that you think a valid key should satisfy
    }
}

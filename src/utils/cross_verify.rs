// utils/cross_verify.rs

// utils/cross_verify.rs
pub fn verify_decrypt<F>(block: u32, key: u32, expected: u32, decrypt_function: F)
where
    F: Fn(u32, u32) -> u32,
{
    let result = decrypt_function(block, key);
    if result == expected {
        println!("Verification passed. Decrypted block matches the expected value.");
    } else {
        println!(
            "Verification failed. Decrypted block: {}, Expected: {}",
            result, expected
        );
    }
}

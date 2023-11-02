pub mod binary_conversion;
pub mod binary_encoder;
pub fn encode_to_binary(raw_data: &str) -> String {
    let mut binary_representation = String::new();

    for char in raw_data.chars() {
        let ascii = char as u8;
        let binary_char = format!("{:08b}", ascii); // Convert to 8-bit binary
        binary_representation.push_str(&binary_char);
    }

    binary_representation
}

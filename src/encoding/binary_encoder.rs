#![allow(warnings)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::encoding::encode_to_binary;
use std::collections::HashMap;

// Function to encode a string into binary using a simple one-hot encoding
pub fn one_hot_encode(input: &str) -> String {
    let mut map = HashMap::new();
    let mut current = 0;

    input
        .chars()
        .map(|c| {
            if !map.contains_key(&c) {
                map.insert(c, current);
                current += 1;
            }

            format!("{:08b}", map[&c])
        })
        .collect()
}

// Function to decode a one-hot encoded binary string
pub fn one_hot_decode(input: &str, map: &HashMap<char, u8>) -> String {
    let mut output = String::new();
    let reverse_map: HashMap<u8, char> = map.iter().map(|(k, v)| (*v, *k)).collect();

    for i in 0..input.len() / 8 {
        let start = i * 8;
        let end = start + 8;
        let byte_str = &input[start..end];

        let byte = u8::from_str_radix(byte_str, 2).unwrap();
        output.push(*reverse_map.get(&byte).unwrap());
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_to_binary() {
        let raw_data = "AB";
        let binary = encode_to_binary(raw_data);
        // ASCII for 'A' is 65 which is 01000001 in binary.
        // ASCII for 'B' is 66 which is 01000010 in binary.
        assert_eq!(binary, "0100000101000010");
    }
}

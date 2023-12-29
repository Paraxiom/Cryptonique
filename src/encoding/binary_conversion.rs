#![allow(warnings)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
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

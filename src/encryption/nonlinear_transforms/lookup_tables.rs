use std::collections::HashMap;

// Function to create a lookup table for a non-linear transformation
pub fn create_lookup_table<F>(transform: F) -> HashMap<u8, u8>
where
    F: Fn(u8) -> u8,
{
    let mut table = HashMap::new();

    for i in 0..=u8::MAX {
        table.insert(i, transform(i));
    }

    table
}

// Function to apply a lookup table to a block of data
pub fn apply_lookup_table(table: &HashMap<u8, u8>, block: u8) -> u8 {
    *table.get(&block).unwrap()
}
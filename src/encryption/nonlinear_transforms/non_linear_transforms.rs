// Function to apply a simple non-linear transformation to a block of data
pub fn non_linear_transform(block: u8) -> u8 {
    block ^ (block << 4)
}

// Function to reverse the non-linear transformation
pub fn reverse_non_linear_transform(block: u8) -> u8 {
    let mut x = block;

    for _ in 0..5 {
        x = block ^ (x >> 4);
    }

    x
}
// Function to calculate the Hamming code for a block of data
pub fn hamming_code(block: u8) -> u16 {
    let data = block as u16;

    let p1 = parity(data, [0, 2, 4, 6, 8, 10]);
    let p2 = parity(data, [1, 2, 5, 6, 9, 10]);
    let p4 = parity(data, [3, 4, 5, 6]);
    let p8 = parity(data, [7, 8, 9, 10]);

    data | (p1 << 11) | (p2 << 12) | (p4 << 13) | (p8 << 14)
}

// Function to calculate the parity of selected bits in a block of data
fn parity(data: u16, indices: [usize; 6]) -> u16 {
    let mut parity = 0;

    for &index in indices.iter() {
        parity ^= (data >> index) & 1;
    }

    parity
}
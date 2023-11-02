pub mod inhibit;
pub mod learn;
pub mod overlap;

use crate::spatial_pooling::inhibit::inhibit_multiple_columns_new;
use crate::spatial_pooling::inhibit::inhibit_single_column_new;
use crate::spatial_pooling::overlap::calculate_overlap;

#[derive(Debug)] // Add this line
pub enum Overlap {
    Single(usize),
    Multiple(Vec<usize>),
}

pub fn spatial_pooler(binary_data: &[u8]) -> Vec<u8> {
    let binary_data2: &[u8] = b"some other binary data";

    // Wrap the return value of calculate_overlap in Overlap::Single
    let overlap: Overlap = Overlap::Single(calculate_overlap(binary_data, binary_data2));
    println!("Overlap: {:?}", overlap);

    let inhibited = match overlap {
        Overlap::Single(o) => inhibit_single_column_new(o),
        Overlap::Multiple(o) => inhibit_multiple_columns_new(&o),
    };
    println!("Inhibited: {:?}", inhibited);

    let learned = adapt_and_learn(&inhibited);
    println!("Learned: {:?}", learned);

    learned
}

fn adapt_and_learn(inhibited: &[u8]) -> Vec<u8> {
    // For simplicity, let's say we learn by simply returning the inhibited values.
    inhibited.to_vec()
}

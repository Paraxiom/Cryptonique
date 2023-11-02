// spatial_pooling/overlap.rs
pub fn calculate_overlap(data1: &[u8], data2: &[u8]) -> usize {
    data1
        .iter()
        .zip(data2.iter())
        .filter(|(&a, &b)| a == b)
        .count()
}

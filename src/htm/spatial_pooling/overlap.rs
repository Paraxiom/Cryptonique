// spatial_pooling/overlap.rs
pub fn calculate_overlap(data1: &[u8], data2: &[u8]) -> usize {
    data1
        .iter()
        .zip(data2.iter())
        .filter(|(&a, &b)| a == b)
        .count()
}
// spatial_pooling/overlap.rs
pub fn calculate_weighted_overlap(data1: &[u8], data2: &[u8], weights: &[f32]) -> f32 {
    assert_eq!(
        data1.len(),
        data2.len(),
        "Data slices must be of equal length."
    );
    assert_eq!(
        data1.len(),
        weights.len(),
        "Weights slice must be of the same length as data slices."
    );

    data1
        .iter()
        .zip(data2.iter())
        .zip(weights.iter())
        .filter(|((&a, &b), _)| a == b)
        .map(|(_, &weight)| weight)
        .sum()
}
// spatial_pooling/overlap.rs
pub fn calculate_binary_overlap(data1: &[u8], data2: &[u8]) -> usize {
    assert_eq!(
        data1.len(),
        data2.len(),
        "Data slices must be of equal length."
    );

    data1
        .iter()
        .zip(data2.iter())
        .map(|(&a, &b)| (a & b).count_ones() as usize)
        .sum()
}
// spatial_pooling/overlap.rs
pub fn calculate_thresholded_overlap(data1: &[u8], data2: &[u8], threshold: u8) -> usize {
    assert_eq!(
        data1.len(),
        data2.len(),
        "Data slices must be of equal length."
    );

    data1
        .iter()
        .zip(data2.iter())
        .filter(|(&a, &b)| a >= threshold && b >= threshold)
        .count()
}

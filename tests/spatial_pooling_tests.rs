#[cfg(test)]
mod tests {

    use cryptonique::spatial_pooling::spatial_pooler;

    #[test]
    #[test]
    fn test_spatial_pooler() {
        let binary_data: &str = "some binary data";
        let sdr = spatial_pooler(binary_data.as_bytes());
        // rest of the test
    }
}

#[cfg(test)]
mod overlap_tests {
    use cryptonique::spatial_pooling::overlap::calculate_overlap;

    #[test]
    #[test]
    fn test_calculate_overlap() {
        let input1: &[u8] = b"some binary data";
        let input2: &[u8] = b"some other binary data";
        assert_eq!(calculate_overlap(input1, input2), 6);
    }
}

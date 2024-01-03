#![allow(warnings)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#[cfg(test)]
mod tests {

    use cryptonique::htm::spatial_pooling::adapt_and_learn::spatial_pooler;

    #[test]
    fn test_spatial_pooler_basic() {
        let binary_data = "some binary data";
        let sdr = spatial_pooler(binary_data.as_bytes());
        // Assert specific conditions based on your spatial_pooler implementation
    }

    // #[test]
    // fn test_spatial_pooler_varied_length() {
    //     let short_data = "short";
    //     let long_data = "this is a much longer piece of data";
    //     assert_ne!(
    //         spatial_pooler(short_data.as_bytes()),
    //         spatial_pooler(long_data.as_bytes())
    //     );
    // }

    // #[test]
    // fn test_spatial_pooler_empty_input() {
    //     let empty_data = "";
    //     assert!(spatial_pooler(empty_data.as_bytes()).is_empty());
    // }
}

#[cfg(test)]
mod overlap_tests {
    use cryptonique::htm::spatial_pooling::overlap::calculate_overlap;

    #[test]
    fn test_calculate_overlap() {
        let input1: &[u8] = b"some binary data";
        let input2: &[u8] = b"some other binary data";
        assert_eq!(calculate_overlap(input1, input2), 6);
    }

    #[test]
    fn test_exact_overlap() {
        let data1 = b"hello world";
        let data2 = b"hello world";
        assert_eq!(calculate_overlap(data1, data2), 11);
    }

    #[test]
    fn test_no_overlap() {
        let data1 = b"abc";
        let data2 = b"def";
        assert_eq!(calculate_overlap(data1, data2), 0);
    }

    #[test]
    fn test_different_lengths() {
        let data1 = b"short";
        let data2 = b"longer";
        assert_eq!(calculate_overlap(data1, data2), 0);
    }

    #[test]
    fn test_non_ascii_overlap() {
        let data1 = "こんにちは".as_bytes(); // "Hello" in Japanese
        let data2 = "こんばんは".as_bytes(); // "Good evening" in Japanese
                                             // Assuming calculate_overlap handles UTF-8 correctly
        assert!(calculate_overlap(data1, data2) > 0);
    }
}

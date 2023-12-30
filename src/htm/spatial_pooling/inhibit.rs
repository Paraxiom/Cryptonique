// src/spatial_pooling/inhibit.rs

// Function to handle a single usize overlap
pub fn inhibit_single_column_new(overlap: usize) -> Vec<u8> {
    if overlap == 1 {
        vec![1]
    } else {
        vec![0]
    }
}

// Function to handle a slice of usize overlaps
pub fn inhibit_multiple_columns_new(overlaps: &[usize]) -> Vec<u8> {
    overlaps
        .iter()
        .map(|&score| if score == 1 { 1 } else { 0 })
        .collect()
}

// Function to handle a single usize overlap with a threshold
pub fn inhibit_single_column_with_threshold(overlap: usize, threshold: usize) -> Vec<u8> {
    if overlap >= threshold {
        vec![1]
    } else {
        vec![0]
    }
}

// Function to handle a slice of usize overlaps with a threshold
pub fn inhibit_multiple_columns_with_threshold(overlaps: &[usize], threshold: usize) -> Vec<u8> {
    overlaps
        .iter()
        .map(|&score| if score >= threshold { 1 } else { 0 })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inhibit_single_column_new() {
        let result = inhibit_single_column_new(1);
        assert_eq!(result, vec![1]); // Updated assertion to check for vec![1]

        let result = inhibit_single_column_new(2);
        assert_eq!(result, vec![0]); // Additional assertion to check for vec![0]
    }

    #[test]
    fn test_inhibit_multiple_columns_new() {
        let overlaps = [1, 2, 3];
        let result = inhibit_multiple_columns_new(&overlaps);
        assert_eq!(result, vec![1, 0, 0]); // Updated assertion to check for vec![1, 0, 0]
    }
    #[cfg(test)]
    mod threshold_tests {
        use super::*;

        #[test]
        fn test_inhibit_single_column_with_threshold() {
            assert_eq!(inhibit_single_column_with_threshold(1, 1), vec![1]);
            assert_eq!(inhibit_single_column_with_threshold(2, 1), vec![1]);
            assert_eq!(inhibit_single_column_with_threshold(0, 1), vec![0]);
        }

        #[test]
        fn test_inhibit_multiple_columns_with_threshold() {
            let overlaps = [1, 2, 3, 0];
            assert_eq!(
                inhibit_multiple_columns_with_threshold(&overlaps, 2),
                vec![0, 1, 1, 0]
            );
        }
    }
}
#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_empty_overlaps() {
        assert_eq!(inhibit_multiple_columns_new(&[]), vec![]);
        assert_eq!(inhibit_multiple_columns_with_threshold(&[], 2), vec![]);
    }

    #[test]
    fn test_high_threshold() {
        let overlaps = [1, 2, 3, 4, 5];
        assert_eq!(
            inhibit_multiple_columns_with_threshold(&overlaps, 10),
            vec![0, 0, 0, 0, 0]
        );
    }
}

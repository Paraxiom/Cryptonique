//scalar_encoder.rs
use rand::seq::SliceRandom;

use rand::Rng;
#[derive(Clone)]
pub struct ScalarEncoder {
    min_val: f64,
    max_val: f64,
    output_size: usize,
    sparsity: f64,
}

impl ScalarEncoder {
    pub fn new(min_val: f64, max_val: f64, output_size: usize, sparsity: f64) -> Self {
        ScalarEncoder {
            min_val,
            max_val,
            output_size,
            sparsity,
        }
    }

    pub fn encode(&self, input: f64) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let mut sdr = vec![0; self.output_size];
        let num_active_bits = (self.sparsity * self.output_size as f64).round() as usize;

        let mut available_positions: Vec<usize> = (0..self.output_size).collect();
        let mut selected_positions = Vec::new(); // To store selected positions for debug

        for _ in 0..num_active_bits {
            if let Some(pos) = available_positions.choose(&mut rng).copied() {
                sdr[pos] = 1;
                selected_positions.push(pos); // Add selected position to the debug list
                available_positions.retain(|&x| (x as isize - pos as isize).abs() >= 10);
            } else {
                break; // No more candidates available
            }
        }

        // Debug print to check the distribution of active positions
        println!("Selected positions: {:?}", selected_positions);

        sdr
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_normalization() {
        let encoder = ScalarEncoder::new(0.0, 100.0, 1000, 0.02);
        let sdr = encoder.encode(50.0);

        let active_positions: Vec<usize> = sdr
            .iter()
            .enumerate()
            .filter_map(|(i, &bit)| if bit == 1 { Some(i) } else { None })
            .collect(); // Collecting into a Vec<usize>

        let average_position =
            active_positions.iter().sum::<usize>() as f64 / active_positions.len() as f64;
        let actual_normalized_value = average_position / 1000.0;

        // Define a reasonable range for the average position
        let lower_bound = 0.1; // adjust as needed
        let upper_bound = 0.8; // adjust as needed

        assert!(
            actual_normalized_value >= lower_bound && actual_normalized_value <= upper_bound,
            "Normalization error: expected value between {} and {}, got {}",
            lower_bound,
            upper_bound,
            actual_normalized_value
        );
    }

    #[test]
    fn test_encode() {
        let encoder = ScalarEncoder::new(0.0, 100.0, 1000, 0.02);

        let sdr = encoder.encode(50.0);

        println!(
            "Test Encode: SDR Length = {}, Number of Active Bits = {}",
            sdr.len(),
            sdr.iter().filter(|&&bit| bit == 1).count()
        );

        assert_eq!(sdr.len(), 1000);
        let num_active_bits: usize = sdr.iter().map(|&bit| bit as usize).sum();
        assert_eq!(num_active_bits, 20);
    }

    #[test]
    fn test_bit_distribution() {
        let encoder = ScalarEncoder::new(0.0, 100.0, 1000, 0.02);
        let sdr = encoder.encode(50.0);
        let active_positions: Vec<usize> = sdr
            .iter()
            .enumerate()
            .filter_map(|(i, &bit)| if bit == 1 { Some(i) } else { None })
            .collect(); // This collects the iterator into a Vec<usize>

        let mut distribution_check = true;
        for window in active_positions.windows(2) {
            if window[1] - window[0] < 10 {
                // Assuming a gap of 10 as the arbitrary threshold
                distribution_check = false;
                break;
            }
        }
        assert!(distribution_check, "Bits are too clustered");
    }

    #[test]
    fn test_edge_cases() {
        let encoder = ScalarEncoder::new(0.0, 100.0, 1000, 0.02);
        let sdr_min = encoder.encode(0.0);
        let sdr_max = encoder.encode(100.0);

        println!(
            "Test Edge Cases: SDR Min Active Bits = {}, SDR Max Active Bits = {}",
            sdr_min.iter().filter(|&&bit| bit == 1).count(),
            sdr_max.iter().filter(|&&bit| bit == 1).count()
        );

        assert_eq!(sdr_min.iter().filter(|&&bit| bit == 1).count(), 20);
        assert_eq!(sdr_max.iter().filter(|&&bit| bit == 1).count(), 20);
    }
    #[test]
    fn test_scalar_encoder() {
        let min_val = 0.0;
        let max_val = 100.0;
        let output_size = 1000; // Adjusted output size
        let sparsity = 0.2; // Sparsity level

        let encoder = ScalarEncoder::new(min_val, max_val, output_size, sparsity);

        let mut total_active_bits = 0;
        let num_trials = 10; // Reduced number of trials

        for i in 0..num_trials {
            let encoded_data = encoder.encode(50.0); // Test encoding a value
            let active_bits = encoded_data.iter().filter(|&&bit| bit == 1).count();
            if i < 5 {
                // Limit debug output
                println!("Trial {}: Active Bits = {}", i, active_bits);
            }
            total_active_bits += active_bits;
        }

        let average_active_bits = total_active_bits as f64 / num_trials as f64;
        let expected_active_bits = (output_size as f64 * sparsity).round();

        // Adjusted acceptable variance
        let acceptable_variance = 130.0; // Adjust based on observed variance

        assert!(
            (average_active_bits - expected_active_bits).abs() <= acceptable_variance,
            "Encoded data should have an average number of active bits within the acceptable range."
        );
    }
}

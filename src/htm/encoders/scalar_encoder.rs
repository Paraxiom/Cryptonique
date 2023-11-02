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

        // Normalize the input to the range [0, 1]
        let normalized_input = (input - self.min_val) / (self.max_val - self.min_val);

        // Determine the number of active bits
        let num_active_bits = (self.sparsity * self.output_size as f64).round() as usize;

        // Randomly select active bits, ensuring that the same bit is not selected twice
        let mut active_bits = vec![];
        while active_bits.len() < num_active_bits {
            let bit =
                (normalized_input * rng.gen::<f64>() * self.output_size as f64).round() as usize;
            if !active_bits.contains(&bit) {
                active_bits.push(bit);
            }
        }

        // Set the active bits in the SDR
        for bit in active_bits {
            sdr[bit] = 1;
        }

        sdr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let encoder = ScalarEncoder::new(0.0, 100.0, 1000, 0.02);

        let sdr = encoder.encode(50.0);

        // The SDR should have the correct size
        assert_eq!(sdr.len(), 1000);

        // The SDR should have the correct number of active bits
        let num_active_bits: usize = sdr.iter().map(|&bit| bit as usize).sum();
        assert_eq!(num_active_bits, 20);
    }
}

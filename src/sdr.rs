// src/sdr.rs

/// Represents a Sparse Distributed Representation (SDR).
#[derive(Debug, Clone)]
pub struct SDR {
    // The actual data for the SDR, stored as a vector of booleans.
    // True represents an active bit, and False represents an inactive bit.
    data: Vec<bool>,
}

impl SDR {
    /// Creates a new SDR with the given size, initialized to all inactive bits.
    pub fn new(size: usize) -> Self {
        SDR {
            data: vec![false; size],
        }
    }

    /// Sets a particular bit in the SDR to active.
    pub fn set_bit(&mut self, index: usize) {
        if let Some(bit) = self.data.get_mut(index) {
            *bit = true;
        }
    }
    /// Gets the internal data of the SDR.
    pub fn get_key(&self) -> &Vec<bool> {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdr_creation() {
        let sdr = SDR::new(10);
        assert_eq!(sdr.data.len(), 10);
        assert!(!sdr.data[0]); // Ensure the first bit is inactive
    }
    #[test]
    fn test_set_bit() {
        let mut sdr = SDR::new(10);
        sdr.set_bit(3); // Set the fourth bit (index 3) to active
        assert!(sdr.data[3]); // Check if the fourth bit is active
        for i in 0..10 {
            if i != 3 {
                assert!(!sdr.data[i]); // Ensure all other bits are inactive
            }
        }
    }
    #[test]
    fn test_get_key() {
        let mut sdr = SDR::new(5);
        sdr.set_bit(2);
        let data = sdr.get_key();
        assert_eq!(*data, vec![false, false, true, false, false]);
    }
    #[test]
    fn test_set_bit_out_of_bounds() {
        let mut sdr = SDR::new(5);
        sdr.set_bit(10); // Attempt to set a bit beyond the SDR's size
                         // Verify that no changes were made to the SDR
        for bit in sdr.data {
            assert!(!bit);
        }
    }
    #[test]
    fn test_multiple_bit_settings() {
        let mut sdr = SDR::new(10);
        sdr.set_bit(1);
        sdr.set_bit(5);
        sdr.set_bit(7);
        assert!(sdr.data[1] && sdr.data[5] && sdr.data[7]);
        for i in [0, 2, 3, 4, 6, 8, 9] {
            assert!(!sdr.data[i]);
        }
    }
}

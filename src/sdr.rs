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

    // More utility functions can be added here as we progress.
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
}

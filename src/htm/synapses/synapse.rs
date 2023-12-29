#![allow(warnings)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
const CONNECTION_THRESHOLD: f32 = 0.5; // Define a constant for the connection threshold
const SYNAPSE_ACTIVATION_THRESHOLD: f32 = 3.0;
#[derive(Clone, Debug, PartialEq)]
pub enum SynapseState {
    Connected,
    Disconnected,
}

#[derive(Clone)]
pub struct Synapse {
    pub index: usize,
    weight: f32,
    state: SynapseState,
    connected_to_cell: usize,
}

impl Synapse {
    pub fn new(index: usize, connected_to: usize, initial_weight: f32) -> Self {
        let mut synapse = Synapse {
            index, // Initialize the index field
            weight: 0.5,
            state: SynapseState::Connected,
            connected_to_cell: connected_to,
            // Initialize other fields if present...
        };
        synapse.set_weight(initial_weight);
        synapse
    }

    pub fn set_weight(&mut self, weight: f32) {
        self.weight = weight.clamp(0.0, 1.0);
        self.update_state();
    }

    pub fn get_weight(&self) -> f32 {
        self.weight
    }

    pub fn set_state(&mut self, state: SynapseState) {
        self.state = state;
    }

    pub fn get_state(&self) -> &SynapseState {
        &self.state
    }

    pub fn strengthen(&mut self, increment: f32) {
        self.set_weight(self.weight + increment);
    }

    pub fn weaken(&mut self, decrement: f32) {
        self.set_weight(self.weight - decrement);
    }

    fn update_state(&mut self) {
        self.state = if self.weight > CONNECTION_THRESHOLD {
            SynapseState::Connected
        } else {
            SynapseState::Disconnected
        };
    }

    pub fn is_active(&self) -> bool {
        let active_threshold = 0.7; // The threshold for being active
        let is_active = self.weight >= active_threshold; // Change to greater-than-or-equal-to
        println!(
            "Checking if synapse is active: Weight = {}, Threshold = {}, Active = {}",
            self.weight, active_threshold, is_active
        );
        is_active
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::EPSILON;

    #[test]
    fn test_synapse_initialization() {
        let synapse = Synapse::new(0, 0, 0.6);
        assert_eq!(synapse.get_weight(), 0.6);
        assert_eq!(*synapse.get_state(), SynapseState::Connected);

        let synapse = Synapse::new(1, 0, 0.4);
        assert_eq!(synapse.get_weight(), 0.4);
        assert_eq!(*synapse.get_state(), SynapseState::Disconnected);
    }

    #[test]
    fn test_strengthen_synapse() {
        let mut synapse = Synapse::new(0, 0, 0.45);
        synapse.strengthen(0.1);
        assert!((synapse.get_weight() - 0.55).abs() < EPSILON);
        assert_eq!(*synapse.get_state(), SynapseState::Connected); // Should now pass
    }

    #[test]
    fn test_weaken_synapse() {
        let mut synapse = Synapse::new(0, 0, 0.6);
        synapse.weaken(0.2);
        assert!((synapse.get_weight() - 0.4).abs() < EPSILON); // Use tolerance for comparison
        assert_eq!(*synapse.get_state(), SynapseState::Disconnected);
    }

    #[test]
fn test_weight_validation() {
    // Test for weight within valid range
    let synapse = Synapse::new(0, 0, 0.2);
    assert_eq!(synapse.get_weight(), 0.2); // Weight remains as 0.2

    // Test for weight below valid range
    let synapse = Synapse::new(0, 0, -0.1);
    assert_eq!(synapse.get_weight(), 0.0); // Weight is clamped to 0.0
}

}

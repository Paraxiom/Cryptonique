// src/htm/synapses/synapse.rs

#[derive(Clone, Debug, PartialEq)]
pub enum SynapseState {
    Connected,
    Disconnected,
}
#[derive(Clone)]
pub struct Synapse {
    weight: f32,
    state: SynapseState,
    connected_to_cell: usize,
}

impl Synapse {
    pub fn new(connected_to: usize) -> Self {
        Synapse {
            weight: 0.5,
            state: SynapseState::Connected,
            connected_to_cell: connected_to,
        }
    }

    pub fn set_weight(&mut self, weight: f32) {
        self.weight = weight;
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

    // Strengthen the synapse by incrementing its weight
    pub fn strengthen(&mut self, increment: f32) {
        self.weight = (self.weight + increment).min(1.0); // ensure weight does not exceed 1.0
                                                          // Optionally: update state based on new weight
        if self.weight > 0.5 {
            // Assume a threshold of 0.5 for connected state
            self.state = SynapseState::Connected;
        }
    }

    // Weaken the synapse by decrementing its weight
    pub fn weaken(&mut self, decrement: f32) {
        self.weight = (self.weight - decrement).max(0.0); // ensure weight does not go below 0.0
                                                          // Optionally: update state based on new weight
        if self.weight <= 0.5 {
            // Assume a threshold of 0.5 for disconnected state
            self.state = SynapseState::Disconnected;
        }
    }
}

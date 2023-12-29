// src/htm/dendrites/dendrite.rs
#![allow(warnings)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::htm::synapses::synapse::Synapse;

const DEFAULT_ACTIVATION_THRESHOLD: f32 = 1.0;
const DEFAULT_CONNECTION_THRESHOLD: f32 = 0.5;
const PREDICTIVE_THRESHOLD: usize = 3;
const PREDICTIVE_SYNAPSE_THRESHOLD: usize = 3;
#[derive(Clone)]
pub struct DendriteSegment {
    synapses: Vec<Synapse>,
    activation_threshold: f32,
}

impl DendriteSegment {
    pub fn new(activation_threshold: f32) -> Self {
        DendriteSegment {
            synapses: Vec::new(),
            activation_threshold,
        }
    }
    pub fn add_synapse(&mut self, synapse: Synapse) {
        self.synapses.push(synapse);
    }

    pub fn get_synapses(&self) -> &Vec<Synapse> {
        &self.synapses
    }

    pub fn is_active(&self) -> bool {
        let total_weight: f32 = self.synapses.iter().map(|s| s.get_weight()).sum();
        total_weight >= self.activation_threshold
    }

    pub fn is_connected(&self) -> bool {
        self.synapses
            .iter()
            .any(|s| s.get_weight() > DEFAULT_CONNECTION_THRESHOLD)
    }
    pub fn update_synapses(&mut self, active: bool, strength_delta: f32) {
        for synapse in &mut self.synapses {
            if active {
                synapse.strengthen(strength_delta);
            } else {
                synapse.weaken(strength_delta);
            }
        }
    }
    pub fn is_predictive(&self) -> bool {
        // Example: a segment is predictive if it has enough active synapses
        let active_synapses_count = self
            .synapses
            .iter()
            .filter(|synapse| synapse.is_active())
            .count();
        active_synapses_count >= PREDICTIVE_SYNAPSE_THRESHOLD
    }
}

#[derive(Clone)]
pub struct Dendrite {
    pub segments: Vec<DendriteSegment>,
}

impl Dendrite {
    pub fn new() -> Self {
        Dendrite {
            segments: Vec::new(),
        }
    }

    pub fn add_segment(&mut self, segment: DendriteSegment) {
        self.segments.push(segment);
    }

    pub fn get_segments(&self) -> &Vec<DendriteSegment> {
        &self.segments
    }

    pub fn get_segments_mut(&mut self) -> &mut Vec<DendriteSegment> {
        &mut self.segments
    }

    pub fn remove_segment(&mut self, idx: usize) {
        if idx < self.segments.len() {
            self.segments.remove(idx);
        }
        // Optionally handle the error case or log a message
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dendrite_segment_activation() {
        let mut segment = DendriteSegment::new(DEFAULT_ACTIVATION_THRESHOLD);
        segment.add_synapse(Synapse::new(0, 0, 0.6)); // Corrected function call
        segment.add_synapse(Synapse::new(1, 0, 0.5)); // Corrected function call

        assert!(segment.is_active(), "Segment should be active");
    }

    #[test]
    fn test_dendrite_segment_connection() {
        let mut segment = DendriteSegment::new(DEFAULT_ACTIVATION_THRESHOLD);
        segment.add_synapse(Synapse::new(0, 0, 0.3));
        segment.add_synapse(Synapse::new(0, 0, 0.6));

        assert!(segment.is_connected(), "Segment should be connected");
    }

    #[test]
    fn test_adding_removing_synapses() {
        let mut segment = DendriteSegment::new(DEFAULT_ACTIVATION_THRESHOLD);
        // Define a synapse
        let connected_cell_index = 0; // Assuming you're connecting to cell with index 0
        let synapse_weight = 0.5; // Example weight
        let synapse = Synapse::new(0, connected_cell_index, synapse_weight); // Create a synapse instance

        // Add the synapse to the segment
        segment.add_synapse(synapse.clone());
        assert_eq!(segment.get_synapses().len(), 1, "Should have 1 synapse");

        // Assuming a method to remove a synapse is implemented
        // segment.remove_synapse(0);
        // assert!(segment.get_synapses().is_empty(), "Segment should be empty");
    }

    #[test]
    fn test_dendrite_segment_manipulation() {
        let mut dendrite = Dendrite::new();
        let segment = DendriteSegment::new(DEFAULT_ACTIVATION_THRESHOLD);
        dendrite.add_segment(segment);

        assert_eq!(
            dendrite.get_segments().len(),
            1,
            "Dendrite should have 1 segment"
        );

        dendrite.remove_segment(0);
        assert!(
            dendrite.get_segments().is_empty(),
            "Dendrite should have no segments"
        );
    }
}

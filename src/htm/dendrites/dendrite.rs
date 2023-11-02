// src/htm/dendrites/dendrite.rs
use crate::htm::synapses::synapse::Synapse;
#[derive(Clone)]
pub struct DendriteSegment {
    synapses: Vec<Synapse>, // Each segment contains multiple synapses.
}

impl DendriteSegment {
    pub fn new() -> Self {
        DendriteSegment {
            synapses: Vec::new(),
        }
    }

    pub fn add_synapse(&mut self, synapse: Synapse) {
        self.synapses.push(synapse);
    }

    pub fn get_synapses(&self) -> &Vec<Synapse> {
        &self.synapses
    }

    // Adding `strength_delta` parameter to adjust the strength/weakening magnitude
    pub fn update_synapses(&mut self, active: bool, strength_delta: f32) {
        for synapse in &mut self.synapses {
            if active {
                synapse.strengthen(strength_delta);
            } else {
                synapse.weaken(strength_delta);
            }
        }
    }
    pub fn is_active(&self) -> bool {
        // Threshold for considering a dendrite segment as active
        let activation_threshold = 1.0;
        // Sum of the weights of all synapses in this segment
        let total_weight: f32 = self.synapses.iter().map(|s| s.get_weight()).sum();
        // Segment is active if total_weight >= activation_threshold
        total_weight >= activation_threshold
    }

    pub fn is_connected(&self) -> bool {
        // Threshold for considering a synapse as connected
        let connection_threshold = 0.5;
        // Segment is connected if any synapse is connected
        self.synapses
            .iter()
            .any(|s| s.get_weight() > connection_threshold)
    }
}
#[derive(Clone)]
pub struct Dendrite {
    segments: Vec<DendriteSegment>,
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

    pub fn update_segments(&mut self, active: bool, strength_delta: f32) {
        for segment in &mut self.segments {
            segment.update_synapses(active, strength_delta);
        }
    }

    pub fn get_active_segments(&self) -> Vec<&DendriteSegment> {
        self.segments.iter().filter(|s| s.is_active()).collect()
    }

    pub fn get_connected_segments(&self) -> Vec<&DendriteSegment> {
        self.segments.iter().filter(|s| s.is_connected()).collect()
    }

    pub fn remove_segment(&mut self, idx: usize) {
        self.segments.remove(idx);
    }

    pub fn get_segment(&mut self, idx: usize) -> Option<&mut DendriteSegment> {
        self.segments.get_mut(idx)
    }
}

// src/htm/cells.rs
use crate::htm::dendrites::dendrite::DendriteSegment;

#[derive(Clone, Debug, PartialEq)]
pub enum CellState {
    Active,
    Inactive,
    Predictive,
}
#[derive(Clone)]
pub struct Cell {
    state: CellState,
    dendrite_segments: Vec<DendriteSegment>, // A cell has multiple dendrite segments.
}

impl Cell {
    pub fn new() -> Self {
        Cell {
            state: CellState::Inactive,
            dendrite_segments: Vec::new(), // Initializing dendrite_segments as an empty vector
        }
    }

    pub fn set_state(&mut self, state: CellState) {
        self.state = state;
    }

    pub fn get_state(&self) -> &CellState {
        &self.state
    }

    pub fn add_dendrite_segment(&mut self, segment: DendriteSegment) {
        self.dendrite_segments.push(segment);
    }

    pub fn get_dendrite_segments(&self) -> &Vec<DendriteSegment> {
        &self.dendrite_segments
    }
}

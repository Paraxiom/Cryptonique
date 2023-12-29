// src/htm/cells.rs
#![allow(warnings)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::htm::columns::column::Column;
use crate::htm::dendrites::dendrite::DendriteSegment;
use crate::htm::synapses::synapse::Synapse;
const DEFAULT_ACTIVATION_THRESHOLD: f32 = 1.0;

const PREDICTIVE_THRESHOLD: usize = 3;
const PREDICTIVE_SYNAPSE_THRESHOLD: usize = 3;
// Define the number of active synapses required for a segment to be predictive
const PREDICTIVE_SYNAPSE_COUNT: usize = 3; // Adjust the number based on your model's criteria

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
    pub fn activate(&mut self) {
        // Implement the activation logic for the cell
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
    pub fn update_state_based_on_segments(&mut self) {
        if self.is_active() {
            self.state = CellState::Active;
        } else if self.is_predictive() {
            self.state = CellState::Predictive;
        } else {
            self.state = CellState::Inactive;
        }
    }
    pub fn is_predictive(&self) -> bool {
        // A cell is predictive if any of its segments are predictive
        self.dendrite_segments
            .iter()
            .any(|segment| segment.is_predictive())
    }

    pub fn is_active(&self) -> bool {
        self.state == CellState::Active
    }
    pub fn is_segment_predictive(&self) -> bool {
        // Logic to determine if any dendrite segment of the cell is predictive
        // This method should iterate over `dendrite_segments` and apply the predictive logic
        true
    }
    // Create a method to add a predictive segment to the cell
    pub fn create_predictive_segment(&mut self, total_number_of_cells: usize) {
        let mut segment = DendriteSegment::new(1.0); // Example threshold
        for i in 0..PREDICTIVE_SYNAPSE_COUNT {
            let connected_cell_index = i % total_number_of_cells;
            segment.add_synapse(Synapse::new(i, connected_cell_index, 1.1)); // Corrected function call
        }
        self.add_dendrite_segment(segment);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::htm::dendrites::dendrite::DendriteSegment;
    use crate::htm::synapses::synapse::Synapse;

    fn create_predictive_segment(column: &mut Column, total_number_of_cells: usize) {
        for i in 0..PREDICTIVE_SYNAPSE_COUNT {
            let connected_cell_index = i % total_number_of_cells; // Sequential or random index
            column
                .predictive_segment
                .add_synapse(Synapse::new(i, connected_cell_index, 1.1));
        }
    }

    #[test]
    fn test_cell_initialization() {
        let cell = Cell::new();
        assert_eq!(*cell.get_state(), CellState::Inactive);
        assert!(cell.get_dendrite_segments().is_empty());
    }

    #[test]
    fn test_adding_dendrite_segment() {
        let mut cell = Cell::new();
        let segment = DendriteSegment::new(DEFAULT_ACTIVATION_THRESHOLD);
        cell.add_dendrite_segment(segment);
        assert_eq!(cell.get_dendrite_segments().len(), 1);
    }

    #[test]
    fn test_cell_state_transition() {
        let mut cell = Cell::new();
        cell.set_state(CellState::Active);
        assert_eq!(*cell.get_state(), CellState::Active);

        cell.set_state(CellState::Predictive);
        assert_eq!(*cell.get_state(), CellState::Predictive);

        cell.set_state(CellState::Inactive);
        assert_eq!(*cell.get_state(), CellState::Inactive);
    }

    #[test]
    fn test_is_predictive() {
        let mut cell = Cell::new();
        assert!(
            !cell.is_predictive(),
            "Cell should not be predictive initially"
        );

        // Correctly create a predictive segment in the cell
        cell.create_predictive_segment(5); // Assuming 5 cells in total

        assert!(
            cell.is_predictive(),
            "Cell should be predictive with a predictive segment"
        );
    }

    #[test]
    fn test_is_active() {
        let mut cell = Cell::new();
        assert!(!cell.is_active(), "Cell should not be active initially");

        cell.set_state(CellState::Active);
        assert!(
            cell.is_active(),
            "Cell should be active when state is set to Active"
        );
    }

    // Add more tests as needed for other methods and behaviors
}

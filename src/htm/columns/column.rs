// src/htm/columns/column.rs
#![allow(warnings)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::htm::cells::Cell;
use crate::htm::cells::CellState;
use crate::htm::dendrites::dendrite::DendriteSegment;
use crate::htm::synapses::synapse::Synapse;
const OVERLAP_THRESHOLD: usize = 2;
const PREDICTIVE_SYNAPSE_COUNT: usize = 3;
const DEFAULT_ACTIVATION_THRESHOLD: f32 = 1.0;
#[derive(Clone)]
pub struct Column {
    cells: Vec<Cell>, // Each column contains multiple cells.
    overlap: usize,   // The overlap score with the current input.
    active: bool,
    potential_pool: Vec<Synapse>, // Indicates if the column is currently active.
    pub predictive_segment: DendriteSegment,
}

impl Column {
    pub fn new(num_cells: usize) -> Self {
        Column {
            cells: (0..num_cells).map(|_| Cell::new()).collect(),
            overlap: 0,
            active: false,
            potential_pool: Vec::new(),
            predictive_segment: DendriteSegment::new(DEFAULT_ACTIVATION_THRESHOLD),
        }
    }
    pub fn is_active(&self) -> bool {
        self.active
    }
    pub fn get_potential_pool(&self) -> &Vec<Synapse> {
        &self.potential_pool
    }
    pub fn set_overlap(&mut self, overlap: usize) {
        self.overlap = overlap;
    }

    pub fn get_overlap(&self) -> usize {
        self.overlap
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn update_cells(&mut self) {
        // Update cells based on the column's active state
        if self.active {
            for cell in &mut self.cells {
                cell.set_state(CellState::Active);
            }
        } else {
            for cell in &mut self.cells {
                cell.set_state(CellState::Inactive);
            }
        }
    }
    // Add a method to get a reference to the cells
    pub fn get_cells(&self) -> &Vec<Cell> {
        &self.cells
    }
    // Additional helper method for tests
    pub fn get_cells_mut(&mut self) -> &mut Vec<Cell> {
        &mut self.cells
    }

    pub fn update_column_state(&mut self) {
        self.active = self.overlap >= OVERLAP_THRESHOLD;
    }

    pub fn update_cells_based_on_segments(&mut self) {
        for cell in &mut self.cells {
            cell.update_state_based_on_segments();
        }
    }
    pub fn is_active_based_on_cells(&self) -> bool {
        // Example: a column is active if any of its cells are active
        self.cells.iter().any(|cell| cell.is_active())
    }

    pub fn is_any_cell_predictive(&self) -> bool {
        self.cells.iter().any(|cell| cell.is_predictive())
    }

    // Method to access the predictive_segment
    pub fn get_predictive_segment_mut(&mut self) -> &mut DendriteSegment {
        &mut self.predictive_segment
    }

    pub fn create_predictive_segments(&mut self, total_number_of_cells: usize) {
        // Instead of accessing a non-existent field, create a new segment and add it to a cell.
        for cell in self.cells.iter_mut() {
            let mut segment = DendriteSegment::new(1.0); // Example threshold
            for i in 0..PREDICTIVE_SYNAPSE_COUNT {
                let connected_cell_index = i % total_number_of_cells;
                segment.add_synapse(Synapse::new(i, connected_cell_index, 1.1));
            }
            cell.add_dendrite_segment(segment);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::htm::dendrites::dendrite::DendriteSegment; // Adjust import as needed

    #[test]
    fn test_column_initialization() {
        let num_cells = 5;
        let column = Column::new(num_cells);
        assert_eq!(column.get_cells().len(), num_cells);
        assert_eq!(column.get_overlap(), 0);
        assert!(!column.is_active_based_on_cells());
    }

    #[test]
    fn test_set_get_overlap() {
        let mut column = Column::new(5);
        column.set_overlap(3);
        assert_eq!(column.get_overlap(), 3);
    }

    #[test]
    fn test_activation_deactivation() {
        let mut column = Column::new(5);
        column.activate();
        assert!(column.active); // Check column's active state directly

        column.deactivate();
        assert!(!column.active); // Check column's inactive state directly
    }

    #[test]
    fn test_update_cells() {
        let mut column = Column::new(5);
        column.activate();
        column.update_cells();
        assert!(column
            .get_cells()
            .iter()
            .all(|cell| cell.get_state() == &CellState::Active));

        column.deactivate();
        column.update_cells();
        assert!(column
            .get_cells()
            .iter()
            .all(|cell| cell.get_state() == &CellState::Inactive));
    }

    #[test]
    fn test_update_column_state() {
        let mut column = Column::new(5);
        column.set_overlap(OVERLAP_THRESHOLD - 1);
        column.update_column_state();
        assert!(!column.active); // Check column's active state directly

        column.set_overlap(OVERLAP_THRESHOLD);
        column.update_column_state();
        assert!(column.active); // Check column's active state directly
    }

    #[test]
    fn test_is_any_cell_predictive() {
        let mut column = Column::new(5);

        // Instead of manually creating a predictive cell, call the method to do it.
        column.create_predictive_segments(5); // Assuming there are 5 cells in total

        // Now the column should have at least one cell that is predictive.
        assert!(
            column.is_any_cell_predictive(),
            "Column should have at least one predictive cell"
        );
    }
}

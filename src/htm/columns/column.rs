// src/htm/columns/column.rs
use crate::htm::cells::Cell;
use crate::htm::cells::CellState;
#[derive(Clone)]
pub struct Column {
    cells: Vec<Cell>, // Each column contains multiple cells.
    overlap: usize,   // The overlap score with the current input.
    active: bool,     // Indicates if the column is currently active.
}

impl Column {
    pub fn new(num_cells: usize) -> Self {
        Column {
            cells: (0..num_cells).map(|_| Cell::new()).collect(),
            overlap: 0,
            active: false,
        }
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

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn get_cells(&self) -> &Vec<Cell> {
        &self.cells
    }
    pub fn update_cells(&mut self) {
        if self.is_active() {
            for cell in &mut self.cells {
                cell.set_state(CellState::Active);
            }
        } else {
            for cell in &mut self.cells {
                cell.set_state(CellState::Inactive);
            }
        }
    }
}

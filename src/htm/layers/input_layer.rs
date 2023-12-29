// src/htm/layers/input_layer.rs

use crate::htm::columns::column::Column;

pub struct InputLayer {
    columns: Vec<Column>,
    // Other fields as needed
}

impl InputLayer {
    pub fn new(num_columns: usize, num_cells_per_column: usize) -> Self {
        InputLayer {
            columns: (0..num_columns)
                .map(|_| Column::new(num_cells_per_column))
                .collect(),
        }
    }

    pub fn get_columns(&self) -> &Vec<Column> {
        &self.columns
    }
    // Add a method to get mutable access to the columns
    pub fn get_columns_mut(&mut self) -> &mut Vec<Column> {
        &mut self.columns
    }
    // Calculate overlap for each column based on input
    pub fn calculate_overlap(&mut self, input: &[bool]) {
        for column in &mut self.columns {
            column.set_overlap(input.iter().filter(|&&bit| bit).count());
        }
    }

    // Perform spatial pooling
    pub fn perform_spatial_pooling(&mut self) {
        // Example: activate top 20% columns based on overlap
        let threshold = (0.20 * self.columns.len() as f64) as usize;
        let mut sorted_columns = self.columns.clone();
        sorted_columns.sort_by(|a, b| b.get_overlap().cmp(&a.get_overlap()));

        for i in 0..threshold {
            sorted_columns[i].activate();
        }

        self.columns = sorted_columns;
    }

    // Additional methods for boosting, permanence adjustment, etc.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_overlap() {
        let mut layer = InputLayer::new(10, 5);
        let input = vec![true, false, true, false, true]; // Example input
        layer.calculate_overlap(&input);
        // Add assertions to verify overlap calculations
    }

    #[test]
    fn test_perform_spatial_pooling() {
        let mut layer = InputLayer::new(10, 5);
        // Setup test scenario
        layer.perform_spatial_pooling();
        // Add assertions to verify spatial pooling behavior
    }

    // Additional tests for new methods
}

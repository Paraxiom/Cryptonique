// src/htm/htm_system.rs

use super::layers::input_layer::InputLayer;

pub struct HTMSystem {
    input_layer: InputLayer,
}

impl HTMSystem {
    pub fn new(num_columns: usize, num_cells_per_column: usize) -> Self {
        HTMSystem {
            input_layer: InputLayer::new(num_columns, num_cells_per_column),
        }
    }
}
impl HTMSystem {
    // ... Previous methods ...

    pub fn perform_spatial_pooling(&mut self, input_data: &Vec<bool>) {
        // Calculate overlap scores for each column based on the input data.
        // For simplicity, we'll assume the overlap score is the number of bits in the column's 
        // potential pool that overlap with the input data.
        for column in self.input_layer.get_columns().iter_mut() {
            let overlap = input_data.iter().filter(|&&bit| bit).count();
            column.set_overlap(overlap);
        }

        // Now, activate the top N columns with the highest overlap scores.
        // For this example, we'll activate the top 20% of columns.
        let threshold = (0.20 * self.input_layer.get_columns().len() as f64) as usize;
        let mut sorted_columns = self.input_layer.get_columns().clone();
        sorted_columns.sort_by(|a, b| b.get_overlap().cmp(&a.get_overlap()));

        for i in 0..threshold {
            sorted_columns[i].activate();
        }
    }
}
// Append to src/htm/htm_system.rs

impl HTMSystem {
    // ... Previous methods ...

    pub fn perform_temporal_memory(&mut self) {
        // For this example, we'll activate a random cell in each active column.
        for column in self.input_layer.get_columns().iter_mut() {
            if column.is_active() {
                let random_cell_index = rand::random::<usize>() % column.get_cells().len();
                column.get_cells()[random_cell_index].activate();
            }
        }
    }
}

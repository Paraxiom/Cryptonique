use super::layers::input_layer::InputLayer;
use rand::random;

pub struct HTMSystem {
    input_layer: InputLayer,
}

impl HTMSystem {
    pub fn new(num_columns: usize, num_cells_per_column: usize) -> Self {
        HTMSystem {
            input_layer: InputLayer::new(num_columns, num_cells_per_column),
        }
    }

    pub fn perform_spatial_pooling(&mut self, input_data: &Vec<bool>) {
        // Calculate overlap scores for each column based on the input data.
        // Assuming each column has a potential pool of synapses
        for column in self.input_layer.get_columns_mut().iter_mut() {
            // Calculate overlap based on the potential pool and input data
            let overlap = column
                .get_potential_pool()
                .iter()
                .filter(|&synapse| input_data[synapse.index]) // Use synapse.index to access input_data
                .count();
            column.set_overlap(overlap);
        }

        // Now, activate the top N columns with the highest overlap scores.
        let threshold = (0.20 * self.input_layer.get_columns().len() as f64) as usize;
        let sorted_columns = self.input_layer.get_columns_mut();
        sorted_columns.sort_by(|a, b| b.get_overlap().cmp(&a.get_overlap()));

        for column in sorted_columns.iter_mut().take(threshold) {
            column.activate();
        }
    }

    pub fn perform_temporal_memory(&mut self) {
        // Activate a random cell in each active column.
        for column in self.input_layer.get_columns_mut().iter_mut() {
            if column.is_active() {
                let random_cell_index = random::<usize>() % column.get_cells().len();
                column.get_cells_mut()[random_cell_index].activate();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_htm_system() {
        let mut htm_system = HTMSystem::new(100, 10);

        // Create a sample input vector (50% true)
        let input_data = vec![true; 50]
            .into_iter()
            .chain(vec![false; 50])
            .collect::<Vec<_>>();

        htm_system.perform_spatial_pooling(&input_data);

        // Verify that the top 20% columns are activated
        let active_columns = htm_system
            .input_layer
            .get_columns()
            .iter()
            .filter(|col| col.is_active())
            .count();
        assert_eq!(
            active_columns, 20,
            "Top 20% of columns should be activated."
        );

        htm_system.perform_temporal_memory();

        // Further tests can be added to verify the state of the system after temporal memory is performed
    }
}

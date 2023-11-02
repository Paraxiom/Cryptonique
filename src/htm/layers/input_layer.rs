// src/htm/layers/input_layer.rs

pub struct InputLayer {
    columns: Vec<Column>, // The layer contains multiple columns.
}

impl InputLayer {
    pub fn new(num_columns: usize, num_cells_per_column: usize) -> Self {
        InputLayer {
            columns: (0..num_columns).map(|_| Column::new(num_cells_per_column)).collect(),
        }
    }

    pub fn get_columns(&self) -> &Vec<Column> {
        &self.columns
    }
}

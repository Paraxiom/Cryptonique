use crate::htm::htm_model::HTMModel;
use std::sync::Mutex;

pub struct SharedState {
    pub counter: u64,
    pub htm_model: Mutex<HTMModel>,
}

impl SharedState {
    pub fn new(htm_model: HTMModel) -> Self {
        SharedState {
            counter: 0,
            htm_model: Mutex::new(htm_model),
        }
    }

    pub fn evolve(&mut self) {
        self.counter += 1;
    }

    pub fn get_counter(&self) -> u64 {
        self.counter
    }

    pub fn get_htm_model(&self) -> &Mutex<HTMModel> {
        &self.htm_model
    }
}

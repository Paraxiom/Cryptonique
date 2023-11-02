pub mod cellular_automata;
use crate::shared_state::SharedState;
pub mod chaotic_maps;
use crate::encryption::noise::cellular_automata::evolve;
use rand::Rng;

pub fn generate_noise(state: &SharedState, length: usize) -> Vec<u8> {
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(state.counter);

    let initial_state = (0..length)
        .map(|_| rng.gen::<u8>() % 2)
        .collect::<Vec<u8>>();
    evolve(&initial_state, length)
}

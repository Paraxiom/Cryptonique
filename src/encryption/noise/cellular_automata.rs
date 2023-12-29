#![allow(warnings)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
pub const RULE_30: [u8; 8] = [0, 1, 1, 1, 1, 0, 0, 0];

pub fn evolve(sdr: &Vec<u8>, steps: usize) -> Vec<u8> {
    let mut state = sdr.clone();
    for _ in 0..steps {
        state = evolve_one_step(&state);
    }
    state
}

fn evolve_one_step(state: &Vec<u8>) -> Vec<u8> {
    let mut new_state = Vec::with_capacity(state.len());
    for i in 0..state.len() {
        let left = if i == 0 { state.len() - 1 } else { i - 1 };
        let center = i;
        let right = if i == state.len() - 1 { 0 } else { i + 1 };

        let pattern = state[left] * 4 + state[center] * 2 + state[right];
        new_state.push(RULE_30[pattern as usize]);
    }
    new_state
}

// Function to generate a sequence using Rule 30
pub fn rule_30(initial: Vec<u8>, n: usize) -> Vec<Vec<u8>> {
    let mut sequence = Vec::with_capacity(n);
    sequence.push(initial);

    for _ in 0..n {
        let mut next = Vec::with_capacity(sequence[0].len());

        for i in 0..sequence[0].len() {
            let left = sequence.last().unwrap()[(i + sequence[0].len() - 1) % sequence[0].len()];
            let center = sequence.last().unwrap()[i];
            let right = sequence.last().unwrap()[(i + 1) % sequence[0].len()];

            next.push(left ^ (center | right));
        }

        sequence.push(next);
    }

    sequence
}

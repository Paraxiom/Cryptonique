// Import the Rng trait to use random methods
use rand::Rng;
// Import the SmallRng PRNG
use rand::rngs::SmallRng;
// Import the SeedableRng trait for seeding the PRNG
use rand::SeedableRng;

/// Apply a simulated quantum phase kickback operation on a classical byte array.
/// In this version, we use a more complex operation involving a pseudo-random sequence.
pub fn apply_phase_kickback(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();

    // Seed a PRNG with the sum of all bytes in the input (as a simple example)
    let seed: u64 = input.iter().map(|&x| x as u64).sum();
    let mut rng = SmallRng::seed_from_u64(seed);

    // Loop through each byte in the input
    for &byte in input.iter() {
        // Generate a random byte from the PRNG
        let random_byte: u8 = rng.gen();

        // Perform XOR with the random byte and the original byte to simulate phase kickback
        let kicked_back_byte = byte ^ random_byte;

        // Append the result to the output vector
        output.push(kicked_back_byte);
    }

    output
}

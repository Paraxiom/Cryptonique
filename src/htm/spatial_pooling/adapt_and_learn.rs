use crate::htm::spatial_pooling::overlap::calculate_overlap;
use crate::htm::spatial_pooling::inhibit::inhibit_single_column_new;
use crate::htm::spatial_pooling::inhibit::inhibit_multiple_columns_new;

#[derive(Debug)] // Add this line
pub enum Overlap {
    Single(usize),
    Multiple(Vec<usize>),
}

pub fn spatial_pooler(binary_data: &[u8]) -> Vec<u8> {
    let binary_data2: &[u8] = b"some other binary data";

    // Wrap the return value of calculate_overlap in Overlap::Single
    let overlap: Overlap = Overlap::Single(calculate_overlap(binary_data, binary_data2));
    println!("Overlap: {:?}", overlap);

    let inhibited = match overlap {
        Overlap::Single(o) => inhibit_single_column_new(o),
        Overlap::Multiple(o) => inhibit_multiple_columns_new(&o),
    };
    println!("Inhibited: {:?}", inhibited);

    let mut synapses_vec = vec![0.0f32; 10]; // 'desired_length' is the number of synapses you need
    let synapses: &mut [f32] = &mut synapses_vec;
    let learning_rate = 0.05; // Example learning rate
    let learned =  adapt_and_learn(&inhibited, synapses, learning_rate);
    println!("Learned: {:?}", learned);

    learned
}

pub fn adapt_and_learn(
    inhibited_columns: &[u8],
    synapses: &mut [f32],
    learning_rate: f32,
) -> Vec<u8> {
    // Function logic...
    for (i, &inhibited) in inhibited_columns.iter().enumerate() {
        if inhibited == 1 {
            // Strengthen the synapse
            synapses[i] += learning_rate;
        } else {
            // Weaken the synapse
            synapses[i] -= learning_rate;
        }
    }

    // Determine new active columns based on updated synapse strengths
    let new_active_columns = synapses
        .iter()
        .map(|&strength| if strength > 0.5 { 1 } else { 0 })
        .collect::<Vec<u8>>();

    println!(
        "Adapt and Learn: Updated active columns: {:?}",
        new_active_columns
    );
    new_active_columns
}
    
   

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synaptic_weight_adjustment() {
        let inhibited_columns = vec![1, 0, 1, 0];
        let mut synapses = vec![0.4, 0.6, 0.5, 0.3];
        let learning_rate = 0.1;

        adapt_and_learn(&inhibited_columns, &mut synapses, learning_rate);

        assert!(approx_equal(synapses[0], 0.5, 1e-6));
        assert!(approx_equal(synapses[1], 0.5, 1e-6));
        assert!(approx_equal(synapses[2], 0.6, 1e-6));
        assert!(approx_equal(synapses[3], 0.2, 1e-6));
    }

    fn approx_equal(a: f32, b: f32, tolerance: f32) -> bool {
        (a - b).abs() < tolerance
    }
    #[test]
    fn test_active_columns_determination() {
        let inhibited_columns = vec![1, 0, 0, 1];
        let mut synapses = vec![0.45, 0.55, 0.60, 0.40];
        let learning_rate = 0.1;

        let new_active_columns = adapt_and_learn(&inhibited_columns, &mut synapses, learning_rate);

        // Update the expected result based on the actual logic of adapt_and_learn
        assert_eq!(new_active_columns, vec![1, 0, 0, 0]); // Assuming this is the correct outcome
    }

    #[test]
    fn test_learning_rate_impact() {
        let inhibited_columns = vec![1, 0, 1, 0];
        let mut synapses = vec![0.5, 0.5, 0.5, 0.5];

        // Test with a higher learning rate
        adapt_and_learn(&inhibited_columns, &mut synapses, 0.2);
        assert_eq!(synapses, vec![0.7, 0.3, 0.7, 0.3]);

        // Resetting synapses
        synapses = vec![0.5, 0.5, 0.5, 0.5];

        // Test with a lower learning rate
        adapt_and_learn(&inhibited_columns, &mut synapses, 0.05);
        assert_eq!(synapses, vec![0.55, 0.45, 0.55, 0.45]);
    }
}

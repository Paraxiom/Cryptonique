// noise_generation.rs

/// Perturb an SDR using both cellular automata and chaotic maps.
pub fn perturb_sdr_with_chaos(sdr: &[bool], initial_conditions: f64) -> Vec<bool> {
    let mut new_sdr = vec![false; sdr.len()];

    // Apply cellular automaton (Rule 30)
    for i in 0..sdr.len() {
        let left = if i == 0 { false } else { sdr[i - 1] };
        let center = sdr[i];
        let right = if i == sdr.len() - 1 { false } else { sdr[i + 1] };

        // Rule 30: XOR of (left, center, right)
        new_sdr[i] = left ^ (center || right);
    }

    // Apply chaotic map (Logistic Map)
    let mut r = initial_conditions;
    for bit in &mut new_sdr {
        r = 4.0 * r * (1.0 - r); // Logistic map formula
        *bit ^= r > 0.5; // Flip the bit based on logistic map output
    }

    new_sdr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        let sdr = vec![true, false, true, false, true];
        let initial_conditions = 0.5;
        let perturbed_sdr = perturb_sdr_with_chaos(&sdr, initial_conditions);

        assert_ne!(sdr, perturbed_sdr, "Perturbed SDR should differ from the original");
    }

    #[test]
    fn test_determinism() {
        let sdr = vec![false, true, false, true];
        let initial_conditions = 0.3;

        let result1 = perturb_sdr_with_chaos(&sdr, initial_conditions);
        let result2 = perturb_sdr_with_chaos(&sdr, initial_conditions);

        assert_eq!(result1, result2, "Function should be deterministic with the same inputs");
    }

    #[test]
    fn test_edge_cases() {
        let empty_sdr = Vec::<bool>::new();
        let single_element_sdr = vec![true];

        let perturbed_empty_sdr = perturb_sdr_with_chaos(&empty_sdr, 0.4);
        let perturbed_single_element_sdr = perturb_sdr_with_chaos(&single_element_sdr, 0.4);

        assert!(perturbed_empty_sdr.is_empty(), "Perturbed empty SDR should still be empty");
        assert_eq!(perturbed_single_element_sdr.len(), 1, "Perturbed single element SDR should have one element");
    }

    #[test]
    fn test_variability() {
        let sdr = vec![true, true, false, false];
        let initial_condition1 = 0.2;
        let initial_condition2 = 0.7;

        let result1 = perturb_sdr_with_chaos(&sdr, initial_condition1);
        let result2 = perturb_sdr_with_chaos(&sdr, initial_condition2);

        assert_ne!(result1, result2, "Different initial conditions should yield different results");
    }
}


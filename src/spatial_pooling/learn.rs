pub fn adapt_and_learn(inhibited_columns: &Vec<u8>) -> Vec<u8> {
    // For simplicity, let's assume that our learning process just returns the same columns for now.
    // In a real-world scenario, this step would involve adjusting the permanences of synapses and might change the active columns.
    println!("Adapt and Learn (inside function): {:?}", inhibited_columns);
    inhibited_columns.clone()
}

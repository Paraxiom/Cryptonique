// src/htm/output_layer.rs

struct Node {
    // Example property
    active: bool,
}

impl Node {
    // Constructor for a new Node
    pub fn new() -> Self {
        Node { active: false }
    }

    // Process input for this node
    pub fn process_input(&mut self, input: bool) {
        // Simple example logic: the node becomes active if input is true
        self.active = input;
    }

    // Get the output of this node
    pub fn get_output(&self) -> bool {
        // Output depends on the node's state
        self.active
    }
}

pub struct OutputLayer {
    nodes: Vec<Node>,
}

impl OutputLayer {
    pub fn new(num_nodes: usize) -> Self {
        OutputLayer {
            nodes: (0..num_nodes).map(|_| Node::new()).collect(),
        }
    }

    // Process input from previous layers
    pub fn process_input(&mut self, input: &[bool]) {
        for (i, node) in self.nodes.iter_mut().enumerate() {
            if i < input.len() {
                node.process_input(input[i]);
            } else {
                // Default behavior for nodes without input
                node.process_input(false);
            }
        }
    }

    // Encode the output
    pub fn encode_output(&self) -> Vec<bool> {
        self.nodes.iter().map(|node| node.get_output()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_layer_processing() {
        let mut layer = OutputLayer::new(10);
        let input = vec![
            true, false, true, false, true, false, false, true, true, false,
        ];
        layer.process_input(&input);

        for (i, node) in layer.nodes.iter().enumerate() {
            assert_eq!(node.active, input[i]);
        }
    }

    #[test]
    fn test_output_encoding() {
        let mut layer = OutputLayer::new(10);
        let input = vec![
            true, false, true, false, true, false, false, true, true, false,
        ];
        layer.process_input(&input);

        let output = layer.encode_output();
        assert_eq!(output, input);
    }
}

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;
use thiserror::Error;

const BRAIN_MUTATION_RATE: f32 = 5.0;
const BRAIN_MUTATION_VARIATION: f32 = 0.5;

#[derive(Error, Debug)]
pub enum NeuralNetError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Invalid network structure: {0}")]
    InvalidStructure(String),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Net {
    n_inputs: usize,
    layers: Vec<Layer>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Layer {
    nodes: Vec<Vec<f64>>,
}

impl Net {
    pub fn new(layer_sizes: Vec<usize>) -> Self {
        if layer_sizes.len() < 2 {
            panic!("Need at least 2 layers");
        }
        for &size in layer_sizes.iter() {
            if size < 1 {
                panic!("Empty layers not allowed");
            }
        }

        let mut layers = Vec::new();
        let first_layer_size = *layer_sizes.first().unwrap();
        let mut prev_layer_size = first_layer_size;

        for &layer_size in layer_sizes[1..].iter() {
            layers.push(Layer::new(layer_size, prev_layer_size));
            prev_layer_size = layer_size;
        }

        Self {
            layers,
            n_inputs: first_layer_size,
        }
    }

    /// Save the neural network to a file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), NeuralNetError> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Load a neural network from a file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, NeuralNetError> {
        let json = fs::read_to_string(path)?;
        let net: Net = serde_json::from_str(&json)?;
        Ok(net)
    }

    /// Get the number of inputs
    pub fn get_input_count(&self) -> usize {
        self.n_inputs
    }

    /// Get the layer sizes
    pub fn get_layer_sizes(&self) -> Vec<usize> {
        let mut sizes = vec![self.n_inputs];
        for layer in &self.layers {
            sizes.push(layer.nodes.len());
        }
        sizes
    }

    pub fn predict(&self, inputs: &Vec<f64>) -> Vec<Vec<f64>> {
        if inputs.len() != self.n_inputs {
            eprintln!(
                "Warning: Bad input size. Expected {}, got {}",
                self.n_inputs,
                inputs.len()
            );
            return vec![inputs.clone()];
        }

        let mut outputs = Vec::new();
        outputs.push(inputs.clone());
        for (layer_index, layer) in self.layers.iter().enumerate() {
            let layer_results = layer.predict(&outputs[layer_index]);
            outputs.push(layer_results);
        }

        outputs
    }

    pub fn mutate(&mut self) {
        self.layers.iter_mut().for_each(|l| l.mutate());
    }
}

impl Layer {
    fn new(layer_size: usize, prev_layer_size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut nodes: Vec<Vec<f64>> = Vec::new();

        for _ in 0..layer_size {
            let mut node: Vec<f64> = Vec::new();
            for _ in 0..prev_layer_size + 1 {
                let random_weight: f64 = rng.gen_range(-1.0f64..1.0f64);
                node.push(random_weight);
            }
            nodes.push(node);
        }

        Self { nodes }
    }

    fn predict(&self, inputs: &Vec<f64>) -> Vec<f64> {
        let mut layer_results = Vec::new();
        for node in self.nodes.iter() {
            layer_results.push(self.sigmoid(self.dot_prod(&node, &inputs)));
        }

        layer_results
    }

    fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        for n in self.nodes.iter_mut() {
            for val in n.iter_mut() {
                if rng.gen_range(0.0..1.0) >= BRAIN_MUTATION_RATE {
                    continue;
                }

                *val += rng.gen_range(-BRAIN_MUTATION_VARIATION..BRAIN_MUTATION_VARIATION) as f64;
            }
        }
    }

    fn dot_prod(&self, node: &Vec<f64>, values: &Vec<f64>) -> f64 {
        let mut it = node.iter();
        let mut total = *it.next().unwrap();
        for (weight, value) in it.zip(values.iter()) {
            total += weight * value;
        }

        total
    }

    fn sigmoid(&self, y: f64) -> f64 {
        1f64 / (1f64 + (-y).exp())
    }
}

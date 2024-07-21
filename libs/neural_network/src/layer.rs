use crate::GenRandFloat;
use crate::neuron::Neuron;

#[derive(Debug)]
pub struct Layer {
    pub(crate) neurons: Vec<Neuron>,
}

impl Layer {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }

    pub fn new<T: GenRandFloat>(rng: &mut T, input_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::new(rng, input_size))
            .collect();

        Self { neurons }
    }

    pub(crate) fn from_weights(input_size: usize, output_size: usize, weights: &mut dyn Iterator<Item = f32>) -> Self {
        Self {
            neurons: (0..output_size)
                .map(|_| Neuron::from_weights(input_size, weights))
                .collect()
        }
    }
}
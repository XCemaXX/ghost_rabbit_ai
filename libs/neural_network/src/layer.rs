use crate::GenRandFloat;
use crate::activation::Activation;
use crate::neuron::Neuron;

#[derive(Debug, Clone)]
pub struct Layer {
    pub(crate) neurons: Vec<Neuron>,
    pub(crate) activation: Activation,
}

impl Layer {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        let mut outputs = Vec::new();
        self.propagate_into(&inputs, &mut outputs);
        outputs
    }

    // Activation in a separate pass: a loop over a slice lets the compiler use SIMD.
    pub fn propagate_into(&self, inputs: &[f32], outputs: &mut Vec<f32>) {
        outputs.clear();
        outputs.extend(self.neurons.iter().map(|neuron| neuron.propagate(inputs)));
        self.activation.apply_all(outputs);
    }

    pub fn new<T: GenRandFloat>(
        rng: &mut T,
        input_size: usize,
        output_size: usize,
        activation: Activation,
    ) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::new(rng, input_size))
            .collect();

        Self {
            neurons,
            activation,
        }
    }

    pub(crate) fn from_weights(
        input_size: usize,
        output_size: usize,
        activation: Activation,
        weights: &mut dyn Iterator<Item = f32>,
    ) -> Self {
        Self {
            neurons: (0..output_size)
                .map(|_| Neuron::from_weights(input_size, weights))
                .collect(),
            activation,
        }
    }
}

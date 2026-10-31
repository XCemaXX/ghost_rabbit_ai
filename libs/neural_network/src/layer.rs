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
        self.neurons
            .iter()
            .map(|neuron| self.activation.apply(neuron.propagate(&inputs)))
            .collect()
    }

    pub fn propagate_into(&self, inputs: &[f32], outputs: &mut Vec<f32>) {
        outputs.clear();
        outputs.extend(
            self.neurons
                .iter()
                .map(|neuron| self.activation.apply(neuron.propagate(inputs))),
        );
    }

    // `grad` is this layer's part of the network gradient, in the order of `weights()`.
    // Returns dLoss/dInput for the layer below.
    pub(crate) fn backward(
        &self,
        inputs: &[f32],
        outputs: &[f32],
        output_grad: &[f32],
        grad: &mut [f32],
    ) -> Vec<f32> {
        let mut input_grad = vec![0.0; inputs.len()];
        let mut grad = grad.iter_mut();
        for ((neuron, &output), &d_output) in self.neurons.iter().zip(outputs).zip(output_grad) {
            let delta = d_output * self.activation.derivative(output);
            *grad.next().unwrap() += delta;
            for ((weight, &input), d_input) in
                neuron.weights.iter().zip(inputs).zip(&mut input_grad)
            {
                *grad.next().unwrap() += delta * input;
                *d_input += delta * weight;
            }
        }
        input_grad
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

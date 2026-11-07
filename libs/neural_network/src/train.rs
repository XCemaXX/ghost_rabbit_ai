// Losses return (loss, dLoss/dOutput) for one sample. Pass the gradient to `Network::backward_batch`.
use crate::Network;

const BETA1: f32 = 0.9;
const BETA2: f32 = 0.999;

pub fn mse(outputs: &[f32], targets: &[f32]) -> (f32, Vec<f32>) {
    assert_eq!(outputs.len(), targets.len());
    let mut loss = 0.0;
    let grad = outputs
        .iter()
        .zip(targets)
        .map(|(&output, &target)| {
            let diff = output - target;
            loss += 0.5 * diff * diff;
            diff
        })
        .collect();
    (loss, grad)
}

// Binary cross-entropy on raw outputs (logits), targets in [0, 1].
pub fn bce_with_logits(logits: &[f32], targets: &[f32]) -> (f32, Vec<f32>) {
    assert_eq!(logits.len(), targets.len());
    let mut loss = 0.0;
    let grad = logits
        .iter()
        .zip(targets)
        .map(|(&logit, &target)| {
            let p = sigmoid(logit).clamp(1e-6, 1.0 - 1e-6);
            loss -= target * p.ln() + (1.0 - target) * (1.0 - p).ln();
            p - target
        })
        .collect();
    (loss, grad)
}

pub fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

pub struct Adam {
    learning_rate: f32,
    m: Vec<f32>,
    v: Vec<f32>,
    steps: i32,
}

impl Adam {
    pub fn new(weights_count: usize, learning_rate: f32) -> Self {
        Self {
            learning_rate,
            m: vec![0.0; weights_count],
            v: vec![0.0; weights_count],
            steps: 0,
        }
    }

    pub fn set_learning_rate(&mut self, learning_rate: f32) {
        self.learning_rate = learning_rate;
    }

    // `grad` is the mean gradient of a batch, in the order of `Network::weights()`.
    pub fn step(&mut self, network: &mut Network, grad: &[f32]) {
        assert_eq!(grad.len(), self.m.len());
        self.steps += 1;
        let m_scale = 1.0 / (1.0 - BETA1.powi(self.steps));
        let v_scale = 1.0 / (1.0 - BETA2.powi(self.steps));
        for (((weight, &g), m), v) in network
            .weights_mut()
            .zip(grad)
            .zip(&mut self.m)
            .zip(&mut self.v)
        {
            *m = BETA1 * *m + (1.0 - BETA1) * g;
            *v = BETA2 * *v + (1.0 - BETA2) * g * g;
            *weight -= self.learning_rate * *m * m_scale / ((*v * v_scale).sqrt() + 1e-8);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Activation, LayerTopology};

    // y = 2x - 1 with one linear neuron.
    #[test]
    fn adam_fits_a_line() {
        let topology = [
            LayerTopology {
                neurons: 1,
                activation: Activation::Relu,
            },
            LayerTopology {
                neurons: 1,
                activation: Activation::Linear,
            },
        ];
        let mut network = Network::from_weights(&topology, [0.0, 0.0]);
        let mut adam = Adam::new(network.weights_count(), 0.05);
        let inputs: [&[f32]; 4] = [&[-1.0], &[0.0], &[1.0], &[2.0]];
        for _ in 0..500 {
            let mut grad = vec![0.0; 2];
            let target = |i: usize| [2.0 * inputs[i][0] - 1.0];
            network.backward_batch(&inputs, |i, out| mse(out, &target(i)).1, &mut grad);
            grad.iter_mut().for_each(|g| *g /= 4.0);
            adam.step(&mut network, &grad);
        }
        let weights: Vec<f32> = network.weights().collect();
        assert!((weights[0] + 1.0).abs() < 0.05, "bias {}", weights[0]);
        assert!((weights[1] - 2.0).abs() < 0.05, "weight {}", weights[1]);
    }
}

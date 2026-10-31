// Losses return (loss, dLoss/dOutput) for one sample. Pass the gradient to `Network::backward`.
use crate::Network;

// A NaN target means "no value": it adds no loss and no gradient.
pub fn mse(outputs: &[f32], targets: &[f32]) -> (f32, Vec<f32>) {
    assert_eq!(outputs.len(), targets.len());
    let mut loss = 0.0;
    let grad = outputs
        .iter()
        .zip(targets)
        .map(|(&output, &target)| {
            if target.is_nan() {
                return 0.0;
            }
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

pub fn softmax_cross_entropy(logits: &[f32], class: usize) -> (f32, Vec<f32>) {
    let max = logits.iter().cloned().fold(f32::MIN, f32::max);
    let exp: Vec<f32> = logits.iter().map(|logit| (logit - max).exp()).collect();
    let sum: f32 = exp.iter().sum();
    let grad = exp
        .iter()
        .enumerate()
        .map(|(i, e)| e / sum - (i == class) as u8 as f32)
        .collect();
    (-(exp[class] / sum).ln(), grad)
}

pub fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

pub struct Adam {
    learning_rate: f32,
    beta1: f32,
    beta2: f32,
    m: Vec<f32>,
    v: Vec<f32>,
    steps: i32,
}

impl Adam {
    pub fn new(weights_count: usize, learning_rate: f32) -> Self {
        Self {
            learning_rate,
            beta1: 0.9,
            beta2: 0.999,
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
        let m_scale = 1.0 / (1.0 - self.beta1.powi(self.steps));
        let v_scale = 1.0 / (1.0 - self.beta2.powi(self.steps));
        for (((weight, &g), m), v) in network
            .weights_mut()
            .zip(grad)
            .zip(&mut self.m)
            .zip(&mut self.v)
        {
            *m = self.beta1 * *m + (1.0 - self.beta1) * g;
            *v = self.beta2 * *v + (1.0 - self.beta2) * g * g;
            *weight -= self.learning_rate * *m * m_scale / ((*v * v_scale).sqrt() + 1e-8);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Activation, LayerTopology};

    #[test]
    fn mse_skips_nan_target() {
        let (loss, grad) = mse(&[1.0, 2.0], &[0.0, f32::NAN]);
        assert_eq!(loss, 0.5);
        assert_eq!(grad, vec![1.0, 0.0]);
    }

    #[test]
    fn softmax_gradient_sums_to_zero() {
        let (_, grad) = softmax_cross_entropy(&[0.5, -1.0, 2.0], 1);
        assert!(grad.iter().sum::<f32>().abs() < 1e-6);
        assert!(grad[1] < 0.0);
    }

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
        for _ in 0..500 {
            let mut grad = vec![0.0; 2];
            for x in [-1.0, 0.0, 1.0, 2.0] {
                let trace = network.trace(vec![x]);
                let (_, output_grad) = mse(trace.output(), &[2.0 * x - 1.0]);
                network.backward(&trace, &output_grad, &mut grad);
            }
            grad.iter_mut().for_each(|g| *g /= 4.0);
            adam.step(&mut network, &grad);
        }
        let weights: Vec<f32> = network.weights().collect();
        assert!((weights[0] + 1.0).abs() < 0.05, "bias {}", weights[0]);
        assert!((weights[1] - 2.0).abs() < 0.05, "weight {}", weights[1]);
    }
}

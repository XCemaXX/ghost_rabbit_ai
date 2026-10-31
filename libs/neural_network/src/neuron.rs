use crate::GenRandFloat;

#[derive(Debug, Clone)]
pub struct Neuron {
    pub(crate) bias: f32,
    pub(crate) weights: Vec<f32>,
}

impl Neuron {
    // Weighted sum, before the layer's activation.
    // Eight partial sums: a single running sum cannot be reordered, so the compiler cannot use SIMD.
    pub fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());
        let mut sums = [0.0; 8];
        let inputs8 = inputs.chunks_exact(8);
        let weights8 = self.weights.chunks_exact(8);
        let tail = inputs8
            .remainder()
            .iter()
            .zip(weights8.remainder())
            .map(|(input, weight)| input * weight)
            .sum::<f32>();
        for (inputs, weights) in inputs8.zip(weights8) {
            for k in 0..8 {
                sums[k] += inputs[k] * weights[k];
            }
        }

        self.bias + sums.iter().sum::<f32>() + tail
    }

    // Variance 1 / input_size: the weighted sum stays near 1, so deep nets do not saturate.
    // The random bias keeps a GA start population diverse.
    pub fn new<T: GenRandFloat>(rng: &mut T, input_size: usize) -> Self {
        let limit = (3.0 / input_size as f32).sqrt();
        let bias = rng.gen_range(-limit..=limit);

        let weights = (0..input_size)
            .map(|_| rng.gen_range(-limit..=limit))
            .collect();

        Self { bias, weights }
    }

    pub(crate) fn from_weights(input_size: usize, weights: &mut dyn Iterator<Item = f32>) -> Self {
        let bias = weights.next().expect("Not enough weights");
        let weights = (0..input_size)
            .map(|_| weights.next().expect("Not enough weights"))
            .collect();
        Self { bias, weights }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod propagate {
        use super::*;

        #[test]
        fn returns_propagated_input() {
            let actual = Neuron {
                bias: 0.1,
                weights: vec![-0.3, 0.6, 0.9],
            }
            .propagate(&[0.5, -0.6, 0.7]);
            let expected: f32 = 0.1 + (0.5 * -0.3) + (-0.6 * 0.6) + (0.7 * 0.9);

            assert!((actual - expected).abs() < f32::EPSILON);
        }
    }
}

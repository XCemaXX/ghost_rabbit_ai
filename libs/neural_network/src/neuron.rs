use crate::GenRandFloat;

#[derive(Debug, Clone)]
pub struct Neuron {
    pub(crate) bias: f32,
    pub(crate) weights: Vec<f32>,
}

impl Neuron {
    pub fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());
        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        (self.bias + output).max(0.0)
    }

    pub fn new<T: GenRandFloat>(rng: &mut T, input_size: usize) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..input_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();

        Self { bias, weights }
    }

    pub(crate) fn from_weights(input_size: usize, weights: &mut dyn Iterator<Item = f32>) -> Self {
        let bias = weights.next().expect("Not enough weights");
        let weights = (0..input_size)
            .map(|_| weights.next().expect("Not enough weights"))
            .collect();
        Self {
            bias,
            weights,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod propagate {
        use super::*;

        #[test]
        fn returns_propagated_input() {
            let actual = Neuron{bias:0.1, weights: vec![-0.3, 0.6, 0.9]}.propagate(&[0.5, -0.6, 0.7]);
            let expected: f32 = 0.1 + (0.5 * -0.3) + (-0.6 * 0.6) + (0.7 * 0.9);
            
            assert!((actual - expected).abs() < f32::EPSILON);
        }
    }
}
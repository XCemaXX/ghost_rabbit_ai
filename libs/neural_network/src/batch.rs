use crate::Network;

impl Network {
    // Adds dLoss/dWeight to `grad`, in the order of `weights()`.
    // Layer by layer for the whole batch: a gradient row stays in cache for all inputs.
    // `output_grad(i, output)` is dLoss/dOutput for input `i`.
    pub fn backward_batch(
        &self,
        inputs: &[&[f32]],
        output_grad: impl Fn(usize, &[f32]) -> Vec<f32>,
        grad: &mut [f32],
    ) {
        assert_eq!(grad.len(), self.weights_count());
        let n = inputs.len();
        // Row-major matrices, one row per input: the outputs of every layer, the inputs first.
        let mut outputs = vec![inputs.concat()];
        for layer in &self.layers {
            let input = outputs.last().unwrap();
            let width = input.len() / n;
            let mut output: Vec<f32> = input
                .chunks_exact(width)
                .flat_map(|row| layer.neurons.iter().map(|neuron| neuron.propagate(row)))
                .collect();
            layer.activation.apply_all(&mut output);
            outputs.push(output);
        }
        let last = outputs.last().unwrap();
        let mut d_output: Vec<f32> = last
            .chunks_exact(last.len() / n)
            .enumerate()
            .flat_map(|(i, row)| output_grad(i, row))
            .collect();

        let mut end = grad.len();
        for (l, layer) in self.layers.iter().enumerate().rev() {
            let (input, output) = (&outputs[l], &outputs[l + 1]);
            let (in_width, out_width) = (input.len() / n, output.len() / n);
            let delta: Vec<f32> = d_output
                .iter()
                .zip(output)
                .map(|(&d, &y)| d * layer.activation.derivative(y))
                .collect();
            let start = end - out_width * (in_width + 1);
            let rows = grad[start..end].chunks_exact_mut(in_width + 1);
            for (o, row) in rows.enumerate() {
                let (bias, weights) = row.split_first_mut().unwrap();
                for (deltas, inputs) in delta
                    .chunks_exact(out_width)
                    .zip(input.chunks_exact(in_width))
                {
                    let d = deltas[o];
                    *bias += d;
                    for (w, &x) in weights.iter_mut().zip(inputs) {
                        *w += d * x;
                    }
                }
            }
            end = start;
            if l == 0 {
                break;
            }
            let mut d_input = vec![0.0; input.len()];
            for (d_in, deltas) in d_input
                .chunks_exact_mut(in_width)
                .zip(delta.chunks_exact(out_width))
            {
                for (neuron, &d) in layer.neurons.iter().zip(deltas) {
                    for (g, &w) in d_in.iter_mut().zip(&neuron.weights) {
                        *g += d * w;
                    }
                }
            }
            d_output = d_input;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Activation, LayerTopology, Network, train};

    #[test]
    fn backward_batch_matches_finite_differences() {
        let layer = |neurons, activation| LayerTopology {
            neurons,
            activation,
        };
        let topology = [
            layer(3, Activation::Tanh),
            layer(4, Activation::Tanh),
            layer(3, Activation::Relu),
            layer(2, Activation::Linear),
        ];
        let count = (3 + 1) * 4 + (4 + 1) * 3 + (3 + 1) * 2;
        let weights: Vec<f32> = (0..count)
            .map(|i| ((i * 7 % 13) as f32 - 6.0) / 10.0)
            .collect();
        let inputs: [&[f32]; 3] = [&[0.3, -0.8, 0.5], &[-0.2, 0.1, 0.9], &[0.7, 0.4, -0.6]];
        let target = |i: usize| [i as f32 / 3.0, -0.5];
        let loss = |weights: Vec<f32>| -> f32 {
            let network = Network::from_weights(&topology, weights);
            (0..inputs.len())
                .map(|i| train::mse(&network.propagate(inputs[i].to_vec()), &target(i)).0)
                .sum()
        };

        let network = Network::from_weights(&topology, weights.iter().copied());
        let mut grad = vec![0.0; count];
        network.backward_batch(&inputs, |i, out| train::mse(out, &target(i)).1, &mut grad);

        let eps = 1e-3;
        for i in 0..count {
            let mut plus = weights.clone();
            plus[i] += eps;
            let mut minus = weights.clone();
            minus[i] -= eps;
            let numeric = (loss(plus) - loss(minus)) / (2.0 * eps);
            assert!(
                (numeric - grad[i]).abs() < 2e-3,
                "weight {i}: {numeric} vs {}",
                grad[i]
            );
        }
    }
}

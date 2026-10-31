use crate::Network;

impl Network {
    // Same sum as `trace` + `backward` for every input, but layer by layer for the whole batch:
    // a gradient row stays in cache for all inputs instead of the whole gradient for each input.
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
    fn backward_batch_matches_backward() {
        let layer = |neurons, activation| LayerTopology {
            neurons,
            activation,
        };
        let topology = [
            layer(5, Activation::Tanh),
            layer(7, Activation::Tanh),
            layer(6, Activation::Relu),
            layer(3, Activation::Linear),
        ];
        let count = (5 + 1) * 7 + (7 + 1) * 6 + (6 + 1) * 3;
        let weights: Vec<f32> = (0..count)
            .map(|i| ((i * 7 % 13) as f32 - 6.0) / 10.0)
            .collect();
        let network = Network::from_weights(&topology, weights);
        let inputs: Vec<Vec<f32>> = (0..9)
            .map(|b| {
                (0..5)
                    .map(|i| ((b * 5 + i) % 11) as f32 / 5.0 - 1.0)
                    .collect()
            })
            .collect();
        let target = |b: usize| [b as f32 / 9.0, -0.5, 0.25];

        let mut expected = vec![0.0; count];
        for (b, x) in inputs.iter().enumerate() {
            let trace = network.trace(x.clone());
            let output_grad = train::mse(trace.output(), &target(b)).1;
            network.backward(&trace, &output_grad, &mut expected);
        }
        let mut actual = vec![0.0; count];
        let rows: Vec<&[f32]> = inputs.iter().map(|x| x.as_slice()).collect();
        network.backward_batch(&rows, |b, out| train::mse(out, &target(b)).1, &mut actual);
        for (i, (a, e)) in actual.iter().zip(&expected).enumerate() {
            assert!((a - e).abs() < 1e-5, "weight {i}: {a} vs {e}");
        }
    }
}

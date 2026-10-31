mod activation;
mod batch;
mod layer;
mod neuron;
pub mod train;

pub use self::activation::Activation;
use self::layer::Layer;
use rand_trait::GenRandFloat;

#[derive(Debug, Clone)]
pub struct Network {
    layers: Vec<Layer>,
}

// Scratch space for `propagate_into`, so a hot loop does not allocate.
#[derive(Debug, Default)]
pub struct Buffers {
    current: Vec<f32>,
    next: Vec<f32>,
}

// Outputs of every layer for one input; the first entry is the input itself.
#[derive(Debug)]
pub struct Trace {
    outputs: Vec<Vec<f32>>,
}

impl Trace {
    pub fn output(&self) -> &[f32] {
        self.outputs.last().unwrap()
    }
}

impl Network {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

    pub fn propagate_into<'a>(&self, inputs: &[f32], buf: &'a mut Buffers) -> &'a [f32] {
        buf.current.clear();
        buf.current.extend_from_slice(inputs);
        for layer in &self.layers {
            layer.propagate_into(&buf.current, &mut buf.next);
            std::mem::swap(&mut buf.current, &mut buf.next);
        }
        &buf.current
    }

    pub fn trace(&self, inputs: Vec<f32>) -> Trace {
        let mut outputs = vec![inputs];
        for layer in &self.layers {
            let next = layer.propagate(outputs.last().unwrap().clone());
            outputs.push(next);
        }
        Trace { outputs }
    }

    // Adds dLoss/dWeight to `grad`, in the order of `weights()`.
    // `output_grad` is dLoss/dOutput, for example from a loss in `train`.
    pub fn backward(&self, trace: &Trace, output_grad: &[f32], grad: &mut [f32]) {
        assert_eq!(grad.len(), self.weights_count());
        let mut end = grad.len();
        let mut d_output = output_grad.to_vec();
        for (i, layer) in self.layers.iter().enumerate().rev() {
            let start = end - layer.neurons.len() * (trace.outputs[i].len() + 1);
            d_output = layer.backward(
                &trace.outputs[i],
                &trace.outputs[i + 1],
                &d_output,
                &mut grad[start..end],
            );
            end = start;
        }
    }

    pub fn new<T: GenRandFloat>(rng: &mut T, layers: &[LayerTopology]) -> Self {
        let layers = layers
            .windows(2)
            .map(|layers| {
                Layer::new(
                    rng,
                    layers[0].neurons,
                    layers[1].neurons,
                    layers[1].activation,
                )
            })
            .collect();

        Self { layers }
    }

    pub fn weights(&self) -> impl Iterator<Item = f32> + '_ {
        self.layers
            .iter()
            .flat_map(|layer| layer.neurons.iter())
            .flat_map(|neuron| std::iter::once(&neuron.bias).chain(&neuron.weights))
            .cloned()
    }

    pub fn weights_mut(&mut self) -> impl Iterator<Item = &mut f32> + '_ {
        self.layers
            .iter_mut()
            .flat_map(|layer| layer.neurons.iter_mut())
            .flat_map(|neuron| std::iter::once(&mut neuron.bias).chain(&mut neuron.weights))
    }

    pub fn weights_count(&self) -> usize {
        self.layers
            .iter()
            .flat_map(|layer| layer.neurons.iter())
            .map(|neuron| neuron.weights.len() + 1)
            .sum()
    }

    pub fn from_weights(
        topology: &[LayerTopology],
        weights: impl IntoIterator<Item = f32>,
    ) -> Self {
        assert!(topology.len() > 1);
        let mut weights = weights.into_iter();
        let layers = topology
            .windows(2)
            .map(|topology| {
                Layer::from_weights(
                    topology[0].neurons,
                    topology[1].neurons,
                    topology[1].activation,
                    &mut weights,
                )
            })
            .collect();
        if weights.next().is_some() {
            panic!("too match weights");
        }
        Self { layers }
    }
}

// The activation of the first (input) layer is not used.
#[derive(Debug)]
pub struct LayerTopology {
    pub neurons: usize,
    pub activation: Activation,
}

#[cfg(test)]
mod tests {
    use self::neuron::Neuron;
    use super::*;
    use rand::{RngExt, rng, rngs::ThreadRng};
    use rand_trait::GenRandFloat;

    pub struct RandGen {
        rng: ThreadRng,
    }

    impl RandGen {
        pub fn new() -> Self {
            Self { rng: rng() }
        }
    }

    impl GenRandFloat for RandGen {
        fn gen_range(&mut self, range: std::ops::RangeInclusive<f32>) -> f32 {
            let low = *range.start();
            let high = *range.end();
            self.rng.random_range(low..high)
        }
    }

    const fn relu(neurons: usize) -> LayerTopology {
        LayerTopology {
            neurons,
            activation: Activation::Relu,
        }
    }

    const LAYERS_TOPOLOGY: [LayerTopology; 3] = [relu(4 + 5 + 24), relu(14), relu(2)];

    #[test]
    fn weights() {
        let mut rng = RandGen::new();
        let brain = Network::new(&mut rng, &LAYERS_TOPOLOGY);
        let ws = brain.weights().collect::<Vec<_>>();
        let brain2 = Network::from_weights(&LAYERS_TOPOLOGY, ws);
        assert_eq!(
            brain.weights().collect::<Vec<_>>(),
            brain2.weights().collect::<Vec<_>>()
        );
        assert_eq!(brain.weights_count(), brain.weights().count());
    }

    #[test]
    fn propagate() {
        let layers = (
            Layer {
                neurons: vec![
                    Neuron {
                        bias: 0.0,
                        weights: vec![-0.5, -0.4, -0.3],
                    },
                    Neuron {
                        bias: 0.0,
                        weights: vec![-0.2, -0.1, 0.0],
                    },
                ],
                activation: Activation::Relu,
            },
            Layer {
                neurons: vec![Neuron {
                    bias: 0.0,
                    weights: vec![-0.5, 0.5],
                }],
                activation: Activation::Relu,
            },
        );
        let network = Network {
            layers: vec![layers.0.clone(), layers.1.clone()],
        };

        let actual = network.propagate(vec![0.5, 0.6, 0.7]);
        let expected = layers.1.propagate(layers.0.propagate(vec![0.5, 0.6, 0.7]));

        for (f, s) in std::iter::zip(&actual, expected) {
            assert!((f - s).abs() < f32::EPSILON)
        }
        let mut buf = Buffers::default();
        assert_eq!(network.propagate_into(&[0.5, 0.6, 0.7], &mut buf), actual);
    }

    #[test]
    fn linear_output_keeps_negative() {
        let topology = [
            relu(2),
            relu(1),
            LayerTopology {
                neurons: 1,
                activation: Activation::Linear,
            },
        ];
        // Hidden: relu(1 + 1 + 1) = 3. Output: -1 + 3 * -2 = -7.
        let network = Network::from_weights(&topology, [1.0, 1.0, 1.0, -1.0, -2.0]);
        assert_eq!(network.propagate(vec![1.0, 1.0]), vec![-7.0]);
    }

    // Compares `backward` with finite differences on a small tanh net.
    #[test]
    fn backward_matches_finite_differences() {
        let topology = [
            relu(3),
            LayerTopology {
                neurons: 4,
                activation: Activation::Tanh,
            },
            LayerTopology {
                neurons: 2,
                activation: Activation::Linear,
            },
        ];
        let weights: Vec<f32> = (0..26)
            .map(|i| ((i * 7 % 11) as f32 - 5.0) / 10.0)
            .collect();
        let inputs = vec![0.3, -0.8, 0.5];
        let targets = [0.2, -0.4];
        let loss = |network: &Network| train::mse(&network.propagate(inputs.clone()), &targets).0;

        let network = Network::from_weights(&topology, weights.clone());
        let trace = network.trace(inputs.clone());
        let (_, output_grad) = train::mse(trace.output(), &targets);
        let mut grad = vec![0.0; network.weights_count()];
        network.backward(&trace, &output_grad, &mut grad);

        let eps = 1e-3;
        for i in 0..weights.len() {
            let mut plus = weights.clone();
            plus[i] += eps;
            let mut minus = weights.clone();
            minus[i] -= eps;
            let numeric = (loss(&Network::from_weights(&topology, plus))
                - loss(&Network::from_weights(&topology, minus)))
                / (2.0 * eps);
            assert!(
                (numeric - grad[i]).abs() < 1e-3,
                "weight {i}: {numeric} vs {}",
                grad[i]
            );
        }
    }
}

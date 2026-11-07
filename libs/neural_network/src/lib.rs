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
}

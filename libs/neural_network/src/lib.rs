mod layer;
mod neuron;

use rand_trait::GenRandFloat;
use self::layer::Layer;

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

    pub fn new<T: GenRandFloat>(rng: &mut T, layers: &[LayerTopology]) -> Self {
        let layers = layers
            .windows(2)
            .map(|layers| Layer::new(rng, layers[0].neurons, layers[1].neurons))
            .collect();

        Self { layers }
    }

    pub fn weights(&self) -> impl Iterator<Item = f32> + '_ {
        self.layers.iter()
            .flat_map(|layer| layer.neurons.iter())
            .flat_map(|neuron| std::iter::once(&neuron.bias).chain(&neuron.weights))
            .cloned()
    }

    pub fn from_weights(topology: &[LayerTopology], weights: impl IntoIterator<Item = f32>) -> Self {
        assert!(topology.len() > 1);
        let mut weights = weights.into_iter();
        let layers = topology
            .windows(2)
            .map(|topology| Layer::from_weights(topology[0].neurons, topology[1].neurons, &mut weights) )
            .collect();
        if weights.next().is_some() {
            panic!("too match weights");
        }
        Self{ layers }
    }
}

#[derive(Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use self::neuron::Neuron;
    use rand_trait::GenRandFloat;
    use rand::{rngs::ThreadRng, thread_rng, Rng};

    pub struct RandGen {
        rng: ThreadRng,
    }

    impl RandGen {
        pub fn new() -> Self {
            Self {
                rng: thread_rng(),
            }
        }
    }

    impl GenRandFloat for RandGen {
        fn gen_range(&mut self, range: std::ops::RangeInclusive<f32>) -> f32 {
            let low = *range.start();
            let high = *range.end();
            self.rng.gen_range(low..high)
        }
    }

    const LAYERS_TOPOLOGY: [LayerTopology; 3] = [
        LayerTopology { neurons: 4 + 5 + 24 },
        LayerTopology { neurons: 14 },
        LayerTopology { neurons: 2 },
    ];

    #[test]
    fn weights() {
        let mut rng = RandGen::new();
        let brain = Network::new(&mut rng, &LAYERS_TOPOLOGY);
        let ws  = brain.weights().collect::<Vec<_>>();
        let brain2 = Network::from_weights(&LAYERS_TOPOLOGY, ws);
        assert_eq!(brain.weights().collect::<Vec<_>>(), brain2.weights().collect::<Vec<_>>());
    }

    #[test]
    fn propagate() {
        let layers = (
            Layer{neurons: vec![
                Neuron{bias: 0.0, weights: vec![-0.5, -0.4, -0.3]},
                Neuron{bias: 0.0, weights: vec![-0.2, -0.1, 0.0]},
            ]},
            Layer{neurons:vec![Neuron{bias:0.0,  weights: vec![-0.5, 0.5]}]},
        );
        let network = Network{ layers: vec![layers.0.clone(), layers.1.clone()] };

        let actual = network.propagate(vec![0.5, 0.6, 0.7]);
        let expected = layers.1.propagate(layers.0.propagate(vec![0.5, 0.6, 0.7]));
        
        for (f, s) in std::iter::zip(actual, expected) {
            assert!((f - s).abs() < f32::EPSILON)
        }
    }
}
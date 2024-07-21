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
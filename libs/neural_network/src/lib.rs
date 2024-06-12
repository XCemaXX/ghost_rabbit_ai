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
}

#[derive(Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}
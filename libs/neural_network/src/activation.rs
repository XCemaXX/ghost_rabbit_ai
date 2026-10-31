#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Activation {
    #[default]
    Relu,
    Tanh,
    Linear,
}

impl Activation {
    pub fn apply(self, x: f32) -> f32 {
        match self {
            Activation::Relu => x.max(0.0),
            Activation::Tanh => x.tanh(),
            Activation::Linear => x,
        }
    }

    // Derivative written through the output y = apply(x): backward pass keeps only outputs.
    pub fn derivative(self, y: f32) -> f32 {
        match self {
            Activation::Relu => (y > 0.0) as u8 as f32,
            Activation::Tanh => 1.0 - y * y,
            Activation::Linear => 1.0,
        }
    }
}

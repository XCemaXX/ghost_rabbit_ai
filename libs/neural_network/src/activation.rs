#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Activation {
    #[default]
    Relu,
    Tanh,
    Linear,
}

impl Activation {
    pub fn apply_all(self, xs: &mut [f32]) {
        match self {
            Activation::Relu => xs.iter_mut().for_each(|x| *x = x.max(0.0)),
            Activation::Tanh => xs.iter_mut().for_each(|x| *x = tanh(*x)),
            Activation::Linear => {}
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

// Rational approximation from Eigen: a few ulp from `f32::tanh`, but several times faster.
// Beyond the clamp, tanh is 1 in f32.
fn tanh(x: f32) -> f32 {
    let x = x.clamp(-7.905_311, 7.905_311);
    let x2 = x * x;
    let p = ((((((-2.760_768_5e-16 * x2 + 2.000_188e-13) * x2 - 8.604_672e-11) * x2
        + 5.122_297e-8)
        * x2
        + 1.485_722_4e-5)
        * x2
        + 6.372_619_3e-4)
        * x2
        + 4.893_524_6e-3)
        * x;
    let q = ((1.198_258_4e-6 * x2 + 1.185_347_1e-4) * x2 + 2.268_434_6e-3) * x2 + 4.893_525e-3;
    p / q
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tanh_is_close_to_std() {
        for i in 0..=200_000 {
            let x = (i as f32 / 200_000.0 - 0.5) * 24.0;
            assert!((tanh(x) - x.tanh()).abs() < 1e-6, "x {x}");
        }
    }
}

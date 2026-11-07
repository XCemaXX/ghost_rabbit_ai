use crate::Chromosome;
use crate::GenRandFloat;

pub trait MutationMethod {
    fn mutate<R: GenRandFloat>(&self, rng: &mut R, child: &mut Chromosome);
}

// Each gene changes with the given probability by a shift uniform in [-magnitude, magnitude].
#[derive(Clone, Debug)]
pub struct UniformMutation {
    probability_to_change_genes: f32,
    change_magnitude: f32,
}

impl UniformMutation {
    pub fn new(probability_to_change_genes: f32, change_magnitude: f32) -> Self {
        assert!((0.0..=1.0).contains(&probability_to_change_genes));
        Self {
            probability_to_change_genes,
            change_magnitude,
        }
    }
}

impl MutationMethod for UniformMutation {
    fn mutate<R: GenRandFloat>(&self, rng: &mut R, chromosome: &mut Chromosome) {
        for g in chromosome.iter_mut() {
            if rng.gen_range(0.0..=1.0) < self.probability_to_change_genes {
                *g += rng.gen_range(-self.change_magnitude..=self.change_magnitude);
            }
        }
    }
}

// Each gene changes with the given probability by a normal shift with standard deviation `sigma`.
#[derive(Clone, Debug)]
pub struct GaussianMutation {
    probability_to_change_genes: f32,
    sigma: f32,
}

impl GaussianMutation {
    pub fn new(probability_to_change_genes: f32, sigma: f32) -> Self {
        assert!((0.0..=1.0).contains(&probability_to_change_genes));
        Self {
            probability_to_change_genes,
            sigma,
        }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate<R: GenRandFloat>(&self, rng: &mut R, chromosome: &mut Chromosome) {
        for g in chromosome.iter_mut() {
            if rng.gen_range(0.0..=1.0) < self.probability_to_change_genes {
                *g += self.sigma * standard_normal(rng);
            }
        }
    }
}

// Box-Muller transform.
fn standard_normal<R: GenRandFloat>(rng: &mut R) -> f32 {
    let u1 = rng.gen_range(0.0..=1.0).max(f32::MIN_POSITIVE);
    let u2 = rng.gen_range(0.0..=1.0);
    (-2.0 * u1.ln()).sqrt() * (std::f32::consts::TAU * u2).cos()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Small deterministic xorshift: consecutive values must be independent for Box-Muller.
    struct XorShift(u32);

    impl GenRandFloat for XorShift {
        fn gen_range(&mut self, range: std::ops::RangeInclusive<f32>) -> f32 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 17;
            self.0 ^= self.0 << 5;
            let unit = (self.0 >> 8) as f32 / (1u32 << 24) as f32;
            range.start() + (range.end() - range.start()) * unit
        }
    }

    #[test]
    fn gaussian_shift_has_given_sigma() {
        let mut rng = XorShift(12345);
        let n = 20_000;
        let shifts: Vec<f32> = (0..n).map(|_| 0.5 * standard_normal(&mut rng)).collect();
        let mean = shifts.iter().sum::<f32>() / n as f32;
        let var = shifts.iter().map(|s| (s - mean) * (s - mean)).sum::<f32>() / n as f32;
        assert!(mean.abs() < 0.02, "mean {mean}");
        assert!((var.sqrt() - 0.5).abs() < 0.02, "sigma {}", var.sqrt());
    }

    #[test]
    fn zero_probability_changes_nothing() {
        let mut rng = XorShift(12345);
        let mut chromosome: Chromosome = [1.0, 2.0, 3.0].into_iter().collect();
        GaussianMutation::new(0.0, 1.0).mutate(&mut rng, &mut chromosome);
        UniformMutation::new(0.0, 1.0).mutate(&mut rng, &mut chromosome);
        assert_eq!(
            chromosome.iter().copied().collect::<Vec<_>>(),
            vec![1.0, 2.0, 3.0]
        );
    }
}

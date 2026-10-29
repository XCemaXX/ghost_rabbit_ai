use crate::Chromosome;
use crate::GenRandFloat;

pub trait MutationMethod {
    fn mutate<R: GenRandFloat>(&self, rng: &mut R, child: &mut Chromosome);
}

#[derive(Clone, Debug)]
pub struct GaussianMutation {
    probability_to_change_genes: f32,
    change_magnitude: f32,
}

impl GaussianMutation {
    pub fn new(probability_to_change_genes: f32, change_magnitude: f32) -> Self {
        assert!((0.0..=1.0).contains(&probability_to_change_genes));
        Self {
            probability_to_change_genes,
            change_magnitude,
        }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate<R: GenRandFloat>(&self, rng: &mut R, chromosome: &mut Chromosome) {
        for g in chromosome.iter_mut() {
            if rng.gen_range(0.0..=1.0) < self.probability_to_change_genes {
                *g += rng.gen_range(-self.change_magnitude..=self.change_magnitude);
            }
        }
    }
}

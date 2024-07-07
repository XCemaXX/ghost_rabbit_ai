use crate::GenRandFloat;
use crate::Chromosome;

pub trait MutationMethod {
    fn mutate<R:GenRandFloat>(&self, rng: &mut R, child: &mut Chromosome);
}

#[derive(Clone, Debug)]
pub struct GaussianMutation {
    probability_to_change_genes: f32,
    change_magnitude: f32,
}

impl GaussianMutation {
    pub fn new(probability_to_change_genes: f32, change_magnitude: f32) -> Self {
        assert!(probability_to_change_genes >= 0.0 && probability_to_change_genes <= 1.0);
        Self { probability_to_change_genes, change_magnitude }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate<R:GenRandFloat>(&self, rng: &mut R, chromosome: &mut Chromosome) {
        for g in chromosome.iter_mut() {
            let change = rng.gen_range(-self.probability_to_change_genes..=self.probability_to_change_genes);
            if change > 0.0 {
                *g += rng.gen_range(-self.change_magnitude..=self.change_magnitude);
            }
        }
    }
}
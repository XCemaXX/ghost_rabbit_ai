use crate::Chromosome;
use crate::GenRandFloat;

pub trait CrossoverMethod {
    fn crossover<R: GenRandFloat>(
        &self,
        rng: &mut R,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome;
    fn setup<R: GenRandFloat>(&mut self, rng: &mut R);
}

pub struct BlxACrossover {
    alpha: f32,
}

impl Default for BlxACrossover {
    fn default() -> Self {
        Self::new()
    }
}

impl BlxACrossover {
    pub fn new() -> Self {
        Self { alpha: 0.0 }
    }
}

impl CrossoverMethod for BlxACrossover {
    fn crossover<R: GenRandFloat>(
        &self,
        rng: &mut R,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome {
        assert_eq!(parent_a.len(), parent_b.len());
        parent_a
            .iter()
            .zip(parent_b.iter())
            .map(|(&a, &b)| {
                let cmin = a.min(b);
                let cmax = a.max(b);
                let d = cmax - cmin;
                let start = cmin - d * self.alpha;
                let end = cmax + d * self.alpha;
                if start == end {
                    start
                } else {
                    rng.gen_range(start..=end)
                }
            })
            .collect()
    }

    fn setup<R: GenRandFloat>(&mut self, rng: &mut R) {
        self.alpha = rng.gen_range(0.0..=1.0);
    }
}

// The child is a copy of the first parent: only mutation changes genes.
pub struct NoCrossover;

impl CrossoverMethod for NoCrossover {
    fn crossover<R: GenRandFloat>(
        &self,
        _rng: &mut R,
        parent_a: &Chromosome,
        _parent_b: &Chromosome,
    ) -> Chromosome {
        parent_a.clone()
    }

    fn setup<R: GenRandFloat>(&mut self, _rng: &mut R) {}
}

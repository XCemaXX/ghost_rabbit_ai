use crate::GenRandFloat;
use crate::Individual;

pub trait SelectionMethod {
    fn select<'a, R: GenRandFloat, I: Individual>(&self, rng: &mut R, population: &'a [I])
    -> &'a I;
}

pub struct RouletteWheelSelection;

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, R: GenRandFloat, I: Individual>(
        &self,
        rng: &mut R,
        population: &'a [I],
    ) -> &'a I {
        let fitness_total: f32 = population
            .iter()
            .map(|individual| {
                assert!(individual.fitness() >= 0.0, "roulette needs fitness >= 0");
                individual.fitness()
            })
            .sum();
        if fitness_total == 0.0 {
            // No information: pick uniformly instead of always the last one.
            let i = (rng.gen_range(0.0..=1.0) * population.len() as f32) as usize;
            return &population[i.min(population.len() - 1)];
        }
        let mut r = rng.gen_range(0.0..=fitness_total);
        for i in population {
            r -= i.fitness();
            if r < 0.0 {
                return i;
            }
        }
        population.last().unwrap()
    }
}

// Best of `size` random individuals. Uses only the order of fitness, not its scale.
pub struct TournamentSelection {
    size: usize,
}

impl TournamentSelection {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        Self { size }
    }
}

impl SelectionMethod for TournamentSelection {
    fn select<'a, R: GenRandFloat, I: Individual>(
        &self,
        rng: &mut R,
        population: &'a [I],
    ) -> &'a I {
        (0..self.size)
            .map(|_| {
                let i = (rng.gen_range(0.0..=1.0) * population.len() as f32) as usize;
                &population[i.min(population.len() - 1)]
            })
            .max_by(|a, b| a.fitness().partial_cmp(&b.fitness()).unwrap())
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Chromosome;

    struct Ind(f32);

    impl Individual for Ind {
        fn fitness(&self) -> f32 {
            self.0
        }
        fn chromosome(&self) -> &Chromosome {
            unimplemented!()
        }
        fn create(_: Chromosome) -> Self {
            unimplemented!()
        }
    }

    // Deterministic RNG: walks through [0, 1) in fixed steps.
    struct Steps(f32);

    impl GenRandFloat for Steps {
        fn gen_range(&mut self, range: std::ops::RangeInclusive<f32>) -> f32 {
            self.0 = (self.0 + 0.37) % 1.0;
            range.start() + (range.end() - range.start()) * self.0
        }
    }

    #[test]
    fn tournament_of_whole_population_picks_best() {
        let population: Vec<Ind> = [3.0, 9.0, 1.0, 5.0].into_iter().map(Ind).collect();
        let mut rng = Steps(0.0);
        // With many draws every index appears, so the best always wins.
        let picked = TournamentSelection::new(40).select(&mut rng, &population);
        assert_eq!(picked.fitness(), 9.0);
    }
}

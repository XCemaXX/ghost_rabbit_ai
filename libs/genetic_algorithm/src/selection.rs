use crate::GenRandFloat;
use crate::Individual;

pub trait SelectionMethod {
    fn select<'a, R:GenRandFloat, I: Individual>
    (&self, rng: &mut R, population: &'a [I]) -> &'a I;
}

pub struct RouletteWheelSelection;

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, R:GenRandFloat, I: Individual>
    (&self, rng: &mut R, population: &'a [I]) -> &'a I {
        let fitness_total = population.iter().map(|individual| individual.fitness()).sum();
        let mut r = rng.gen_range(0.0..=fitness_total);
        for i in population {
            r -= i.fitness();
            if r < 0.0 {
                return i;
            }
        }
        return population.last().unwrap();
    }
}
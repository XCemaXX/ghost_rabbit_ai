mod selection;
mod chromosome;
mod crossover;
mod mutation;

use rand_trait::GenRandFloat;
use crossover::CrossoverMethod;
use mutation::MutationMethod;
use selection::SelectionMethod;

pub use chromosome::Chromosome;
pub use crossover::BlxACrossover;
pub use selection::RouletteWheelSelection;
pub use mutation::GaussianMutation;

pub trait Individual {
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
    fn create(chromosome: Chromosome) -> Self;
}

pub struct GeneticAlgorithm<S, C, M> {
    selection_method: S,
    crossover_method: C,
    mutation_method: M,
}

impl<S:SelectionMethod, C:CrossoverMethod, M:MutationMethod> GeneticAlgorithm<S, C, M> {
    pub fn new(selection_method: S, crossover_method: C, mutation_method: M) -> Self {
        Self {
            selection_method,
            crossover_method,
            mutation_method,
        }
    }

    pub fn create_population<I: Individual, R: GenRandFloat>(&mut self, rng: &mut R, population: &[I]) -> Vec<I> {
        assert!(!population.is_empty());
        let mut next_population = Vec::new();

        let parents_count = (population.len() as f32 * 0.3) as usize;
        next_population.append(&mut self.get_top_parents(population, parents_count));

        self.crossover_method.setup(rng);
        let children_count = population.len() - parents_count;
        next_population.append(&mut self.create_children(rng, population, children_count));

        next_population
    }

    fn get_top_parents<I: Individual>(&self, population: &[I], parents_count: usize) -> Vec<I> {
        let mut fitness: Vec<(f32, &I)> = population.iter().map(|individual| (individual.fitness(), individual)).collect();
        fitness.sort_by(|a, b| { b.0.partial_cmp(&a.0).unwrap() });
        fitness[0..parents_count].iter().map(|(_, i)| { I::create(i.chromosome().clone()) }).collect()
    }

    fn create_children<I: Individual, R: GenRandFloat>(&self, rng: &mut R, population: &[I], children_count: usize) -> Vec<I> {
        (0..children_count).map(|_| {
            let parent_a = self.selection_method.select(rng, population).chromosome();
            let parent_b = self.selection_method.select(rng, population).chromosome();
            let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);
            self.mutation_method.mutate(rng, &mut child);
            I::create(child)
        }).collect()
    }
}
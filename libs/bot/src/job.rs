// GA for `policy`: the same generations for the trainer tool and, in small steps, for the game.
use crate::{Bot, Rng, policy_topology, train};
use genetic_algorithm::{
    self as ga, BlxACrossover, GeneticAlgorithm, MutationMethod, RouletteWheelSelection,
    UniformMutation,
};
use neural_network::{LayerTopology, Network};

pub const POPULATION: usize = 100;
// Games per individual and generation; all individuals play the same games.
pub const GAMES: u64 = 4;
pub const TEST_GAMES: u64 = 200;
// Generations the player can stop at. The score grows fast at the start, later neighbours differ only by noise.
pub const POINTS: [usize; 7] = [0, 5, 10, 20, 40, 70, 100];
const ELITE: usize = 30;
const MUTATE_PROBABILITY: f32 = 0.1;
const MUTATE_MAGNITUDE: f32 = 0.3;

struct Individual {
    fitness: f32,
    chromosome: ga::Chromosome,
}

impl ga::Individual for Individual {
    fn fitness(&self) -> f32 {
        self.fitness
    }

    fn chromosome(&self) -> &ga::Chromosome {
        &self.chromosome
    }

    fn create(chromosome: ga::Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome,
        }
    }
}

pub struct PolicyGa {
    ga: GeneticAlgorithm<RouletteWheelSelection, BlxACrossover, UniformMutation>,
    topology: Vec<LayerTopology>,
    rng: Rng,
    pub population: Vec<Network>,
    pub generation: usize,
}

impl PolicyGa {
    pub fn new(mut rng: Rng) -> Self {
        let topology = policy_topology();
        let population = (0..POPULATION)
            .map(|_| Network::new(&mut rng, &topology))
            .collect();
        Self::with_population(rng, topology, population, 0)
    }

    /// The elite and its mutated copies. `generation` is the number of this population.
    pub fn from_elite(mut rng: Rng, elite: &[Network], generation: usize) -> Self {
        let topology = policy_topology();
        let mutation = UniformMutation::new(MUTATE_PROBABILITY, MUTATE_MAGNITUDE);
        let mut population = elite.to_vec();
        while population.len() < POPULATION {
            let parent = &elite[rng.below(elite.len())];
            let mut child: ga::Chromosome = parent.weights().collect();
            mutation.mutate(&mut rng, &mut child);
            population.push(Network::from_weights(&topology, child));
        }
        Self::with_population(rng, topology, population, generation)
    }

    fn with_population(
        rng: Rng,
        topology: Vec<LayerTopology>,
        population: Vec<Network>,
        generation: usize,
    ) -> Self {
        Self {
            ga: GeneticAlgorithm::new(
                RouletteWheelSelection,
                BlxACrossover::new(),
                UniformMutation::new(MUTATE_PROBABILITY, MUTATE_MAGNITUDE),
            ),
            topology,
            rng,
            population,
            generation,
        }
    }

    /// Fitness of one game out of `GAMES`.
    pub fn game_fitness(&self, vision: &Network, individual: usize, game: u64) -> f32 {
        let mut bot = Bot::new(vision, &self.population[individual]);
        let seed = self.generation as u64 * 100 + game;
        train::fitness(|g| bot.act(g), seed)
    }

    pub fn next_generation(&mut self, fitness: &[f32]) {
        let current: Vec<Individual> = self
            .population
            .iter()
            .zip(fitness)
            .map(|(p, &fitness)| Individual {
                fitness,
                chromosome: p.weights().collect(),
            })
            .collect();
        self.population = self
            .ga
            .create_population(&mut self.rng, &current)
            .into_iter()
            .map(|i| Network::from_weights(&self.topology, i.chromosome))
            .collect();
        self.generation += 1;
    }
}

pub fn best(fitness: &[f32]) -> usize {
    (0..fitness.len())
        .max_by(|&a, &b| fitness[a].partial_cmp(&fitness[b]).unwrap())
        .unwrap()
}

/// Score of one held-out game.
pub fn test_game(vision: &Network, policy: &Network, game: u64) -> f32 {
    let mut bot = Bot::new(vision, policy);
    train::score(|g| bot.act(g), 1_000_000 + game)
}

pub fn test_mean(mut scores: Vec<f32>) -> f32 {
    // Sorted sum: the same mean as the research log printed.
    scores.sort_by(|a, b| a.partial_cmp(b).unwrap());
    scores.iter().sum::<f32>() / scores.len() as f32
}

/// A finished point of a policy run.
pub struct Point {
    pub generation: usize,
    pub policy: Network,
    pub score: f32,
    pub elite: Vec<Network>,
    pub rng: Rng,
}

/// GA up to `target`, one game per `step`.
pub struct PolicyJob {
    vision: Network,
    ga: PolicyGa,
    start: usize,
    target: usize,
    fitness: Vec<f32>,
    sum: f32,
    game: u64,
    test: Option<Test>,
    done: bool,
}

struct Test {
    best: usize,
    scores: Vec<f32>,
}

impl PolicyJob {
    pub fn new(vision: Network, ga: PolicyGa, target: usize) -> Self {
        let start = ga.generation;
        Self {
            vision,
            ga,
            start,
            target,
            fitness: Vec::new(),
            sum: 0.0,
            game: 0,
            test: None,
            done: false,
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn progress(&self) -> f32 {
        let total = (self.target - self.start + 1) as f32;
        let part = self.fitness.len() as f32 / POPULATION as f32;
        (self.ga.generation - self.start) as f32 / total + part / total
    }

    /// Plays one game. Returns a point when it is finished.
    pub fn step(&mut self) -> Option<Point> {
        if let Some(test) = &mut self.test {
            let policy = &self.ga.population[test.best];
            let game = test.scores.len() as u64;
            test.scores.push(test_game(&self.vision, policy, game));
            if test.scores.len() < TEST_GAMES as usize {
                return None;
            }
            return Some(self.finish_point());
        }
        let individual = self.fitness.len();
        self.sum += self.ga.game_fitness(&self.vision, individual, self.game);
        self.game += 1;
        if self.game < GAMES {
            return None;
        }
        self.fitness.push(self.sum / GAMES as f32);
        self.sum = 0.0;
        self.game = 0;
        if self.fitness.len() == POPULATION {
            if POINTS.contains(&self.ga.generation) {
                self.test = Some(Test {
                    best: best(&self.fitness),
                    scores: Vec::new(),
                });
            } else {
                self.next_generation();
            }
        }
        None
    }

    fn finish_point(&mut self) -> Point {
        let test = self.test.take().unwrap();
        let mut order: Vec<usize> = (0..POPULATION).collect();
        order.sort_by(|&a, &b| self.fitness[b].partial_cmp(&self.fitness[a]).unwrap());
        let point = Point {
            generation: self.ga.generation,
            policy: self.ga.population[test.best].clone(),
            score: test_mean(test.scores),
            elite: order[..ELITE]
                .iter()
                .map(|&i| self.ga.population[i].clone())
                .collect(),
            rng: self.ga.rng.clone(),
        };
        if self.ga.generation == self.target {
            self.done = true;
        } else {
            self.next_generation();
        }
        point
    }

    fn next_generation(&mut self) {
        let fitness = std::mem::take(&mut self.fitness);
        self.ga.next_generation(&fitness);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vision_topology;

    #[test]
    fn from_elite_fills_the_population() {
        let mut rng = Rng(1);
        let elite: Vec<Network> = (0..ELITE)
            .map(|_| Network::new(&mut rng, &policy_topology()))
            .collect();
        let ga = PolicyGa::from_elite(rng, &elite, 6);
        assert_eq!(ga.population.len(), POPULATION);
        assert_eq!(ga.generation, 6);
    }

    #[test]
    fn job_ends_with_a_point_at_the_target() {
        let vision = Network::new(&mut Rng(2), &vision_topology());
        let mut job = PolicyJob::new(vision, PolicyGa::new(Rng(3)), 0);
        let mut points = Vec::new();
        while !job.is_done() {
            points.extend(job.step());
        }
        assert_eq!(points.len(), 1);
        assert_eq!(points[0].generation, 0);
        assert_eq!(points[0].elite.len(), ELITE);
    }
}

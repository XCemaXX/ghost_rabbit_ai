// GA for `policy`: the same generations for the trainer tool and, in small steps, for the game.
use crate::train::{self, TestResult};
use crate::{Bot, Rng, policy_topology};
use genetic_algorithm::{
    self as ga, BlxACrossover, GeneticAlgorithm, RouletteWheelSelection, UniformMutation,
};
use neural_network::Network;

pub const POPULATION: usize = 50;
// Games per individual and generation; all individuals play the same games.
const GAMES: u64 = 4;
pub const TEST_GAMES: u64 = 200;
// Generations the player can stop at, and those a policy file holds.
// Life grows fast at the start, later neighbours differ only by noise.
pub const POINTS: [usize; 8] = [0, 1, 2, 5, 10, 20, 30, 50];
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
    rng: Rng,
    population: Vec<Network>,
    generation: usize,
}

impl PolicyGa {
    pub fn new(mut rng: Rng) -> Self {
        let topology = policy_topology();
        let population = (0..POPULATION)
            .map(|_| Network::new(&mut rng, &topology))
            .collect();
        Self::resume(Resume {
            generation: 0,
            population,
            rng,
        })
    }

    pub fn resume(state: Resume) -> Self {
        Self {
            ga: GeneticAlgorithm::new(
                RouletteWheelSelection,
                BlxACrossover::new(),
                UniformMutation::new(MUTATE_PROBABILITY, MUTATE_MAGNITUDE),
            ),
            rng: state.rng,
            population: state.population,
            generation: state.generation,
        }
    }

    pub fn population(&self) -> &[Network] {
        &self.population
    }

    pub fn mean_fitness(&self, vision: &Network, individual: usize) -> f32 {
        let sum: f32 = (0..GAMES)
            .map(|game| self.game_fitness(vision, individual, game))
            .sum();
        sum / GAMES as f32
    }

    fn game_fitness(&self, vision: &Network, individual: usize, game: u64) -> f32 {
        let mut bot = Bot::new(vision, &self.population[individual]);
        let seed = self.generation as u64 * 100 + game;
        train::fitness(|g| bot.act(g), seed)
    }

    pub fn next_generation(&mut self, fitness: &[f32]) {
        assert_eq!(fitness.len(), self.population.len());
        let topology = policy_topology();
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
            .map(|i| Network::from_weights(&topology, i.chromosome))
            .collect();
        self.generation += 1;
    }
}

pub fn best_index(fitness: &[f32]) -> usize {
    (0..fitness.len())
        .max_by(|&a, &b| fitness[a].partial_cmp(&fitness[b]).unwrap())
        .unwrap()
}

/// One held-out game.
pub fn test_game(vision: &Network, policy: &Network, game: u64) -> TestResult {
    let mut bot = Bot::new(vision, policy);
    train::test_result(|g| bot.act(g), 1_000_000 + game)
}

/// A few very long games would dominate the mean. Each median is taken on its own.
pub fn median_result(results: &[TestResult]) -> TestResult {
    let median = |value: fn(&TestResult) -> f32| {
        let mut values: Vec<f32> = results.iter().map(value).collect();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        values[values.len() / 2]
    };
    TestResult {
        life: median(|r| r.life),
        score: median(|r| r.score),
    }
}

/// A finished point of a policy run.
pub struct Point {
    pub generation: usize,
    pub policy: Network,
    pub result: TestResult,
}

/// The state of a run after a point: going on from it gives the same run as without a stop.
#[derive(Clone)]
pub struct Resume {
    pub generation: usize,
    pub population: Vec<Network>,
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
    results: Vec<TestResult>,
}

impl PolicyJob {
    pub fn new(vision: Network, ga: PolicyGa, target: usize) -> Self {
        let start = ga.generation;
        assert!(start <= target && POINTS.contains(&target));
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
        if self.done {
            return 1.0;
        }
        let total = generations_cost(self.start..=self.target);
        let finished = generations_cost(self.start..self.ga.generation);
        let games = self.fitness.len() as f32 / POPULATION as f32;
        let tests = self
            .test
            .as_ref()
            .map_or(0.0, |test| test.results.len() as f32 / TEST_GAMES as f32);
        (finished + games + tests) / total
    }

    /// Plays one game. Returns a point when it is finished.
    pub fn step(&mut self) -> Option<(Point, Resume)> {
        if self.done {
            return None;
        }
        if let Some(test) = &mut self.test {
            let policy = &self.ga.population[test.best];
            let game = test.results.len() as u64;
            test.results.push(test_game(&self.vision, policy, game));
            if test.results.len() < TEST_GAMES as usize {
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
                    best: best_index(&self.fitness),
                    results: Vec::new(),
                });
            } else {
                self.next_generation();
            }
        }
        None
    }

    fn finish_point(&mut self) -> (Point, Resume) {
        let test = self.test.take().unwrap();
        let point = Point {
            generation: self.ga.generation,
            policy: self.ga.population[test.best].clone(),
            result: median_result(&test.results),
        };
        self.done = self.ga.generation == self.target;
        self.next_generation();
        let resume = Resume {
            generation: self.ga.generation,
            population: self.ga.population.clone(),
            rng: self.ga.rng.clone(),
        };
        (point, resume)
    }

    fn next_generation(&mut self) {
        let fitness = std::mem::take(&mut self.fitness);
        self.ga.next_generation(&fitness);
    }
}

// A point plays its test games too: about as many as a generation.
fn generations_cost(generations: impl Iterator<Item = usize>) -> f32 {
    generations
        .map(|g| if POINTS.contains(&g) { 2.0 } else { 1.0 })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vision_topology;

    #[test]
    fn medians_are_taken_separately() {
        let result = |life, score| TestResult { life, score };
        let results = [result(1.0, 30.0), result(2.0, 10.0), result(3.0, 20.0)];
        assert_eq!(median_result(&results), result(2.0, 20.0));
    }

    fn run(mut job: PolicyJob) -> Vec<(Point, Resume)> {
        let mut points = Vec::new();
        let mut progress = 0.0;
        while !job.is_done() {
            points.extend(job.step());
            assert!(job.progress() >= progress);
            progress = job.progress();
        }
        assert_eq!(progress, 1.0);
        points
    }

    #[test]
    fn a_stopped_run_goes_on_as_one_run() {
        let vision = Network::new(&mut Rng(2), &vision_topology());
        let weights = |p: &Point| p.policy.weights().collect::<Vec<_>>();
        let target = POINTS[1];
        let whole = run(PolicyJob::new(
            vision.clone(),
            PolicyGa::new(Rng(3)),
            target,
        ));

        let first = run(PolicyJob::new(
            vision.clone(),
            PolicyGa::new(Rng(3)),
            POINTS[0],
        ));
        let resume = first[0].1.clone();
        let rest = run(PolicyJob::new(vision, PolicyGa::resume(resume), target));

        assert_eq!(rest[0].0.generation, target);
        assert_eq!(weights(&rest[0].0), weights(&whole[1].0));
    }
}

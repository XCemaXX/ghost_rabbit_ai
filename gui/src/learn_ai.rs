
use game_logic::{Difficulty, GameState, MoveDirection};
use crate::rand_gen::RandGen;
use neural_network::{LayerTopology, Network};
use genetic_algorithm::{self as ga, GeneticAlgorithm};

const SIMULATION_SECS: f32 = 10.0; // CHANGE. NEED TO SETUP
const DT: f32 = 1.0 / 60.0;
const SIMULATION_STEPS: usize = (SIMULATION_SECS / DT) as usize;
const SIMULATION_GENERATIONS: usize = 100; // CHANGE. NEED TO SETUP
const POPULATION_SIZE: usize = 100; // CHANGE. NEED TO SETUP
const LAYERS_TOPOLOGY: [LayerTopology; 3] = [
    LayerTopology { neurons: 4 + 5 + 24 },
    LayerTopology { neurons: 14 }, // CHANGE. NEED TO SETUP. try 8-18 
    LayerTopology { neurons: 1 },
];

struct AiPlayer {
    brain: Network
}

struct AiPlayerIndividual {
    fitness: f32,
    chromosome: ga::Chromosome,
}

impl ga::Individual for AiPlayerIndividual {
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

impl AiPlayerIndividual {
    pub fn from_player(player: &AiPlayer, difficulty: Difficulty) -> Self {
        Self {
            fitness: player.fitness(difficulty),
            chromosome: player.brain.weights().collect(),
        }
    }

    pub fn into_player(self, topology: &[LayerTopology]) -> AiPlayer {
        AiPlayer {
            brain: Network::from_weights(topology, self.chromosome),
        }
    }
}

impl AiPlayer {
    fn fitness(&self, difficulty: Difficulty) -> f32 {
        let mut game_engine = GameState::new(RandGen{}, difficulty);
        for _ in 0..SIMULATION_STEPS {
            let inputs_ai = generate_inputs_for_ai(&game_engine);
            let move_direction = get_next_move_by_ai(&self.brain, inputs_ai);
            game_engine.move_player_by_x(DT, move_direction);
            let is_game_over = game_engine.next_step(DT);
            if is_game_over {
                break;
            }
        }
        game_engine.get_score() as f32
    }
}

pub fn learn_ai(difficulty: Difficulty) -> Network {
    let mut ga = GeneticAlgorithm::new(
        ga::RouletteWheelSelection, 
        ga::BlxACrossover::new(), 
        ga::GaussianMutation::new(0.01, 0.1)); // CHANGE. NEED TO SETUP
    
    let mut rng = RandGen{};
    let mut players = create_random_players(&mut rng, &LAYERS_TOPOLOGY);
    for _ in 0..SIMULATION_GENERATIONS {
        let current_population: Vec<_> = players
            .iter()
            .map(|player| { AiPlayerIndividual::from_player(player, difficulty) })
            .collect();
        let evolved_population = ga.create_population(&mut rng, &current_population);
        players = evolved_population
            .into_iter()
            .map(|individual| individual.into_player(&LAYERS_TOPOLOGY))
            .collect();
    }

    players.into_iter().max_by_key(
        |p| {
            p.fitness(difficulty) as usize
        }
    ).unwrap().brain
}

fn create_random_players(rng: &mut RandGen, topology: &[LayerTopology]) -> Vec<AiPlayer> {
    let mut players = Vec::new();
    for _ in 0..POPULATION_SIZE {
        let player = AiPlayer{
            brain: Network::new(rng, &topology),
        };
        players.push(player);
    }
    players
}

pub fn generate_inputs_for_ai(game: &GameState<RandGen>) -> Vec<f32> {
    let mut inputs = Vec::new();
    inputs.push(game.player.position.x);
    inputs.push(game.player.position.y);
    inputs.push(game.player.speed.x);
    inputs.push(game.player.speed.y);
    inputs.push(game.monster.position.x);
    inputs.push(game.monster.position.y);
    inputs.push(game.monster.speed.x);
    inputs.push(game.monster.speed.y);
    inputs.push(game.monster.is_dead as i32 as f32);
    let mut floor_positions: Vec<(f32, f32)> = game.floors.iter().map(|f| {
        (f.position.x, f.position.y)
    }).collect();
    floor_positions.sort_by(|a, b| { a.1.partial_cmp(&b.1).unwrap() });
    for (x, y) in floor_positions {
        inputs.push(x);
        inputs.push(y);
    }
    inputs
}

pub fn get_next_move_by_ai(network: &Network, inputs: Vec<f32>) -> MoveDirection {
    let response = network.propagate(inputs);
    if response[0] < 0.33 {
        MoveDirection::Left
    } else if response[0] > 0.66 {
        MoveDirection::Right
    } else {
        MoveDirection::None
    }
}
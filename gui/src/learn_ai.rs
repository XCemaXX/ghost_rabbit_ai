
use game_logic::{Difficulty, GameState, MoveDirection};
use crate::rand_gen::RandGen;
use neural_network::{LayerTopology, Network};
use genetic_algorithm::{self as ga, GeneticAlgorithm};

const SIMULATION_SECS: f32 = 7.0; // CHANGE. NEED TO SETUP
const DT: f32 = 1.0 / 60.0;
const SIMULATION_STEPS: usize = (SIMULATION_SECS / DT) as usize;
const SIMULATION_GENERATIONS: usize = 500; // CHANGE. NEED TO SETUP
const POPULATION_SIZE: usize = 100; // CHANGE. NEED TO SETUP
const LAYERS_TOPOLOGY: [LayerTopology; 3] = [
    LayerTopology { neurons: 4 + 5 + 4  }, //+ 24
    LayerTopology { neurons: 5 }, // CHANGE. NEED TO SETUP. try 8-18 
    LayerTopology { neurons: 2 },
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
            let is_game_over = game_engine.next_step(DT).game_over;
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
        ga::GaussianMutation::new(0.01, 0.03)); // CHANGE. NEED TO SETUP
    
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
    inputs.push(floor_positions[0].0);
    inputs.push(floor_positions[0].1);
    inputs.push(floor_positions[1].0);
    inputs.push(floor_positions[1].1);
    /*for (x, y) in floor_positions {
        inputs.push(x);
        inputs.push(y);
    }*/
    inputs
}

pub fn get_next_move_by_ai(network: &Network, inputs: Vec<f32>) -> MoveDirection {
    let response = network.propagate(inputs);
    if response[0] > 0.1 && response[1] < 0.1 {
        MoveDirection::Left
    } else if response[0] < 0.1 && response[1] > 0.1 {
        MoveDirection::Right
    } else {
        MoveDirection::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn learn_ai_test() {
        let difficulty = Difficulty::Unreal;
        let _ai_player = learn_ai(difficulty);
    }

    struct CodeSolver {
        network: Network
    }

    struct CodeSolverIndividual {
        fitness: f32,
        chromosome: ga::Chromosome,
    }
    
    impl ga::Individual for CodeSolverIndividual {
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

    impl CodeSolverIndividual {
        pub fn from_solver(s: &CodeSolver) -> Self {
            Self {
                fitness: s.fitness(),
                chromosome: s.network.weights().collect(),
            }
        }
    
        pub fn into_solver(self, topology: &[LayerTopology]) -> CodeSolver {
            CodeSolver {
                network: Network::from_weights(topology, self.chromosome),
            }
        }
    }

    fn to_input(i: i32) -> Vec<f32>{
        vec![(i & 1) as f32, ((i & 2) >> 1) as f32, ((i & 4) >> 2) as f32, ((i & 8) >> 3) as f32]
    }

    impl CodeSolver {
        fn fitness(&self) -> f32 {
            let test_inputs = vec![0, 2, 3, 5, 7, 8, 10, 11, 14, 15];
            let test_outputs = vec![15, 13, 12, 10, 8, 7, 5, 4, 1, 0];

            let mut dev_sum = 0.0;
            for (i, o) in std::iter::zip(test_inputs, test_outputs) {
                let fi = to_input(i);
                let fo = self.network.propagate(fi);
                let mut deviation = (fo[0] - (o & 1) as f32).abs();
                deviation += (fo[1] - ((o & 2) >> 1) as f32).abs();
                deviation += (fo[2] - ((o & 4) >> 2) as f32).abs();
                deviation += (fo[3] - ((o & 8) >> 3) as f32).abs();

                dev_sum += deviation;
            }
            1.0 / dev_sum
        }
    }

    #[test]
    fn code() {
        let pop_size = 100;
        let sim_generations = 100;
        let mut rng = RandGen{};
        let topology: [LayerTopology; 2] = [
            LayerTopology { neurons: 4 },
            LayerTopology { neurons: 4 },
        ];
        let mut networks = Vec::new();
        
        for _ in 0..pop_size {
            let s = CodeSolver{
                network: Network::new(&mut rng, &topology),
            };
            networks.push(s);
        }

        let mut ga = GeneticAlgorithm::new(
            ga::RouletteWheelSelection, 
            ga::BlxACrossover::new(), 
            ga::GaussianMutation::new(0.01, 0.03));

        for _ in 0..sim_generations {
            let current_population: Vec<_> = networks
                .iter()
                .map(|s| { CodeSolverIndividual::from_solver(s) })
                .collect();
            let evolved_population = ga.create_population(&mut rng, &current_population);
            networks = evolved_population
                .into_iter()
                .map(|individual| individual.into_solver(&topology))
                .collect();
        }
    
        let res = networks.into_iter().max_by_key(
            |s| {
                s.fitness() as usize
            }
        ).unwrap().network;
        let mut equals = 0;
        for i in 0..16 {
            let cv = res.propagate(to_input(i));
            print!("{}: ", i);
            let r = (cv[0] > 0.5) as i32 + (((cv[1] > 0.5) as i32) << 1) + (((cv[2]>0.5) as i32) << 2) + (((cv[3]>0.5) as i32) << 3);
            println!("{}", r);
            equals += ((15 - i) == r) as i32;
        }
        assert!(equals > 13);
    }
}
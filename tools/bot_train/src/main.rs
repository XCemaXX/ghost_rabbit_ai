// Trains the bot and writes its weights to resources/bot.
// Run by hand (`cargo run --release -p bot_train`): it takes minutes, so CI does not run it.
use bot::job::{self, GAMES, POPULATION, PolicyGa, TEST_GAMES};
use bot::train::{Sample, loss_grad, play_and_label};
use bot::{GENERATION_STEP, MAX_GENERATION, Rng, to_bytes, vision_topology};
use neural_network::{Network, train::Adam};
use rayon::prelude::*;
use std::path::Path;

const VISION_ROUNDS: usize = 8;
const VISION_GAMES: u64 = 1500;
const VISION_EPOCHS: usize = 8;
// A large batch gives the threads large tasks; a higher learning rate makes up for fewer steps.
const BATCH: usize = 2048;
const PARTS: usize = 16;
const LEARNING_RATE: f32 = 8.5e-3;

fn main() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../resources/bot");
    std::fs::create_dir_all(&dir).unwrap();
    let visions = train_vision();
    for (round, vision) in visions.iter().enumerate() {
        std::fs::write(dir.join(format!("vision_r{round}.bin")), to_bytes(vision)).unwrap();
    }

    // One GA per vision round; rounds run side by side.
    let policies: Vec<Vec<(usize, f32, Network)>> = visions
        .par_iter()
        .enumerate()
        .map(|(round, vision)| train_policy(vision, round))
        .collect();
    let mut scores = String::from("round,generation,heldout_mean\n");
    for (round, checkpoints) in policies.iter().enumerate() {
        let mut bytes = Vec::new();
        for (generation, mean, policy) in checkpoints {
            bytes.extend(to_bytes(policy));
            scores += &format!("{round},{generation},{mean:.0}\n");
        }
        std::fs::write(dir.join(format!("policy_r{round}.bin")), bytes).unwrap();
    }
    std::fs::write(dir.join("scores.csv"), scores).unwrap();
}

// DAgger: each round the current net plays, the game copy labels the frames, the net trains on all frames.
fn train_vision() -> Vec<Network> {
    let mut rng = Rng(4);
    let mut vision = Network::new(&mut rng, &vision_topology());
    let mut data: Vec<Sample> = Vec::new();
    let mut rounds = Vec::new();
    for round in 0..VISION_ROUNDS as u64 {
        let seeds: Vec<u64> = (0..VISION_GAMES)
            .map(|i| 90_000 + round * 100_000 + i)
            .collect();
        let player = (round > 0).then_some(&vision);
        let games: Vec<Vec<Sample>> = seeds
            .par_iter()
            .map(|&s| play_and_label(player, s))
            .collect();
        data.extend(games.into_iter().flatten());
        fit(&mut vision, &data, &mut rng);
        println!("vision round {round}: {} frames", data.len());
        rounds.push(vision.clone());
    }
    rounds
}

fn fit(vision: &mut Network, data: &[Sample], rng: &mut Rng) {
    let n = vision.weights_count();
    let mut adam = Adam::new(n, LEARNING_RATE);
    let mut order: Vec<usize> = (0..data.len()).collect();
    for epoch in 0..VISION_EPOCHS {
        for i in (1..order.len()).rev() {
            order.swap(i, rng.below(i + 1));
        }
        adam.set_learning_rate(
            LEARNING_RATE * (1.0 - epoch as f32 / VISION_EPOCHS as f32).max(0.1),
        );
        for batch in order.chunks(BATCH) {
            // Fixed split into parts: the sum order, so the result, does not depend on threads.
            let parts: Vec<&[usize]> = batch.chunks(batch.len().div_ceil(PARTS)).collect();
            let grads: Vec<Vec<f32>> = parts
                .par_iter()
                .map(|part| {
                    let mut grad = vec![0.0; n];
                    let inputs: Vec<&[f32]> = part.iter().map(|&i| &data[i].inputs[..]).collect();
                    let labels =
                        |k: usize, output: &[f32]| loss_grad(output, &data[part[k]].labels);
                    vision.backward_batch(&inputs, labels, &mut grad);
                    grad
                })
                .collect();
            let mut grad = vec![0.0; n];
            for part in grads {
                for (g, p) in grad.iter_mut().zip(part) {
                    *g += p;
                }
            }
            grad.iter_mut().for_each(|g| *g /= batch.len() as f32);
            adam.step(vision, &grad);
        }
    }
}

// Returns (generation, held-out mean score, best policy) every GENERATION_STEP generations.
fn train_policy(vision: &Network, round: usize) -> Vec<(usize, f32, Network)> {
    let mut ga = PolicyGa::new(Rng(21));
    let mut checkpoints = Vec::new();
    for generation in 0..=MAX_GENERATION {
        let fitness: Vec<f32> = (0..POPULATION)
            .into_par_iter()
            .map(|i| {
                (0..GAMES)
                    .map(|k| ga.game_fitness(vision, i, k))
                    .sum::<f32>()
                    / GAMES as f32
            })
            .collect();
        if generation.is_multiple_of(GENERATION_STEP) {
            let policy = &ga.population[job::best(&fitness)];
            let scores = (0..TEST_GAMES)
                .into_par_iter()
                .map(|k| job::test_game(vision, policy, k))
                .collect();
            let mean = job::test_mean(scores);
            println!("round {round} generation {generation}: {mean:.0}");
            checkpoints.push((generation, mean, policy.clone()));
        }
        ga.next_generation(&fitness);
    }
    checkpoints
}

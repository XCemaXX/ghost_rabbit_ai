// Trains the bot and writes its weights to resources/bot.
// Run by hand (`cargo run --release -p bot_train`): it takes minutes, so CI does not run it.
use bot::policy_job::{self, GAMES, POPULATION, PolicyGa, TEST_GAMES};
use bot::train::{Sample, TestResult, play_and_label};
use bot::vision_job::{self, VISION_EPOCHS, VISION_GAMES, VISION_ROUNDS};
use bot::{GENERATION_STEP, MAX_GENERATION, Rng, to_bytes, vision_topology};
use neural_network::Network;
use rayon::prelude::*;
use std::path::Path;

fn main() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../resources/bot");
    std::fs::create_dir_all(&dir).unwrap();
    let visions = train_vision();
    for (round, vision) in visions.iter().enumerate() {
        std::fs::write(dir.join(format!("vision_r{round}.bin")), to_bytes(vision)).unwrap();
    }

    // One GA per vision round; rounds run side by side.
    let policies: Vec<Vec<(usize, TestResult, Network)>> = visions
        .par_iter()
        .enumerate()
        .map(|(round, vision)| train_policy(vision, round))
        .collect();
    let mut scores = String::from("round,generation,median_life,median_score\n");
    for (round, checkpoints) in policies.iter().enumerate() {
        let mut bytes = Vec::new();
        for (generation, result, policy) in checkpoints {
            bytes.extend(to_bytes(policy));
            let (life, score) = (result.life, result.score);
            scores += &format!("{round},{generation},{life:.0},{score:.0}\n");
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
            .map(|i| vision_job::vision_seed(round, i))
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
    let mut adam = vision_job::new_adam(vision);
    let mut order: Vec<usize> = (0..data.len()).collect();
    for epoch in 0..VISION_EPOCHS {
        for i in (1..order.len()).rev() {
            order.swap(i, rng.below(i + 1));
        }
        adam.set_learning_rate(vision_job::epoch_learning_rate(epoch));
        for batch in order.chunks(vision_job::BATCH) {
            let parts: Vec<&[usize]> = batch.chunks(vision_job::part_size(batch.len())).collect();
            let grads: Vec<Vec<f32>> = parts
                .par_iter()
                .map(|part| vision_job::part_grad(vision, data, part))
                .collect();
            vision_job::apply_batch(&mut adam, vision, grads, batch.len());
        }
    }
}

// Returns (generation, held-out medians, best policy) every GENERATION_STEP generations.
fn train_policy(vision: &Network, round: usize) -> Vec<(usize, TestResult, Network)> {
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
            let policy = &ga.population[policy_job::best(&fitness)];
            let results: Vec<TestResult> = (0..TEST_GAMES)
                .into_par_iter()
                .map(|k| policy_job::test_game(vision, policy, k))
                .collect();
            let result = policy_job::median_result(&results);
            let (life, score) = (result.life, result.score);
            println!("round {round} generation {generation}: {life:.0} s, {score:.0} points");
            checkpoints.push((generation, result, policy.clone()));
        }
        ga.next_generation(&fitness);
    }
    checkpoints
}

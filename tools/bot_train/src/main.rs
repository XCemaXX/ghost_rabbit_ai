// Trains the bot and writes its weights to resources/bot.
// Run by hand (`cargo run --release -p bot_train`): it takes minutes, so CI does not run it.
use bot::policy_job::{self, POINTS, POPULATION, Point, PolicyGa, TEST_GAMES};
use bot::train::{Sample, TestResult, play_and_label};
use bot::vision_job::{self, VISION_GAMES, VISION_ROUNDS};
use bot::{Network, Rng, to_bytes, vision_topology};
use rayon::prelude::*;
use std::path::Path;

// A fixed split: the sum order, so the weights, do not depend on the number of threads.
const PARTS: usize = 16;

fn main() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../resources/bot");
    std::fs::create_dir_all(&dir).unwrap();
    let visions = train_vision();
    for (round, vision) in visions.iter().enumerate() {
        std::fs::write(dir.join(format!("vision_r{round}.bin")), to_bytes(vision)).unwrap();
    }

    // One GA per vision round; rounds run side by side.
    let policies: Vec<Vec<Point>> = visions
        .par_iter()
        .enumerate()
        .map(|(round, vision)| train_policy(vision, round))
        .collect();
    let mut scores = String::from("round,generation,median_life,median_score\n");
    for (round, points) in policies.iter().enumerate() {
        let mut bytes = Vec::new();
        for point in points {
            bytes.extend(to_bytes(&point.policy));
            let (generation, life, score) =
                (point.generation, point.result.life, point.result.score);
            scores += &format!("{round},{generation},{life:.0},{score:.0}\n");
        }
        std::fs::write(dir.join(format!("policy_r{round}.bin")), bytes).unwrap();
    }
    std::fs::write(dir.join("scores.csv"), scores).unwrap();
}

// DAgger: each round the current net plays, the game copy labels the frames,
// the net trains on the frames of this and the previous round.
fn train_vision() -> Vec<Network> {
    let mut rng = Rng(4);
    let mut vision = Network::new(&mut rng, &vision_topology());
    let mut data: Vec<Sample> = Vec::new();
    let mut rounds = Vec::new();
    for round in 0..VISION_ROUNDS {
        let player = (round > 0).then_some(&vision);
        let games: Vec<Vec<Sample>> = (0..VISION_GAMES)
            .into_par_iter()
            .map(|i| play_and_label(player, vision_job::vision_seed(round, i)))
            .collect();
        let old_frames = data.len();
        data.extend(games.into_iter().flatten());
        fit(
            &mut vision,
            &data,
            &mut rng,
            vision_job::round_epochs(round),
        );
        println!("vision round {round}: {} frames", data.len());
        data.drain(..old_frames);
        rounds.push(vision.clone());
    }
    rounds
}

fn fit(vision: &mut Network, data: &[Sample], rng: &mut Rng, epochs: usize) {
    let mut adam = vision_job::new_adam(vision);
    let mut order: Vec<usize> = (0..data.len()).collect();
    for epoch in 0..epochs {
        for i in (1..order.len()).rev() {
            order.swap(i, rng.below(i + 1));
        }
        adam.set_learning_rate(vision_job::epoch_learning_rate(epoch, epochs));
        for batch in order.chunks(vision_job::BATCH) {
            let parts: Vec<&[usize]> = batch.chunks(batch.len().div_ceil(PARTS)).collect();
            let grads: Vec<Vec<f32>> = parts
                .par_iter()
                .map(|part| vision_job::part_grad(vision, data, part))
                .collect();
            vision_job::apply_batch(&mut adam, vision, grads, batch.len());
        }
    }
}

// Returns the policy of each of `POINTS`.
fn train_policy(vision: &Network, round: usize) -> Vec<Point> {
    let mut ga = PolicyGa::new(Rng(21));
    let mut points = Vec::new();
    for generation in 0..=*POINTS.last().unwrap() {
        let fitness: Vec<f32> = (0..POPULATION)
            .into_par_iter()
            .map(|i| ga.mean_fitness(vision, i))
            .collect();
        if POINTS.contains(&generation) {
            let policy = &ga.population()[policy_job::best_index(&fitness)];
            let results: Vec<TestResult> = (0..TEST_GAMES)
                .into_par_iter()
                .map(|k| policy_job::test_game(vision, policy, k))
                .collect();
            let result = policy_job::median_result(&results);
            let (life, score) = (result.life, result.score);
            println!("round {round} generation {generation}: {life:.0} s, {score:.0} points");
            points.push(Point {
                generation,
                policy: policy.clone(),
                result,
            });
        }
        ga.next_generation(&fitness);
    }
    points
}

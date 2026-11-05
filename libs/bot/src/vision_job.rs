// DAgger for `vision`: helpers shared with the trainer tool, and a job in small steps for the game.
use crate::train::{self, LabeledGame, Sample};
use crate::{Rng, vision_topology};
use neural_network::Network;
use neural_network::train::Adam;

pub const VISION_ROUNDS: usize = 8;
pub const VISION_GAMES: u64 = 1500;
// The first net learns from nothing and then chooses the frames of the next round: it needs more.
const FIRST_ROUND_EPOCHS: usize = 8;
const EPOCHS: usize = 3;
// Frames of this round and the one before. All frames would make `fit` grow with the square of rounds.
const WINDOW_ROUNDS: usize = 2;
pub const BATCH: usize = 512;
const LEARNING_RATE: f32 = 5e-3;
// Swaps per step: shuffling all frames at once would take several screen frames.
const SHUFFLE_PER_STEP: usize = 16_384;
// Time of one data round against one frame and epoch of `fit`, for the progress bar.
const COLLECT_COST: f32 = 6.0;
const FIT_FRAME_COST: f32 = 4.6e-6;
// A little above the usual count: going over it doubles the memory of the frames.
const EXPECTED_NEW_FRAMES: usize = 256_000;

pub fn vision_seed(round: u64, game: u64) -> u64 {
    90_000 + round * 100_000 + game
}

pub fn new_adam(vision: &Network) -> Adam {
    Adam::new(vision.weights_count(), LEARNING_RATE)
}

pub fn round_epochs(round: u64) -> usize {
    if round == 0 {
        FIRST_ROUND_EPOCHS
    } else {
        EPOCHS
    }
}

pub fn epoch_learning_rate(epoch: usize, epochs: usize) -> f32 {
    LEARNING_RATE * (1.0 - epoch as f32 / epochs as f32).max(0.1)
}

pub fn part_grad(vision: &Network, data: &[Sample], part: &[usize]) -> Vec<f32> {
    let mut grad = vec![0.0; vision.weights_count()];
    let inputs: Vec<&[f32]> = part.iter().map(|&i| &data[i].inputs[..]).collect();
    let labels = |k: usize, output: &[f32]| train::loss_grad(output, &data[part[k]].labels);
    vision.backward_batch(&inputs, labels, &mut grad);
    grad
}

pub fn apply_batch(adam: &mut Adam, vision: &mut Network, grads: Vec<Vec<f32>>, batch: usize) {
    let mut grad = vec![0.0; vision.weights_count()];
    for part in grads {
        for (g, p) in grad.iter_mut().zip(part) {
            *g += p;
        }
    }
    grad.iter_mut().for_each(|g| *g /= batch as f32);
    adam.step(vision, &grad);
}

/// One DAgger round of `vision`, in small steps: the net plays, the game copy labels the frames,
/// the net trains on the frames of this and the previous round. Without frames, it first plays
/// the previous round again: the same games and net give the same frames.
pub struct VisionJob {
    vision: Network,
    history: Vec<Network>,
    first_collect_round: u64,
    collect_round: u64,
    round: u64,
    games: u64,
    data: Vec<Sample>,
    old_frames: usize,
    rng: Rng,
    phase: Phase,
}

enum Phase {
    Collect {
        game: u64,
        current: Box<LabeledGame>,
    },
    Fit {
        adam: Adam,
        order: Vec<usize>,
        epoch: usize,
        unshuffled: usize,
        batch_start: usize,
    },
    Done,
}

impl VisionJob {
    /// `rounds` are the nets of the earlier rounds, `data` the frames of the last of them.
    pub fn new(rounds: &[Network], mut data: Vec<Sample>, mut rng: Rng, games: u64) -> Self {
        let vision = rounds
            .last()
            .cloned()
            .unwrap_or_else(|| Network::new(&mut rng, &vision_topology()));
        let round = rounds.len() as u64;
        let collect_round = if data.is_empty() {
            round.saturating_sub(WINDOW_ROUNDS as u64 - 1)
        } else {
            round
        };
        // Growing by doubling would copy all frames inside one step, and hold twice the memory.
        let window = WINDOW_ROUNDS * EXPECTED_NEW_FRAMES;
        data.reserve(window.saturating_sub(data.len()));
        Self {
            vision,
            history: rounds.to_vec(),
            first_collect_round: collect_round,
            collect_round,
            round,
            games,
            old_frames: data.len(),
            data,
            rng,
            phase: Phase::Collect {
                game: 0,
                current: Box::new(LabeledGame::new(vision_seed(collect_round, 0))),
            },
        }
    }

    /// True while the frames of the previous round are played again.
    pub fn is_rebuilding(&self) -> bool {
        self.collect_round < self.round && matches!(self.phase, Phase::Collect { .. })
    }

    pub fn is_done(&self) -> bool {
        matches!(self.phase, Phase::Done)
    }

    /// The trained net, the frames of this round and the random state.
    pub fn finish(mut self) -> (Network, Vec<Sample>, Rng) {
        // In place: a copy would raise the peak of memory, and WASM memory never shrinks.
        self.data.drain(..self.old_frames);
        (self.vision, self.data, self.rng)
    }

    /// The frames of the previous round and the random state.
    pub fn stop(mut self) -> (Vec<Sample>, Rng) {
        self.data.truncate(self.old_frames);
        (self.data, self.rng)
    }

    pub fn progress(&self) -> f32 {
        let epochs = round_epochs(self.round) as f32;
        let collect_rounds = (self.round - self.first_collect_round + 1) as f32;
        let collect_cost = COLLECT_COST * collect_rounds;
        let fit_cost = epochs * window_frames(self.round) as f32 * FIT_FRAME_COST;
        let collect_share = collect_cost / (collect_cost + fit_cost);
        match &self.phase {
            Phase::Collect { game, .. } => {
                let done = (self.collect_round - self.first_collect_round) as f32;
                collect_share * (done + *game as f32 / self.games as f32) / collect_rounds
            }
            Phase::Fit {
                order,
                epoch,
                batch_start,
                ..
            } => {
                let part = *batch_start as f32 / order.len() as f32;
                collect_share + (1.0 - collect_share) * (*epoch as f32 + part) / epochs
            }
            Phase::Done => 1.0,
        }
    }

    pub fn step(&mut self) {
        match &mut self.phase {
            Phase::Collect { game, current } => {
                let player = match self.collect_round {
                    0 => None,
                    r => Some(&self.history[r as usize - 1]),
                };
                self.data.extend(current.step(player));
                if current.is_done() {
                    *game += 1;
                    if *game < self.games {
                        **current = LabeledGame::new(vision_seed(self.collect_round, *game));
                    } else if self.collect_round < self.round {
                        self.collect_round += 1;
                        *game = 0;
                        **current = LabeledGame::new(vision_seed(self.collect_round, 0));
                        if self.collect_round == self.round {
                            self.old_frames = self.data.len();
                        }
                    } else {
                        let order: Vec<usize> = (0..self.data.len()).collect();
                        self.phase = Phase::Fit {
                            adam: new_adam(&self.vision),
                            unshuffled: order.len(),
                            order,
                            epoch: 0,
                            batch_start: 0,
                        };
                    }
                }
            }
            Phase::Fit {
                adam,
                order,
                epoch,
                unshuffled,
                batch_start,
            } => {
                let epochs = round_epochs(self.round);
                if *unshuffled > 1 {
                    for _ in 0..SHUFFLE_PER_STEP.min(*unshuffled - 1) {
                        let i = *unshuffled - 1;
                        order.swap(i, self.rng.below(i + 1));
                        *unshuffled -= 1;
                    }
                    if *unshuffled == 1 {
                        adam.set_learning_rate(epoch_learning_rate(*epoch, epochs));
                    }
                    return;
                }
                let batch_len = BATCH.min(order.len() - *batch_start);
                let batch = &order[*batch_start..*batch_start + batch_len];
                let grad = part_grad(&self.vision, &self.data, batch);
                apply_batch(adam, &mut self.vision, vec![grad], batch_len);
                *batch_start += batch_len;
                if *batch_start == order.len() {
                    *batch_start = 0;
                    *epoch += 1;
                    *unshuffled = order.len();
                    if *epoch == epochs {
                        self.phase = Phase::Done;
                    }
                }
            }
            Phase::Done => {}
        }
    }
}

fn window_frames(round: u64) -> usize {
    (round as usize + 1).min(WINDOW_ROUNDS) * EXPECTED_NEW_FRAMES
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vision_job_trains_and_stop_keeps_old_frames() {
        let mut job = VisionJob::new(&[], Vec::new(), Rng(5), 2);
        let before: Vec<f32> = job.vision.weights().collect();
        let mut progress = 0.0;
        while !job.is_done() {
            job.step();
            assert!(job.progress() >= progress);
            progress = job.progress();
        }
        assert_eq!(progress, 1.0);
        let (vision, data, rng) = job.finish();
        assert_ne!(vision.weights().collect::<Vec<_>>(), before);
        let frames = data.len();
        assert!(frames > 0);

        let first_inputs = data[0].inputs;

        // The frames lost, as after a page reload: the job plays the first round again.
        let rounds = [vision];
        let mut rebuilt = VisionJob::new(&rounds, Vec::new(), rng.clone(), 2);
        while rebuilt.is_rebuilding() {
            rebuilt.step();
        }
        assert_eq!(rebuilt.data.len(), frames);
        assert_eq!(rebuilt.data[0].inputs, first_inputs);

        let mut next = VisionJob::new(&rounds, data, rng, 2);
        assert_eq!(next.progress(), 0.0);
        next.step();
        next.step();
        assert_eq!(next.stop().0.len(), frames);
    }

    #[test]
    fn only_the_last_round_stays_after_a_round() {
        let run = |rounds: &[Network], data, rng| {
            let mut job = VisionJob::new(rounds, data, rng, 2);
            while !job.is_done() {
                job.step();
            }
            job
        };
        let (first, data, rng) = run(&[], Vec::new(), Rng(5)).finish();
        let job = run(std::slice::from_ref(&first), data, rng);
        let new_frames = job.data.len() - job.old_frames;
        let (second, data, rng) = job.finish();
        assert_eq!(data.len(), new_frames);

        // Without frames, only the previous round is played again.
        let rounds = [first, second];
        let mut rebuilt = VisionJob::new(&rounds, Vec::new(), rng, 2);
        assert_eq!(rebuilt.collect_round, 1);
        while rebuilt.is_rebuilding() {
            rebuilt.step();
        }
        assert_eq!(rebuilt.data.len(), new_frames);
        assert_eq!(rebuilt.data[0].inputs, data[0].inputs);
    }
}

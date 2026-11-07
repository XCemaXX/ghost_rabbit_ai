//! A game bot of two nets that sees only the screen:
//! `vision` predicts, for Left / None / Right, the height gain and the risk to die by monster or by fall;
//! `policy` scores each move by its numbers and the previous move; the best score wins.
//!
//! Flow: `vision_job` trains `vision` round by round, `policy_job` trains a `policy` on one round,
//! `to_bytes` keeps the nets, `Bot` plays with a pair.
mod climb;
mod features;
pub mod policy_job;
mod rng;
pub mod train;
pub mod vision_job;

use climb::climb_move;
use features::{INPUTS, features};
use policy_job::POINTS;
pub use rng::Rng;

use game_logic::{GameState, MoveDirection};
pub use neural_network::Network;
use neural_network::{Activation, Buffers, LayerTopology, train::sigmoid};
use rand_trait::GenRandFloat;

const VISION_OUTPUTS: usize = 9;
// Gain, monster risk, fall risk of the move; "it is the previous move"; "it is None".
const POLICY_INPUTS: usize = 5;
// The bot decides every N frames and repeats the move in between.
const DECISION_EVERY: usize = 2;

pub(crate) const MOVES: [MoveDirection; 3] = [
    MoveDirection::Left,
    MoveDirection::None,
    MoveDirection::Right,
];

pub fn vision_topology() -> Vec<LayerTopology> {
    let tanh = |neurons| LayerTopology {
        neurons,
        activation: Activation::Tanh,
    };
    vec![
        tanh(INPUTS),
        tanh(64),
        tanh(64),
        tanh(64),
        LayerTopology {
            neurons: VISION_OUTPUTS,
            activation: Activation::Linear,
        },
    ]
}

// One small net with the same weights for all moves: GA finds a rule for a move once, not three times.
pub fn policy_topology() -> Vec<LayerTopology> {
    let linear = |neurons| LayerTopology {
        neurons,
        activation: Activation::Linear,
    };
    vec![linear(POLICY_INPUTS), linear(1)]
}

pub struct Bot<'a> {
    vision: &'a Network,
    policy: &'a Network,
    step: usize,
    last: MoveDirection,
    vision_buf: Buffers,
    policy_buf: Buffers,
}

impl<'a> Bot<'a> {
    pub fn new(vision: &'a Network, policy: &'a Network) -> Self {
        Self {
            vision,
            policy,
            step: 0,
            last: MoveDirection::None,
            vision_buf: Buffers::default(),
            policy_buf: Buffers::default(),
        }
    }

    pub fn act<R: GenRandFloat>(&mut self, g: &GameState<R>) -> MoveDirection {
        self.step += 1;
        if !(self.step - 1).is_multiple_of(DECISION_EVERY) {
            return self.last;
        }
        let seen = vision_view(
            self.vision
                .propagate_into(&features(g), &mut self.vision_buf),
        );
        let mut best = (f32::NEG_INFINITY, self.last);
        for (i, &m) in MOVES.iter().enumerate() {
            let is = |x: bool| if x { 1.0 } else { 0.0 };
            // The previous move as an input: without it the policy cannot keep a move.
            let inputs = [
                seen[i],
                seen[3 + i],
                seen[6 + i],
                is(m == self.last),
                is(m == MoveDirection::None),
            ];
            let score = self.policy.propagate_into(&inputs, &mut self.policy_buf)[0];
            if score >= best.0 {
                best = (score, m);
            }
        }
        self.last = best.1;
        self.last
    }
}

// Vision outputs as the policy sees them: gains as is, risks as probabilities.
pub(crate) fn vision_view(out: &[f32]) -> [f32; VISION_OUTPUTS] {
    let mut v = [0.0; VISION_OUTPUTS];
    for i in 0..3 {
        v[i] = out[i];
        v[3 + i] = sigmoid(out[3 + i]);
        v[6 + i] = sigmoid(out[6 + i]);
    }
    v
}

/// Weights as little-endian f32, in the order of `Network::weights()`.
pub fn to_bytes(network: &Network) -> Vec<u8> {
    network.weights().flat_map(f32::to_le_bytes).collect()
}

/// `None` if the size does not fit the net.
pub fn vision_from_bytes(bytes: &[u8]) -> Option<Network> {
    from_bytes(&vision_topology(), bytes)
}

/// `None` if the size does not fit the net.
pub fn policy_from_bytes(bytes: &[u8]) -> Option<Network> {
    from_bytes(&policy_topology(), bytes)
}

fn from_bytes(topology: &[LayerTopology], bytes: &[u8]) -> Option<Network> {
    if bytes.len() != size_bytes(topology) {
        return None;
    }
    let weights = bytes
        .chunks_exact(4)
        .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]));
    Some(Network::from_weights(topology, weights))
}

/// A policy file holds the best policy of each of `POINTS`.
pub fn policy_at_generation(checkpoints: &[u8], generation: usize) -> Network {
    let point = POINTS.iter().position(|&p| p == generation).unwrap();
    let size = policy_size_bytes();
    let start = point * size;
    policy_from_bytes(&checkpoints[start..start + size]).unwrap()
}

pub fn policy_size_bytes() -> usize {
    size_bytes(&policy_topology())
}

fn size_bytes(topology: &[LayerTopology]) -> usize {
    let weights: usize = topology
        .windows(2)
        .map(|l| (l[0].neurons + 1) * l[1].neurons)
        .sum();
    4 * weights
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policy_at_generation_reads_its_slot() {
        let n = policy_size_bytes() / 4;
        let mut bytes = Vec::new();
        for g in POINTS {
            let weights: Vec<f32> = (0..n).map(|i| (g * 1000 + i) as f32).collect();
            bytes.extend(to_bytes(&Network::from_weights(
                &policy_topology(),
                weights,
            )));
        }
        let first = policy_at_generation(&bytes, 10).weights().next().unwrap();
        assert_eq!(first, 10_000.0);
    }

    #[test]
    fn the_policy_picks_the_move_with_the_best_score() {
        let vision = Network::new(&mut Rng(1), &vision_topology());
        // Only "it is None" counts.
        let policy = Network::from_weights(&policy_topology(), vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0]);
        let g = GameState::new(Rng(2), game_logic::Difficulty::Unreal);
        assert!(Bot::new(&vision, &policy).act(&g) == MoveDirection::None);
    }
}

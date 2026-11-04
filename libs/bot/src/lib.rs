// A game bot of two nets that sees only the screen:
// `vision` predicts, for Left / None / Right, the height gain and the risk to die by monster or by fall;
// `policy` reads these 9 numbers and its previous move and picks the move.
mod climb;
mod features;
mod rng;
pub mod train;

use climb::climb_move;
pub use features::{INPUTS, features, wrap_dx};
pub use rng::Rng;

use game_logic::{GameState, MoveDirection};
pub use neural_network::Network;
use neural_network::{Activation, Buffers, LayerTopology, train::sigmoid};
use rand_trait::GenRandFloat;

pub const VISION_OUTPUTS: usize = 9;
// A policy file holds the best policy of every GENERATION_STEP-th generation, from 0 to MAX_GENERATION.
pub const GENERATION_STEP: usize = 5;
pub const MAX_GENERATION: usize = 100;
// The bot decides every N frames and repeats the move in between.
pub const DECISION_EVERY: usize = 2;

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

pub fn policy_topology() -> Vec<LayerTopology> {
    let relu = |neurons| LayerTopology {
        neurons,
        activation: Activation::Relu,
    };
    vec![relu(VISION_OUTPUTS + 3), relu(8), relu(3)]
}

pub struct Bot<'a> {
    vision: &'a Network,
    policy: &'a Network,
    step: usize,
    last: MoveDirection,
    inputs: Vec<f32>,
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
            inputs: Vec::new(),
            vision_buf: Buffers::default(),
            policy_buf: Buffers::default(),
        }
    }

    pub fn act<R: GenRandFloat>(&mut self, g: &GameState<R>) -> MoveDirection {
        self.step += 1;
        if !(self.step - 1).is_multiple_of(DECISION_EVERY) {
            return self.last;
        }
        features(g, &mut self.inputs);
        // The previous move as an input: without it the policy cannot keep a move.
        let mut seen = [0.0; VISION_OUTPUTS + 3];
        seen[..VISION_OUTPUTS].copy_from_slice(&vision_view(
            self.vision
                .propagate_into(&self.inputs, &mut self.vision_buf),
        ));
        seen[VISION_OUTPUTS + MOVES.iter().position(|&m| m == self.last).unwrap()] = 1.0;
        let out = self.policy.propagate_into(&seen, &mut self.policy_buf);
        self.last = MOVES[(0..3)
            .max_by(|&a, &b| out[a].partial_cmp(&out[b]).unwrap())
            .unwrap()];
        self.last
    }
}

// Vision outputs as the policy sees them: gains as is, risks as probabilities.
pub fn vision_view(out: &[f32]) -> [f32; VISION_OUTPUTS] {
    let mut v = [0.0; VISION_OUTPUTS];
    for i in 0..3 {
        v[i] = out[i];
        v[3 + i] = sigmoid(out[3 + i]);
        v[6 + i] = sigmoid(out[6 + i]);
    }
    v
}

// Weights as little-endian f32, in the order of `Network::weights()`.
pub fn to_bytes(network: &Network) -> Vec<u8> {
    network.weights().flat_map(f32::to_le_bytes).collect()
}

pub fn from_bytes(topology: &[LayerTopology], bytes: &[u8]) -> Network {
    let weights = bytes
        .chunks_exact(4)
        .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]));
    Network::from_weights(topology, weights)
}

pub fn policy_at_generation(checkpoints: &[u8], generation: usize) -> Network {
    assert!(generation.is_multiple_of(GENERATION_STEP) && generation <= MAX_GENERATION);
    let size = 4 * policy_weights();
    let start = generation / GENERATION_STEP * size;
    from_bytes(&policy_topology(), &checkpoints[start..start + size])
}

fn policy_weights() -> usize {
    policy_topology()
        .windows(2)
        .map(|l| (l[0].neurons + 1) * l[1].neurons)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytes_round_trip() {
        let weights: Vec<f32> = (0..policy_weights())
            .map(|i| i as f32 * 0.25 - 3.0)
            .collect();
        let network = Network::from_weights(&policy_topology(), weights.clone());
        let back = from_bytes(&policy_topology(), &to_bytes(&network));
        assert_eq!(back.weights().collect::<Vec<_>>(), weights);
    }

    #[test]
    fn policy_at_generation_reads_its_slot() {
        let n = policy_weights();
        let mut bytes = Vec::new();
        for g in (0..=MAX_GENERATION).step_by(GENERATION_STEP) {
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
    fn wrap_dx_takes_short_way() {
        let w = game_logic::SCREEN_WEIGHT;
        assert!((wrap_dx(0.9 * w) + 0.1 * w).abs() < 1e-6);
        assert!((wrap_dx(-0.9 * w) - 0.1 * w).abs() < 1e-6);
    }
}

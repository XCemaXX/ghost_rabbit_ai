use std::collections::HashMap;

use bot::Network;

const VISIONS: [&[u8]; 8] = [
    include_bytes!("../../../resources/bot/vision_r0.bin"),
    include_bytes!("../../../resources/bot/vision_r1.bin"),
    include_bytes!("../../../resources/bot/vision_r2.bin"),
    include_bytes!("../../../resources/bot/vision_r3.bin"),
    include_bytes!("../../../resources/bot/vision_r4.bin"),
    include_bytes!("../../../resources/bot/vision_r5.bin"),
    include_bytes!("../../../resources/bot/vision_r6.bin"),
    include_bytes!("../../../resources/bot/vision_r7.bin"),
];
const POLICIES: [&[u8]; 8] = [
    include_bytes!("../../../resources/bot/policy_r0.bin"),
    include_bytes!("../../../resources/bot/policy_r1.bin"),
    include_bytes!("../../../resources/bot/policy_r2.bin"),
    include_bytes!("../../../resources/bot/policy_r3.bin"),
    include_bytes!("../../../resources/bot/policy_r4.bin"),
    include_bytes!("../../../resources/bot/policy_r5.bin"),
    include_bytes!("../../../resources/bot/policy_r6.bin"),
    include_bytes!("../../../resources/bot/policy_r7.bin"),
];
const SCORES: &str = include_str!("../../../resources/bot/scores.csv");

pub const ROUNDS: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
// Dense at the start, where the score grows fast: neighbour points later differ only by noise.
pub const GENERATIONS: [usize; 7] = [0, 5, 10, 20, 40, 70, 100];

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Selection {
    pub round: usize,
    pub generation: usize,
}

pub fn load(s: Selection) -> (Network, Network) {
    let vision = bot::from_bytes(&bot::vision_topology(), VISIONS[s.round]);
    let policy = bot::policy_at_generation(POLICIES[s.round], s.generation);
    (vision, policy)
}

pub struct Scores(HashMap<Selection, u32>);

impl Scores {
    pub fn new() -> Self {
        let parse = |line: &str| {
            let mut fields = line.split(',').map(|f| f.parse().unwrap());
            let mut next = || fields.next().unwrap();
            let (round, generation) = (next() as usize, next() as usize);
            (Selection { round, generation }, next())
        };
        Self(SCORES.lines().skip(1).map(parse).collect())
    }

    pub fn get(&self, s: Selection) -> u32 {
        self.0[&s]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_selection_has_nets_and_score() {
        let scores = Scores::new();
        for round in ROUNDS {
            for generation in GENERATIONS {
                let s = Selection { round, generation };
                load(s);
                scores.get(s);
            }
        }
    }
}

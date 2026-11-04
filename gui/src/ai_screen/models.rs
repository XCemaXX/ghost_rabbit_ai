use std::collections::HashMap;

use bot::job::{POINTS, Point, PolicyGa, PolicyJob};
use bot::{Network, Rng};
use macroquad::miniquad;

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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    pub round: usize,
    pub generation: usize,
    pub my_policy: bool,
}

/// Policies trained by the player on one `vision` round.
struct MyPolicy {
    points: Vec<Point>,
    inherited: Option<(usize, usize)>,
}

pub struct Models {
    scores: HashMap<(usize, usize), u32>,
    my_policies: HashMap<usize, MyPolicy>,
}

impl Models {
    pub fn new() -> Self {
        let parse = |line: &str| {
            let mut fields = line.split(',').map(|f| f.parse().unwrap());
            let mut next = || fields.next().unwrap();
            let key = (next() as usize, next() as usize);
            (key, next())
        };
        Self {
            scores: SCORES.lines().skip(1).map(parse).collect(),
            my_policies: HashMap::new(),
        }
    }

    pub fn load(&self, s: Selection) -> (Network, Network) {
        let policy = if s.my_policy {
            self.my_point(s).map_or_else(
                || Network::new(&mut Rng(s.round as u64), &bot::policy_topology()),
                |point| point.policy.clone(),
            )
        } else {
            bot::policy_at_generation(POLICIES[s.round], s.generation)
        };
        (load_vision(s.round), policy)
    }

    /// `None` for an untrained policy of the player.
    pub fn score(&self, s: Selection) -> Option<u32> {
        if s.my_policy {
            self.my_point(s).map(|point| point.score as u32)
        } else {
            Some(self.scores[&(s.round, s.generation)])
        }
    }

    /// Generations of the player's policy on this round; empty if there is none.
    pub fn my_generations(&self, round: usize) -> Vec<usize> {
        self.my_policies.get(&round).map_or(Vec::new(), |p| {
            p.points.iter().map(|point| point.generation).collect()
        })
    }

    /// The round and generation the player's policy started from.
    pub fn inherited(&self, round: usize) -> Option<(usize, usize)> {
        self.my_policies.get(&round)?.inherited
    }

    /// The next point to train on this round; `None` if the last one is reached.
    pub fn next_point(&self, round: usize) -> Option<usize> {
        let last = self.my_generations(round).last().copied();
        match last {
            Some(last) => POINTS.into_iter().find(|&p| p > last),
            None => Some(POINTS[1]),
        }
    }

    /// Goes on from the last point of the round, or else from the nearest earlier round.
    pub fn start_job(&mut self, round: usize) -> PolicyJob {
        let target = self.next_point(round).expect("the last point is reached");
        let ga = if let Some(last) = self.my_policies.get(&round).and_then(|p| p.points.last()) {
            PolicyGa::from_elite(last.rng.clone(), &last.elite, last.generation + 1)
        } else {
            let rng = Rng(miniquad::date::now().to_bits() ^ round as u64);
            let earlier = (0..round).rev().find_map(|r| {
                let last = self.my_policies.get(&r)?.points.last()?;
                Some((r, last))
            });
            match earlier {
                Some((r, last)) => {
                    let inherited = Some((r, last.generation));
                    let ga = PolicyGa::from_elite(rng, &last.elite, 0);
                    self.my_policies.insert(
                        round,
                        MyPolicy {
                            points: Vec::new(),
                            inherited,
                        },
                    );
                    ga
                }
                None => PolicyGa::new(rng),
            }
        };
        PolicyJob::new(load_vision(round), ga, target)
    }

    pub fn add_point(&mut self, round: usize, point: Point) {
        self.my_policies
            .entry(round)
            .or_insert_with(|| MyPolicy {
                points: Vec::new(),
                inherited: None,
            })
            .points
            .push(point);
    }

    fn my_point(&self, s: Selection) -> Option<&Point> {
        self.my_policies
            .get(&s.round)?
            .points
            .iter()
            .find(|p| p.generation == s.generation)
    }
}

fn load_vision(round: usize) -> Network {
    bot::from_bytes(&bot::vision_topology(), VISIONS[round])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_pretrained_selection_has_nets_and_score() {
        let models = Models::new();
        for round in ROUNDS {
            for generation in POINTS {
                let s = Selection {
                    round,
                    generation,
                    my_policy: false,
                };
                models.load(s);
                models.score(s);
            }
        }
    }

    fn point(generation: usize) -> Point {
        let mut rng = Rng(generation as u64);
        let policy = Network::new(&mut rng, &bot::policy_topology());
        Point {
            generation,
            policy: policy.clone(),
            score: 1.0,
            elite: vec![policy],
            rng,
        }
    }

    #[test]
    fn next_point_follows_the_last_finished_one() {
        let mut models = Models::new();
        assert_eq!(models.next_point(2), Some(POINTS[1]));
        models.add_point(2, point(POINTS[0]));
        models.add_point(2, point(POINTS[1]));
        assert_eq!(models.next_point(2), Some(POINTS[2]));
        models.add_point(2, point(*POINTS.last().unwrap()));
        assert_eq!(models.next_point(2), None);
    }

    #[test]
    fn a_new_round_inherits_from_the_nearest_earlier_one() {
        let mut models = Models::new();
        models.add_point(1, point(POINTS[1]));
        models.add_point(3, point(POINTS[2]));
        models.start_job(5);
        assert_eq!(models.inherited(5), Some((3, POINTS[2])));
        models.start_job(0);
        assert_eq!(models.inherited(0), None);
    }
}

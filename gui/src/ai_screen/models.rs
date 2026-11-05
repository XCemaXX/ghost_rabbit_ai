use std::collections::HashMap;

use bot::policy_job::{POINTS, Point, PolicyGa, PolicyJob};
use bot::train::Sample;
use bot::vision_job::{VISION_GAMES, VISION_ROUNDS, VisionJob};
use bot::{Network, Rng};
use macroquad::miniquad;

const VISIONS: [&[u8]; VISION_ROUNDS] = [
    include_bytes!("../../../resources/bot/vision_r0.bin"),
    include_bytes!("../../../resources/bot/vision_r1.bin"),
    include_bytes!("../../../resources/bot/vision_r2.bin"),
    include_bytes!("../../../resources/bot/vision_r3.bin"),
    include_bytes!("../../../resources/bot/vision_r4.bin"),
    include_bytes!("../../../resources/bot/vision_r5.bin"),
    include_bytes!("../../../resources/bot/vision_r6.bin"),
    include_bytes!("../../../resources/bot/vision_r7.bin"),
];
const POLICIES: [&[u8]; VISION_ROUNDS] = [
    include_bytes!("../../../resources/bot/policy_r0.bin"),
    include_bytes!("../../../resources/bot/policy_r1.bin"),
    include_bytes!("../../../resources/bot/policy_r2.bin"),
    include_bytes!("../../../resources/bot/policy_r3.bin"),
    include_bytes!("../../../resources/bot/policy_r4.bin"),
    include_bytes!("../../../resources/bot/policy_r5.bin"),
    include_bytes!("../../../resources/bot/policy_r6.bin"),
    include_bytes!("../../../resources/bot/policy_r7.bin"),
];
const LIVES: &str = include_str!("../../../resources/bot/scores.csv");

// Seed of the nets shown before any training.
const UNTRAINED_SEED: u64 = 1;

#[derive(Clone, Copy)]
pub struct Selection {
    pub round: usize,
    pub generation: usize,
    pub my_vision: bool,
    pub my_policy: bool,
}

/// A policy belongs to the `vision` it trained on: my `vision` or not, and its round.
pub type PolicyKey = (bool, usize);

impl Selection {
    pub fn policy_key(&self) -> PolicyKey {
        (self.my_vision, self.round)
    }
}

/// Policies trained by the player on one `vision`.
#[derive(Default)]
struct MyPolicy {
    points: Vec<Point>,
    inherited: Option<(usize, usize)>,
}

/// The rounds of the player's `vision`, and the frames for the next round.
struct MyVision {
    rounds: Vec<Network>,
    data: Vec<Sample>,
    rng: Rng,
}

pub struct Models {
    lives: HashMap<(usize, usize), u32>,
    my_policies: HashMap<PolicyKey, MyPolicy>,
    my_vision: MyVision,
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
            lives: LIVES.lines().skip(1).map(parse).collect(),
            my_policies: HashMap::new(),
            my_vision: MyVision {
                rounds: Vec::new(),
                data: Vec::new(),
                rng: random_rng(),
            },
        }
    }

    pub fn load(&self, s: Selection) -> (Network, Network) {
        let policy = if s.my_policy {
            self.my_point(s)
                .map_or_else(untrained_policy, |p| p.policy.clone())
        } else {
            bot::policy_at_generation(POLICIES[s.round], s.generation)
        };
        (self.load_vision(s.my_vision, s.round), policy)
    }

    fn load_vision(&self, my_vision: bool, round: usize) -> Network {
        if my_vision {
            self.my_vision.rounds.get(round).map_or_else(
                || Network::new(&mut Rng(UNTRAINED_SEED), &bot::vision_topology()),
                Network::clone,
            )
        } else {
            bot::from_bytes(&bot::vision_topology(), VISIONS[round])
        }
    }

    /// Median seconds of life; `None` for an untrained policy of the player.
    pub fn life(&self, s: Selection) -> Option<u32> {
        if s.my_policy {
            self.my_point(s).map(|point| point.life.round() as u32)
        } else {
            Some(self.lives[&(s.round, s.generation)])
        }
    }

    pub fn my_vision_rounds(&self) -> usize {
        self.my_vision.rounds.len()
    }

    /// The next round of the player's `vision`; `None` after the last one.
    pub fn next_vision_round(&self) -> Option<usize> {
        let next = self.my_vision_rounds();
        (next < VISION_ROUNDS).then_some(next)
    }

    pub fn start_vision_job(&mut self) -> VisionJob {
        let vision = &mut self.my_vision;
        VisionJob::new(
            vision.rounds.last().cloned(),
            std::mem::take(&mut vision.data),
            vision.rng.clone(),
            vision.rounds.len() as u64,
            VISION_GAMES,
        )
    }

    pub fn finish_vision_job(&mut self, job: VisionJob) {
        let (net, data, rng) = job.finish();
        self.my_vision.rounds.push(net);
        self.my_vision.data = data;
        self.my_vision.rng = rng;
    }

    pub fn stop_vision_job(&mut self, job: VisionJob) {
        (self.my_vision.data, self.my_vision.rng) = job.stop();
    }

    pub fn my_generations(&self, key: PolicyKey) -> Vec<usize> {
        self.my_policies.get(&key).map_or(Vec::new(), |p| {
            p.points.iter().map(|point| point.generation).collect()
        })
    }

    pub fn inherited(&self, key: PolicyKey) -> Option<(usize, usize)> {
        self.my_policies.get(&key)?.inherited
    }

    /// The next point to train; `None` if the last one is reached.
    pub fn next_point(&self, key: PolicyKey) -> Option<usize> {
        let last = self.my_generations(key).last().copied();
        match last {
            Some(last) => POINTS.into_iter().find(|&p| p > last),
            None => Some(POINTS[1]),
        }
    }

    /// Goes on from the last point, or else from the nearest earlier round of the same `vision`.
    pub fn start_policy_job(&mut self, key: PolicyKey) -> PolicyJob {
        let target = self.next_point(key).expect("the last point is reached");
        let (my_vision, round) = key;
        let ga = if let Some(last) = self.my_policies.get(&key).and_then(|p| p.points.last()) {
            PolicyGa::from_elite(last.rng.clone(), &last.elite, last.generation + 1)
        } else {
            let earlier = (0..round).rev().find_map(|r| {
                let last = self.my_policies.get(&(my_vision, r))?.points.last()?;
                Some((r, last))
            });
            match earlier {
                Some((r, last)) => {
                    let inherited = Some((r, last.generation));
                    let ga = PolicyGa::from_elite(random_rng(), &last.elite, 0);
                    self.my_policies.insert(
                        key,
                        MyPolicy {
                            inherited,
                            ..Default::default()
                        },
                    );
                    ga
                }
                None => PolicyGa::new(random_rng()),
            }
        };
        PolicyJob::new(self.load_vision(my_vision, round), ga, target)
    }

    pub fn add_point(&mut self, key: PolicyKey, point: Point) {
        self.my_policies.entry(key).or_default().points.push(point);
    }

    fn my_point(&self, s: Selection) -> Option<&Point> {
        self.my_policies
            .get(&s.policy_key())?
            .points
            .iter()
            .find(|p| p.generation == s.generation)
    }
}

fn untrained_policy() -> Network {
    Network::new(&mut Rng(UNTRAINED_SEED), &bot::policy_topology())
}

fn random_rng() -> Rng {
    Rng(miniquad::date::now().to_bits())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point(generation: usize) -> Point {
        let mut rng = Rng(generation as u64);
        let policy = Network::new(&mut rng, &bot::policy_topology());
        Point {
            generation,
            policy: policy.clone(),
            life: 1.0,
            elite: vec![policy],
            rng,
        }
    }

    #[test]
    fn every_pretrained_selection_has_nets_and_score() {
        let models = Models::new();
        for round in 0..VISION_ROUNDS {
            for generation in POINTS {
                let s = Selection {
                    round,
                    generation,
                    my_vision: false,
                    my_policy: false,
                };
                models.load(s);
                models.life(s);
            }
        }
    }

    #[test]
    fn next_point_follows_the_last_finished_one() {
        let mut models = Models::new();
        let key = (false, 2);
        assert_eq!(models.next_point(key), Some(POINTS[1]));
        models.add_point(key, point(POINTS[0]));
        models.add_point(key, point(POINTS[1]));
        assert_eq!(models.next_point(key), Some(POINTS[2]));
        models.add_point(key, point(*POINTS.last().unwrap()));
        assert_eq!(models.next_point(key), None);
    }

    #[test]
    fn a_new_round_inherits_only_from_the_same_vision() {
        let mut models = Models::new();
        models.add_point((false, 1), point(POINTS[1]));
        models.add_point((false, 3), point(POINTS[2]));
        models.start_policy_job((false, 5));
        assert_eq!(models.inherited((false, 5)), Some((3, POINTS[2])));
        models.start_policy_job((false, 0));
        assert_eq!(models.inherited((false, 0)), None);
        models.start_policy_job((true, 5));
        assert_eq!(models.inherited((true, 5)), None);
    }

    #[test]
    fn untrained_my_models_load() {
        let s = Selection {
            round: 0,
            generation: 0,
            my_vision: true,
            my_policy: true,
        };
        let models = Models::new();
        models.load(s);
        assert_eq!(models.life(s), None);
    }
}

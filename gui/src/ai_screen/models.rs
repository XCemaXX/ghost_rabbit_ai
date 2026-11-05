use std::collections::HashMap;

use super::storage;
use bot::policy_job::{POINTS, Point, PolicyGa, PolicyJob};
use bot::train::{Sample, TestResult};
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
const RESULTS: &str = include_str!("../../../resources/bot/scores.csv");

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

/// A finished point of the player's policy.
pub struct MyPoint {
    pub generation: usize,
    pub policy: Network,
    pub result: TestResult,
}

/// Policies trained by the player on one `vision`.
/// Training goes on from `elite` and `rng`, the state at the last point.
pub struct MyPolicy {
    pub points: Vec<MyPoint>,
    pub elite: Vec<Network>,
    pub rng: Rng,
    pub inherited: Option<(usize, usize)>,
}

/// The rounds of the player's `vision`, and the frames for the next round.
struct MyVision {
    rounds: Vec<Network>,
    data: Vec<Sample>,
    rng: Rng,
}

pub struct Models {
    results: HashMap<(usize, usize), TestResult>,
    my_policies: HashMap<PolicyKey, MyPolicy>,
    my_vision: MyVision,
    pub save_failed: bool,
}

impl Models {
    pub fn new() -> Self {
        let parse = |line: &str| {
            let mut fields = line.split(',').map(|f| f.parse::<f32>().unwrap());
            let mut next = || fields.next().unwrap();
            let key = (next() as usize, next() as usize);
            let (life, score) = (next(), next());
            (key, TestResult { life, score })
        };
        Self {
            results: RESULTS.lines().skip(1).map(parse).collect(),
            my_policies: HashMap::new(),
            my_vision: MyVision {
                rounds: Vec::new(),
                data: Vec::new(),
                rng: random_rng(),
            },
            save_failed: false,
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

    /// Medians of the test games; `None` for an untrained policy of the player.
    pub fn result(&self, s: Selection) -> Option<TestResult> {
        if s.my_policy {
            self.my_point(s).map(|point| point.result)
        } else {
            Some(self.results[&(s.round, s.generation)])
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
            &vision.rounds,
            std::mem::take(&mut vision.data),
            vision.rng.clone(),
            VISION_GAMES,
        )
    }

    pub fn finish_vision_job(&mut self, job: VisionJob) {
        let (net, data, rng) = job.finish();
        let round = self.my_vision.rounds.len();
        self.save_failed = !storage::save_vision_round(round, &net, &rng);
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
        let last_generation = |p: &MyPolicy| p.points.last().map(|point| point.generation);
        let ga = if let Some((policy, generation)) = self
            .my_policies
            .get(&key)
            .and_then(|p| Some((p, last_generation(p)?)))
        {
            PolicyGa::from_elite(policy.rng.clone(), &policy.elite, generation + 1)
        } else {
            let earlier = (0..round).rev().find_map(|r| {
                let policy = self.my_policies.get(&(my_vision, r))?;
                Some((r, last_generation(policy)?, policy))
            });
            match earlier {
                Some((r, generation, policy)) => {
                    let rng = random_rng();
                    let ga = PolicyGa::from_elite(rng.clone(), &policy.elite, 0);
                    self.my_policies.insert(
                        key,
                        MyPolicy {
                            points: Vec::new(),
                            elite: Vec::new(),
                            rng,
                            inherited: Some((r, generation)),
                        },
                    );
                    ga
                }
                None => PolicyGa::new(random_rng()),
            }
        };
        PolicyJob::new(self.load_vision(my_vision, round), ga, target)
    }

    /// Returns the policy with the new point, to be saved.
    pub fn add_point(&mut self, key: PolicyKey, point: Point) -> &MyPolicy {
        let policy = self.my_policies.entry(key).or_insert_with(|| MyPolicy {
            points: Vec::new(),
            elite: Vec::new(),
            rng: point.rng.clone(),
            inherited: None,
        });
        policy.points.push(MyPoint {
            generation: point.generation,
            policy: point.policy,
            result: point.result,
        });
        policy.elite = point.elite;
        policy.rng = point.rng;
        policy
    }

    pub fn has_points(&self, key: PolicyKey) -> bool {
        !self.my_generations(key).is_empty()
    }

    pub fn reset_policy(&mut self, key: PolicyKey) {
        self.my_policies.remove(&key);
    }

    /// Removes the player's `vision` and the policies trained on it.
    pub fn reset_vision(&mut self) {
        self.my_vision.rounds.clear();
        self.my_vision.data = Vec::new();
        self.my_vision.rng = random_rng();
        self.my_policies.retain(|&(my_vision, _), _| !my_vision);
    }

    /// Adds what the browser keeps from earlier visits.
    pub fn with_saved() -> Self {
        let mut models = Self::new();
        let (rounds, rng) = storage::load_vision();
        models.my_vision.rounds = rounds;
        if let Some(rng) = rng {
            models.my_vision.rng = rng;
        }
        for round in 0..VISION_ROUNDS {
            for my_vision in [false, true] {
                let key = (my_vision, round);
                if let Some(policy) = storage::load_policy(key) {
                    models.my_policies.insert(key, policy);
                }
            }
        }
        models
    }

    fn my_point(&self, s: Selection) -> Option<&MyPoint> {
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
            result: TestResult {
                life: 1.0,
                score: 2.0,
            },
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
                models.result(s);
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
        models.reset_policy((false, 3));
        assert!(!models.has_points((false, 3)));
    }

    #[test]
    fn reset_vision_removes_only_its_policies() {
        let mut models = Models::new();
        models.add_point((true, 1), point(POINTS[1]));
        models.add_point((false, 1), point(POINTS[1]));
        models.reset_vision();
        assert!(!models.has_points((true, 1)));
        assert!(models.has_points((false, 1)));
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
        assert!(models.result(s).is_none());
    }
}

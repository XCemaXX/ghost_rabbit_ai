use std::collections::HashMap;

use super::my_policy::MyPolicy;
use super::storage::{self, PolicyKey};
use bot::policy_job::{POINTS, Point, PolicyGa, PolicyJob, Resume};
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

impl Selection {
    pub fn policy_key(&self) -> PolicyKey {
        PolicyKey {
            my_vision: self.my_vision,
            round: self.round,
        }
    }
}

/// The rounds of the player's `vision`, and the frames of the last round.
struct MyVision {
    rounds: Vec<Network>,
    data: Vec<Sample>,
    rng: Rng,
}

pub struct Models {
    results: HashMap<(usize, usize), TestResult>,
    my_policies: HashMap<PolicyKey, MyPolicy>,
    my_vision: MyVision,
    save_failed: bool,
}

impl Models {
    fn new() -> Self {
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

    /// Adds what the browser keeps from earlier visits.
    pub fn from_storage() -> Self {
        let mut models = Self::new();
        let (rounds, rng) = storage::load_vision();
        models.my_vision.rounds = rounds;
        if let Some(rng) = rng {
            models.my_vision.rng = rng;
        }
        for round in 0..VISION_ROUNDS {
            for my_vision in [false, true] {
                let key = PolicyKey { my_vision, round };
                // Its `vision` is lost: a new one must not get this policy and its results.
                if my_vision && round >= models.my_vision_rounds() {
                    storage::remove_policy(key);
                } else if let Some(policy) =
                    storage::load_policy(key).and_then(|text| MyPolicy::from_text(&text))
                {
                    models.my_policies.insert(key, policy);
                }
            }
        }
        models
    }

    /// Some save did not reach the browser since the last reset of `vision`.
    pub fn save_failed(&self) -> bool {
        self.save_failed
    }

    /// The pair of nets to play with.
    pub fn nets(&self, s: Selection) -> (Network, Network) {
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
            bot::vision_from_bytes(VISIONS[round]).unwrap()
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

    /// The rounds to choose from. An untrained net of the player is round 0.
    pub fn rounds(&self, my_vision: bool) -> Vec<usize> {
        if my_vision {
            (0..self.my_vision_rounds().max(1)).collect()
        } else {
            (0..VISION_ROUNDS).collect()
        }
    }

    /// The generations to choose from. An untrained policy of the player is generation 0.
    pub fn generations(&self, key: PolicyKey, my_policy: bool) -> Vec<usize> {
        if !my_policy {
            return POINTS.to_vec();
        }
        let generations = self.my_generations(key);
        if generations.is_empty() {
            vec![0]
        } else {
            generations
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
        assert!(self.next_vision_round().is_some());
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
        self.save_failed |= !storage::save_vision_round(round, &net, &rng);
        self.my_vision.rounds.push(net);
        self.my_vision.data = data;
        self.my_vision.rng = rng;
    }

    pub fn stop_vision_job(&mut self, job: VisionJob) {
        (self.my_vision.data, self.my_vision.rng) = job.stop();
    }

    fn my_generations(&self, key: PolicyKey) -> Vec<usize> {
        self.my_policies.get(&key).map_or(Vec::new(), |p| {
            p.points.iter().map(|point| point.generation).collect()
        })
    }

    /// The next point to train; `None` if the last one is reached.
    pub fn next_point(&self, key: PolicyKey) -> Option<usize> {
        let last = self.my_generations(key).last().copied();
        match last {
            Some(last) => POINTS.into_iter().find(|&p| p > last),
            None => Some(POINTS[1]),
        }
    }

    /// A policy on the player's `vision` needs that round.
    pub fn can_train_policy(&self, key: PolicyKey) -> bool {
        !key.my_vision || key.round < self.my_vision_rounds()
    }

    /// Goes on from the last point, or else starts from a random population.
    pub fn start_policy_job(&mut self, key: PolicyKey) -> PolicyJob {
        let target = self.next_point(key).expect("the last point is reached");
        assert!(self.can_train_policy(key));
        let ga = match self.my_policies.get(&key) {
            Some(policy) => PolicyGa::resume(policy.resume.clone()),
            None => PolicyGa::new(random_rng()),
        };
        PolicyJob::new(self.load_vision(key.my_vision, key.round), ga, target)
    }

    pub fn add_point(&mut self, key: PolicyKey, point: Point, resume: Resume) {
        let policy = self.push_point(key, point, resume);
        let text = policy.to_text();
        self.save_failed |= !storage::save_policy(key, &text);
    }

    fn push_point(&mut self, key: PolicyKey, point: Point, resume: Resume) -> &MyPolicy {
        let policy = self.my_policies.entry(key).or_insert_with(|| MyPolicy {
            points: Vec::new(),
            resume: resume.clone(),
        });
        policy.points.push(point);
        policy.resume = resume;
        policy
    }

    pub fn has_points(&self, key: PolicyKey) -> bool {
        !self.my_generations(key).is_empty()
    }

    pub fn reset_policy(&mut self, key: PolicyKey) {
        self.my_policies.remove(&key);
        storage::remove_policy(key);
    }

    /// Removes the player's `vision` and the policies trained on it.
    pub fn reset_vision(&mut self) {
        self.forget_my_vision();
        storage::remove_vision();
        for round in 0..VISION_ROUNDS {
            storage::remove_policy(PolicyKey {
                my_vision: true,
                round,
            });
        }
        self.save_failed = false;
    }

    fn forget_my_vision(&mut self) {
        self.my_vision.rounds.clear();
        self.my_vision.data = Vec::new();
        self.my_vision.rng = random_rng();
        self.my_policies.retain(|key, _| !key.my_vision);
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

// Native storage writes a file, so these tests stay on the methods without it.
#[cfg(test)]
mod tests {
    use super::*;

    fn point(generation: usize) -> (Point, Resume) {
        let mut rng = Rng(generation as u64);
        let policy = Network::new(&mut rng, &bot::policy_topology());
        let resume = Resume {
            generation: generation + 1,
            population: vec![policy.clone()],
            rng,
        };
        let result = TestResult {
            life: 1.0,
            score: 2.0,
        };
        let point = Point {
            generation,
            policy,
            result,
        };
        (point, resume)
    }

    fn key(my_vision: bool, round: usize) -> PolicyKey {
        PolicyKey { my_vision, round }
    }

    fn push(models: &mut Models, key: PolicyKey, generation: usize) {
        let (point, resume) = point(generation);
        models.push_point(key, point, resume);
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
                models.nets(s);
                models.result(s);
            }
        }
    }

    #[test]
    fn next_point_follows_the_last_finished_one() {
        let mut models = Models::new();
        let key = key(false, 2);
        assert_eq!(models.next_point(key), Some(POINTS[1]));
        push(&mut models, key, POINTS[0]);
        push(&mut models, key, POINTS[1]);
        assert_eq!(models.next_point(key), Some(POINTS[2]));
        push(&mut models, key, *POINTS.last().unwrap());
        assert_eq!(models.next_point(key), None);
    }

    #[test]
    fn forgetting_my_vision_removes_only_its_policies() {
        let mut models = Models::new();
        push(&mut models, key(true, 1), POINTS[1]);
        push(&mut models, key(false, 1), POINTS[1]);
        models.forget_my_vision();
        assert!(!models.has_points(key(true, 1)));
        assert!(models.has_points(key(false, 1)));
    }
}

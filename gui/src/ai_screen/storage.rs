use base64::{Engine, engine::general_purpose::STANDARD};
use bot::train::TestResult;
use bot::vision_job::VISION_ROUNDS;
use bot::{Network, Rng};

use super::models::{MyPoint, MyPolicy, PolicyKey};

// The version goes up when the format or the nets change: old data is then ignored.
fn policy_key((my_vision, round): PolicyKey) -> String {
    let vision = if my_vision { "my" } else { "pretrained" };
    format!("bot_policy_v5_{vision}_r{round}")
}

fn vision_key(round: usize) -> String {
    format!("bot_vision_v3_r{round}")
}

const VISION_RNG_KEY: &str = "bot_vision_v3_rng";

pub fn load_policy(key: PolicyKey) -> Option<MyPolicy> {
    let storage = quad_storage::STORAGE.lock().unwrap();
    decode(&storage.get(&policy_key(key))?)
}

/// Returns false if the browser did not keep the data.
pub fn save_policy(key: PolicyKey, policy: &MyPolicy) -> bool {
    let value = encode(policy);
    set_checked(&policy_key(key), &value)
}

pub fn remove_policy(key: PolicyKey) {
    quad_storage::STORAGE
        .lock()
        .unwrap()
        .remove(&policy_key(key));
}

/// The saved rounds in order, up to the first missing or broken one, and the random state.
pub fn load_vision() -> (Vec<Network>, Option<Rng>) {
    let storage = quad_storage::STORAGE.lock().unwrap();
    let mut rounds = Vec::new();
    while let Some(net) = storage
        .get(&vision_key(rounds.len()))
        .and_then(|text| from_base64(&text))
        .filter(|bytes| bytes.len() == bot::vision_weights() * 4)
    {
        rounds.push(bot::from_bytes(&bot::vision_topology(), &net));
    }
    let rng = storage
        .get(VISION_RNG_KEY)
        .and_then(|text| text.parse().ok())
        .map(Rng);
    (rounds, rng)
}

/// Returns false if the browser did not keep the data.
pub fn save_vision_round(round: usize, net: &Network, rng: &Rng) -> bool {
    let value = to_base64(&bot::to_bytes(net));
    set_checked(&vision_key(round), &value) && set_checked(VISION_RNG_KEY, &rng.0.to_string())
}

pub fn remove_vision() {
    let mut storage = quad_storage::STORAGE.lock().unwrap();
    for round in 0..VISION_ROUNDS {
        storage.remove(&vision_key(round));
    }
    storage.remove(VISION_RNG_KEY);
}

fn set_checked(key: &str, value: &str) -> bool {
    let mut storage = quad_storage::STORAGE.lock().unwrap();
    storage.set(key, value);
    storage.get(key).as_deref() == Some(value)
}

fn encode(policy: &MyPolicy) -> String {
    let mut bytes = Vec::new();
    bytes.extend(policy.rng.0.to_le_bytes());
    bytes.extend((policy.points.len() as u32).to_le_bytes());
    for point in &policy.points {
        bytes.extend((point.generation as u32).to_le_bytes());
        bytes.extend(point.result.life.to_le_bytes());
        bytes.extend(point.result.score.to_le_bytes());
        bytes.extend(bot::to_bytes(&point.policy));
    }
    bytes.extend((policy.elite.len() as u32).to_le_bytes());
    for net in &policy.elite {
        bytes.extend(bot::to_bytes(net));
    }
    to_base64(&bytes)
}

fn decode(text: &str) -> Option<MyPolicy> {
    let bytes = from_base64(text)?;
    let mut r = Reader(&bytes);
    let rng = Rng(u64::from_le_bytes(r.take(8)?.try_into().ok()?));
    let mut points = Vec::new();
    for _ in 0..r.i32()? {
        points.push(MyPoint {
            generation: r.i32()? as usize,
            result: TestResult {
                life: r.f32()?,
                score: r.f32()?,
            },
            policy: r.net()?,
        });
    }
    let mut elite = Vec::new();
    for _ in 0..r.i32()? {
        elite.push(r.net()?);
    }
    r.0.is_empty().then_some(MyPolicy { points, elite, rng })
}

struct Reader<'a>(&'a [u8]);

impl<'a> Reader<'a> {
    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        if self.0.len() < n {
            return None;
        }
        let (head, tail) = self.0.split_at(n);
        self.0 = tail;
        Some(head)
    }

    fn i32(&mut self) -> Option<i32> {
        Some(i32::from_le_bytes(self.take(4)?.try_into().ok()?))
    }

    fn f32(&mut self) -> Option<f32> {
        Some(f32::from_le_bytes(self.take(4)?.try_into().ok()?))
    }

    fn net(&mut self) -> Option<Network> {
        let bytes = self.take(bot::policy_weights() * 4)?;
        Some(bot::from_bytes(&bot::policy_topology(), bytes))
    }
}

fn to_base64(bytes: &[u8]) -> String {
    STANDARD.encode(bytes)
}

fn from_base64(text: &str) -> Option<Vec<u8>> {
    STANDARD.decode(text).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn policy() -> MyPolicy {
        let net = |seed| Network::new(&mut Rng(seed), &bot::policy_topology());
        MyPolicy {
            points: vec![MyPoint {
                generation: 5,
                result: TestResult {
                    life: 123.5,
                    score: 4567.0,
                },
                policy: net(1),
            }],
            elite: vec![net(2), net(3)],
            rng: Rng(77),
        }
    }

    #[test]
    fn policy_round_trip() {
        let back = decode(&encode(&policy())).unwrap();
        assert_eq!(back.rng.0, 77);
        assert_eq!(back.points[0].generation, 5);
        assert_eq!(back.points[0].result.life, 123.5);
        assert_eq!(back.points[0].result.score, 4567.0);
        assert_eq!(back.elite.len(), 2);
        assert_eq!(
            back.elite[1].weights().collect::<Vec<_>>(),
            policy().elite[1].weights().collect::<Vec<_>>()
        );
    }

    #[test]
    fn broken_data_is_ignored() {
        let text = encode(&policy());
        assert!(decode(&text[..text.len() - 4]).is_none());
        assert!(decode("not base64!").is_none());
    }
}

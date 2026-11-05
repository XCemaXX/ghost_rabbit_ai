use base64::{Engine, engine::general_purpose::STANDARD};
use bot::vision_job::VISION_ROUNDS;
use bot::{Network, Rng};

/// A policy belongs to the `vision` it trained on: my `vision` or not, and its round.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PolicyKey {
    pub my_vision: bool,
    pub round: usize,
}

// The version goes up when the format or the nets change: old data is then ignored.
fn policy_key(key: PolicyKey) -> String {
    let vision = if key.my_vision { "my" } else { "pretrained" };
    format!("bot_policy_v8_{vision}_r{}", key.round)
}

fn vision_key(round: usize) -> String {
    format!("bot_vision_v4_r{round}")
}

const VISION_RNG_KEY: &str = "bot_vision_v4_rng";

pub fn load_policy(key: PolicyKey) -> Option<String> {
    quad_storage::STORAGE.lock().unwrap().get(&policy_key(key))
}

/// Returns false if the browser did not keep the data.
pub fn save_policy(key: PolicyKey, text: &str) -> bool {
    set_checked(&policy_key(key), text)
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
        .and_then(|text| STANDARD.decode(text).ok())
        .and_then(|bytes| bot::vision_from_bytes(&bytes))
    {
        rounds.push(net);
    }
    let rng = storage
        .get(VISION_RNG_KEY)
        .and_then(|text| text.parse().ok())
        .map(Rng);
    (rounds, rng)
}

/// Returns false if the browser did not keep the data.
pub fn save_vision_round(round: usize, net: &Network, rng: &Rng) -> bool {
    let value = STANDARD.encode(bot::to_bytes(net));
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

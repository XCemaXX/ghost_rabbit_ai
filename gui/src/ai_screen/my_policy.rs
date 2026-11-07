use base64::{Engine, engine::general_purpose::STANDARD};
use bot::policy_job::{POPULATION, Point, Resume};
use bot::train::TestResult;
use bot::{Network, Rng};

/// Policies trained by the player on one `vision`, and the state to go on from the last point.
pub struct MyPolicy {
    pub points: Vec<Point>,
    pub resume: Resume,
}

impl MyPolicy {
    pub fn to_text(&self) -> String {
        let mut bytes = Vec::new();
        bytes.extend((self.points.len() as u32).to_le_bytes());
        for point in &self.points {
            bytes.extend((point.generation as u32).to_le_bytes());
            bytes.extend(point.result.life.to_le_bytes());
            bytes.extend(point.result.score.to_le_bytes());
            bytes.extend(bot::to_bytes(&point.policy));
        }
        bytes.extend((self.resume.generation as u32).to_le_bytes());
        bytes.extend(self.resume.rng.0.to_le_bytes());
        for net in &self.resume.population {
            bytes.extend(bot::to_bytes(net));
        }
        STANDARD.encode(&bytes)
    }

    /// `None` for broken data.
    pub fn from_text(text: &str) -> Option<Self> {
        let bytes = STANDARD.decode(text).ok()?;
        let mut r = Reader(&bytes);
        let mut points = Vec::new();
        for _ in 0..r.u32()? {
            points.push(Point {
                generation: r.u32()? as usize,
                result: TestResult {
                    life: r.f32()?,
                    score: r.f32()?,
                },
                policy: r.net()?,
            });
        }
        let generation = r.u32()? as usize;
        let rng = Rng(u64::from_le_bytes(r.take(8)?.try_into().ok()?));
        let mut population = Vec::new();
        for _ in 0..POPULATION {
            population.push(r.net()?);
        }
        let resume = Resume {
            generation,
            population,
            rng,
        };
        r.0.is_empty().then_some(Self { points, resume })
    }
}

struct Reader<'a>(&'a [u8]);

impl<'a> Reader<'a> {
    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        let (head, tail) = self.0.split_at_checked(n)?;
        self.0 = tail;
        Some(head)
    }

    fn u32(&mut self) -> Option<u32> {
        Some(u32::from_le_bytes(self.take(4)?.try_into().ok()?))
    }

    fn f32(&mut self) -> Option<f32> {
        Some(f32::from_le_bytes(self.take(4)?.try_into().ok()?))
    }

    fn net(&mut self) -> Option<Network> {
        bot::policy_from_bytes(self.take(bot::policy_size_bytes())?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn net(seed: u64) -> Network {
        Network::new(&mut Rng(seed), &bot::policy_topology())
    }

    fn policy() -> MyPolicy {
        MyPolicy {
            points: vec![Point {
                generation: 5,
                result: TestResult {
                    life: 123.5,
                    score: 4567.0,
                },
                policy: net(1),
            }],
            resume: Resume {
                generation: 6,
                population: (0..POPULATION as u64).map(net).collect(),
                rng: Rng(77),
            },
        }
    }

    #[test]
    fn policy_round_trip() {
        let back = MyPolicy::from_text(&policy().to_text()).unwrap();
        assert_eq!(back.points[0].generation, 5);
        assert_eq!(back.points[0].result, policy().points[0].result);
        assert_eq!(
            back.points[0].policy.weights().collect::<Vec<_>>(),
            policy().points[0].policy.weights().collect::<Vec<_>>()
        );
        assert_eq!(back.resume.generation, 6);
        assert_eq!(back.resume.rng.0, 77);
        let weights = |p: &MyPolicy| -> Vec<f32> {
            let last = p.resume.population.last().unwrap();
            last.weights().collect()
        };
        assert_eq!(weights(&back), weights(&policy()));
    }

    #[test]
    fn broken_data_is_ignored() {
        let text = policy().to_text();
        assert!(MyPolicy::from_text(&text[..text.len() - 4]).is_none());
        assert!(MyPolicy::from_text("not base64!").is_none());
    }
}

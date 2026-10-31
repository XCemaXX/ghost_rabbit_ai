use rand_trait::GenRandFloat;

// SplitMix64: small and seedable, so training runs repeat exactly.
#[derive(Clone)]
pub struct Rng(pub u64);

impl Rng {
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }

    pub fn unit(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 / (1u64 << 24) as f32
    }

    pub fn below(&mut self, n: usize) -> usize {
        (self.next_u64() % n as u64) as usize
    }
}

impl GenRandFloat for Rng {
    fn gen_range(&mut self, range: std::ops::RangeInclusive<f32>) -> f32 {
        range.start() + (range.end() - range.start()) * self.unit()
    }
}

// For rollouts on a game copy: always the middle, so the copy knows nothing about the future.
pub struct MidRng;

impl GenRandFloat for MidRng {
    fn gen_range(&mut self, range: std::ops::RangeInclusive<f32>) -> f32 {
        (range.start() + range.end()) * 0.5
    }
}

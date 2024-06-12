pub trait GenRandFloat {
    fn gen_range(&mut self, range: std::ops::RangeInclusive<f32>) -> f32;
}
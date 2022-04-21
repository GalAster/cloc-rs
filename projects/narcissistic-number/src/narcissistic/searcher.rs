use super::*;

pub struct PluperfectDigitalInvariantSearcher {
    base: usize,
}

impl PluperfectDigitalInvariantSearcher {
    pub fn new(base: usize) -> Self {
        Self { base }
    }
}
impl Iterator for PluperfectDigitalInvariantSearcher {
    type Item = BigUint;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

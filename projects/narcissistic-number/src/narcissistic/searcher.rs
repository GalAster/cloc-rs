use super::*;

pub struct PluperfectDigitalInvariantSearcher {
    base: usize,
}

impl PluperfectDigitalInvariantSearcher {
    pub fn new(base: usize) -> Self {
        Self { base }
    }

    pub fn upper_bound(&self) -> usize {
        self.base
    }
}
impl Iterator for PluperfectDigitalInvariantSearcher {
    type Item = BigUint;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

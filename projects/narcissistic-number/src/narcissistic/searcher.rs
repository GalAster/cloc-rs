use super::*;

/// A narcissist number searcher
#[allow(dead_code)]
#[derive(Debug)]
pub struct NarcissisticSearcher {
    base: usize,
    digits: usize,
}

impl NarcissisticSearcher {
    /// Creates a new searcher
    pub fn new(base: usize) -> Self {
        Self { base, digits: 0 }
    }
    /// Returns the upper bound of the search in base-n
    pub fn upper_bound(&self) -> usize {
        self.base
    }
}

impl Iterator for NarcissisticSearcher {
    type Item = NarcissisticNumber;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

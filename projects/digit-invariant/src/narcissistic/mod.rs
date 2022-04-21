use num::BigUint;

mod hardcode;
mod searcher;

use self::hardcode::*;
pub use self::searcher::PluperfectDigitalInvariantSearcher;

pub fn narcissistic_numbers() -> impl Iterator<Item = BigUint> {
    narcissistic::narcissistic_numbers(10)
}

pub fn pluperfect_digital_invariant(base: usize) -> impl Iterator<Item = BigUint> {
    narcissistic::narcissistic_numbers(base as u8)
}

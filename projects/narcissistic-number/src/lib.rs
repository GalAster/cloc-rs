mod narcissistic;

pub use narcissistic::{pluperfect_digital_invariant, PluperfectDigitalInvariantSearcher};
use num::BigUint;

pub fn narcissistic_number(base: usize) -> Vec<BigUint> {
    pluperfect_digital_invariant(base).collect()
}

pub fn perfect_digital_invariant(base: usize) {}

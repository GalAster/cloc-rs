use narcissistic::NarcissisticNumber;
use num::BigUint;

pub fn narcissistic_numbers() -> impl Iterator<Item = NarcissisticNumber> {
    narcissistic::narcissistic_numbers(10)
}

pub fn pluperfect_digital_invariant(base: usize) -> impl Iterator<Item = NarcissisticNumber> {
    narcissistic::narcissistic_numbers(base)
}

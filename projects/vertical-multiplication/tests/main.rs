#![allow(dead_code)]
use digit_invariant::narcissistic_numbers;
use num::BigUint;

#[test]
fn narcissistic() {
    for (i, n) in narcissistic_numbers().enumerate() {
        println!("#{}: {}", i + 1, n);
    }
}

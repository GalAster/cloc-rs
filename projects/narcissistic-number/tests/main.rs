#![allow(dead_code)]
use narcissistic::pluperfect_digital_invariant;
use num::BigUint;
use std::str::FromStr;

#[test]
fn narcissistic() {
    for (i, n) in pluperfect_digital_invariant(10).enumerate() {
        println!("#{}: {}", i + 1, n);
    }
}

fn print_int(list: &[&str]) {
    println!("[");
    for i in list {
        println!("BigUint::from_bytes_le(&{:?}),", BigUint::from_str(i).unwrap().to_bytes_le());
    }
    println!("]");
}

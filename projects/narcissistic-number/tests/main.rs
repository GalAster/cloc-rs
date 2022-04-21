use narcissistic::narcissistic_number;
use num::BigUint;
use std::str::FromStr;

#[test]
fn test() {
    let base10 = &[];
    print_int(base10);
}

fn print_int(list: &[&str]) {
    println!("[");
    for i in list {
        println!("BigUint::from_bytes_le(&{:?}),", BigUint::from_str(i).unwrap().to_bytes_le());
    }
    println!("]");
}

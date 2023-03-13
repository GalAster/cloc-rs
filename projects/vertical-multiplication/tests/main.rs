use num::BigInt;

use vertical_multiplication::v_mul;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    println!("{}", v_mul(&BigInt::from(12), &BigInt::from(345), 10));
}

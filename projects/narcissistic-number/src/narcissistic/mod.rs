use num::bigint::ToBigUint;
use num::BigUint;


pub fn pluperfect_digital_invariant(base: usize) -> Vec<BigUint> {
    match base {
        0 => { panic!() }
        1 => { base1() }
        _ => panic!()
    }
}

#[inline]
fn base1() -> Vec<BigUint> {
    [1u8].iter().map(|u| BigUint::from(*u)).collect()
}

#[test]
fn test() {
    println!("{:?}", base1())
}
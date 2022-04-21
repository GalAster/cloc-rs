use num::bigint::ToBigUint;
use num::BigUint;

const BASE2: [u8; 1] = [1];
const BASE3: [u8; 5] = [1, 2, 5, 8, 17];
const BASE4: [u8; 11] = [1, 2, 3, 28, 29, 35, 43, 55, 62, 83, 243];


pub struct PluperfectDigitalInvariant {

}

pub fn pluperfect_digital_invariant(base: usize) -> Box<dyn Iterator<Item=BigUint>> {
    match base {
        0 | 1 => { panic!("base {} doesn't a valid base", base) }
        2 => {
            Box::new(BASE2.iter().map(|u| BigUint::from(*u)))
        }
        3 => {
            Box::new(BASE2.iter().map(|u| BigUint::from(*u)))
        }

        _ => panic!()
    }
}


#[inline]
fn base1() -> [u8; 1] {
    [1u8]
}

#[inline]
fn base2() -> Vec<BigUint> {
    [1u8, 2, 5, 8, 17].iter().map(|u| BigUint::from(*u)).collect()
}

#[test]
fn test() {
    println!("{:?}", base1())
}
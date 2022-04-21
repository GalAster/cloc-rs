mod hardcode;

use hardcode::BASE10;
use num::BigUint;

pub fn munchausen_numbers(base: usize) -> Box<dyn Iterator<Item = BigUint>> {
    match base {
        0 | 1 => panic!("base {} doesn't a valid base", base),
        10 => Box::new(BASE10.iter().map(|u| BigUint::from(*u))),
        _ => unimplemented!(),
    }
}

mod hardcode;

use self::hardcode::*;
use num::BigUint;

pub fn munchausen_numbers(base: usize) -> Box<dyn Iterator<Item = BigUint>> {
    match base {
        0 | 1 => panic!("base {} doesn't a valid base", base),
        2 => Box::new(BASE2.iter().map(|u| BigUint::from(*u))),
        3 => Box::new(BASE3.iter().map(|u| BigUint::from(*u))),
        4 => Box::new(BASE4.iter().map(|u| BigUint::from(*u))),
        5 => Box::new(BASE5.iter().map(|u| BigUint::from(*u))),
        10 => Box::new(BASE10.iter().map(|u| BigUint::from(*u))),
        _ => unimplemented!(),
    }
}

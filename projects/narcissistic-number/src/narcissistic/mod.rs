use num::{bigint::ToBigUint, BigUint};

mod hardcode;
mod searcher;

use self::hardcode::*;
pub use self::searcher::PluperfectDigitalInvariantSearcher;

pub fn pluperfect_digital_invariant(base: usize) -> Box<dyn Iterator<Item = BigUint>> {
    match base {
        0 | 1 => {
            panic!("base {} doesn't a valid base", base)
        }
        2 => Box::new(BASE2.iter().map(|u| BigUint::from(*u))),
        3 => Box::new(BASE3.iter().map(|u| BigUint::from(*u))),
        4 => Box::new(BASE4.iter().map(|u| BigUint::from(*u))),
        5 => Box::new(BASE4.iter().map(|u| BigUint::from(*u))),
        _ => panic!(),
    }
}

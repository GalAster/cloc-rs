use crate::{BASE10, BASE2, BASE3, BASE4, BASE5, BASE6, BASE7, BASE8, BASE9};
use num::BigUint;
use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
};
mod display;
mod searcher;

pub use self::searcher::NarcissisticSearcher;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NarcissisticNumber {
    base: u8,
    number: BigUint,
}

pub fn narcissistic_numbers(base: u8) -> Box<dyn Iterator<Item = NarcissisticNumber>> {
    macro_rules! box_number {
        ($t:tt) => {
            Box::new($t.iter().map(move |u| NarcissisticNumber::new_unchecked(*u, base)))
        };
    }
    match base {
        0 | 1 => panic!("base {} doesn't a valid base", base),
        2 => box_number!(BASE2),
        3 => box_number!(BASE3),
        4 => box_number!(BASE4),
        5 => box_number!(BASE5),
        6 => box_number!(BASE6),
        7 => box_number!(BASE7),
        8 => box_number!(BASE8),
        9 => box_number!(BASE9),
        10 => box_number!(BASE10),
        _ => Box::new(NarcissisticSearcher::new(base)),
    }
}

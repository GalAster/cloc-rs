use crate::{base11, base12, base13, base14, base15, base16, BASE10, BASE2, BASE3, BASE4, BASE5, BASE6, BASE7, BASE8, BASE9};
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
    pub base: u8,
    pub number: BigUint,
}

pub fn narcissistic_numbers(base: u8) -> Box<dyn Iterator<Item = NarcissisticNumber>> {
    macro_rules! box_number {
        ($t:expr) => {
            Box::new($t.into_iter().map(|u| NarcissisticNumber::new_unchecked(u.clone(), base.clone())))
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
        11 => box_number!(base11()),
        12 => box_number!(base12()),
        13 => box_number!(base13()),
        14 => box_number!(base14()),
        15 => box_number!(base15()),
        16 => box_number!(base16()),
        _ => Box::new(NarcissisticSearcher::new(base)),
    }
}

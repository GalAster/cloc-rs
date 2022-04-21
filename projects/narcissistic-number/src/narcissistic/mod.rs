use crate::{base11, base12, base13, base14, base15, base16, BASE10, BASE2, BASE3, BASE4, BASE5, BASE6, BASE7, BASE8, BASE9};
use num::BigUint;
use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
};
mod display;
mod searcher;

pub use self::searcher::NarcissisticSearcher;

/// Representation of a narcissistic number.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NarcissisticNumber {
    /// base of the number
    pub base: usize,
    /// base-10 representation of the number
    pub number: BigUint,
}

/// Return a iterator that contains all narcissistic numbers of the given base.
pub fn narcissistic_numbers(base: usize) -> Box<dyn Iterator<Item = NarcissisticNumber>> {
    match base {
        0 | 1 => panic!("base {} doesn't a valid base", base),
        2 => Box::new(BASE2.into_iter().map(move |u| NarcissisticNumber::new_unchecked(u, base))),
        3 => Box::new(BASE3.into_iter().map(move |u| NarcissisticNumber::new_unchecked(u, base))),
        4 => Box::new(BASE4.into_iter().map(move |u| NarcissisticNumber::new_unchecked(u, base))),
        5 => Box::new(BASE5.into_iter().map(move |u| NarcissisticNumber::new_unchecked(u, base))),
        6 => Box::new(BASE6.into_iter().map(move |u| NarcissisticNumber::new_unchecked(u, base))),
        7 => Box::new(BASE7.into_iter().map(move |u| NarcissisticNumber::new_unchecked(u, base))),
        8 => Box::new(BASE8.into_iter().map(move |u| NarcissisticNumber::new_unchecked(u, base))),
        9 => Box::new(BASE9.into_iter().map(move |u| NarcissisticNumber::new_unchecked(u, base))),
        10 => Box::new(BASE10.into_iter().map(move |u| NarcissisticNumber::new_unchecked(u, base))),
        11 => Box::new(base11().into_iter().map(move |u| NarcissisticNumber::new_unchecked(u, base))),
        12 => Box::new(base12().into_iter().map(move |u| NarcissisticNumber::new_unchecked(u, base))),
        13 => Box::new(base13().into_iter().map(move |u| NarcissisticNumber::new_unchecked(u, base))),
        14 => Box::new(base14().into_iter().map(move |u| NarcissisticNumber::new_unchecked(u, base))),
        15 => Box::new(base15().into_iter().map(move |u| NarcissisticNumber::new_unchecked(u, base))),
        16 => Box::new(base16().into_iter().map(move |u| NarcissisticNumber::new_unchecked(u, base))),
        _ => Box::new(NarcissisticSearcher::new(base)),
    }
}

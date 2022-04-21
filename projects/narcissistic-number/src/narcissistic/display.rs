use super::*;
use num::{Integer, ToPrimitive, Zero};
use std::{collections::VecDeque, fmt::Write};

impl Display for NarcissisticNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let digits = self.clone().as_digits();
        if self.base <= 36 || !f.alternate() {
            for i in self.clone().as_digits() {
                match i {
                    x @ 0..=9 => f.write_char((b'0' + x) as char)?,
                    x @ 10..=36 => f.write_char((b'A' + (x - 10)) as char)?,
                    _ => unreachable!(),
                }
            }
        }
        else {
            write!(f, "{:?}_{{{}}}", digits, self.base)?
        }
        Ok(())
    }
}

impl PartialOrd for NarcissisticNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.base != other.base {
            return None;
        }
        Some(self.number.cmp(&other.number))
    }
}

impl NarcissisticNumber {
    pub fn new<N: Into<BigUint>>(n: N, base: u8) -> Option<Self> {
        let n = Self::new_unchecked(n.into(), base);
        match n.is_valid() {
            true => Some(n),
            false => None,
        }
    }

    pub fn as_digits(self) -> VecDeque<u8> {
        uint_to_digits(self.number, self.base)
    }
}

impl NarcissisticNumber {
    pub(crate) fn new_unchecked<N: Into<BigUint>>(n: N, base: u8) -> Self {
        Self { base, number: n.into() }
    }
    pub(crate) fn is_valid(&self) -> bool {
        let v = uint_to_digits(self.number.clone(), self.base);
        let p = v.len() as u32;
        v.iter().map(|s| BigUint::from(*s).pow(p)).sum::<BigUint>() == self.number
    }
}

fn uint_to_digits(n: BigUint, base: u8) -> VecDeque<u8> {
    let base = BigUint::from(base);
    let mut out = VecDeque::default();
    // bug?
    #[allow(unused_assignments)]
    let (mut rest, mut digit) = (n, BigUint::zero());
    while !rest.le(&base) {
        (rest, digit) = rest.div_mod_floor(&base);
        unsafe {
            out.push_front(digit.clone().to_u8().unwrap_unchecked());
        }
    }
    unsafe {
        out.push_front(rest.clone().to_u8().unwrap_unchecked());
    }
    out
}

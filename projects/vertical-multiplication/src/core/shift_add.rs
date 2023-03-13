use std::fmt::{Display, Formatter};
use std::ops::Mul;

use num::BigInt;

use crate::core::ShiftAdd;



impl Display for ShiftAdd {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let tail = ".".repeat(self.tail_digits + self.tail_rhs);
        write!(f, "{}{}", self.result, tail)
    }
}

impl ShiftAdd {
    pub fn as_integer(&self) -> BigInt {
        BigInt::from(self.tailing_power()).mul(&self.result)
    }
    pub fn tailing_power(&self) -> BigInt {
        BigInt::from(10).pow(self.tailing_digits() as u32)
    }
    pub fn count_digits(&self) -> usize {
        self.result.to_string().len() + self.tailing_digits()
    }
    pub fn pretty_format(&self, width: usize, leading: &str) -> String {
        // let mut result = leading.to_string();
        let space_width = width - self.count_digits();
        format!("{}{}{}", leading, " ".repeat(space_width), self)
    }
}



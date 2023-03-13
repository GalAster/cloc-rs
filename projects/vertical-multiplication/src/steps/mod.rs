use std::{
    fmt::{Display, Formatter, },
    ops::Mul,
};

use num::{BigInt, Integer, Signed, ToPrimitive};

mod display;
mod shift_add;

#[derive(Debug)]
pub struct MultiplicationSteps {
    base: u32,
    a: BigInt,
    b: BigInt,
    result: BigInt,
    steps: Vec<ShiftAdd>,
}

#[derive(Debug)]
pub struct ShiftAdd {
    /// result of two digits' multiplication
    result: BigInt,
    /// tailing zeros from a
    tail_digits: usize,
}


/// Vertical multiplication in base 10
pub fn v_mul_b10_long(a: &BigInt, b: &BigInt) -> MultiplicationSteps {
    let lhs = get_digits_rev(a, 10);
    let rhs = get_digits_rev(b, 10);
    let mut steps = MultiplicationSteps::new(a, b);
    for (tail_rhs, dx) in rhs.iter().enumerate() {
        for (tail_lhs, dy) in lhs.iter().enumerate() {
            steps.push_step(ShiftAdd::new(dx.mul(dy), tail_rhs + tail_lhs))
        }
    }
    steps
}

impl Display for MultiplicationSteps {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let max_indent = self.max_digits() + 2;
        let a_len = self.a.to_string().len();
        let b_len = self.b.to_string().len();
        writeln!(f, "{space}{}", self.a, space = " ".repeat(max_indent - a_len + 2))?;
        writeln!(f, " ×{space}{}", self.b, space = " ".repeat(max_indent - b_len))?;
        writeln!(f, "{}", "-".repeat(max_indent + 3))?;
        for (i, add) in self.steps.iter().enumerate() {
            if i == 0 {
                writeln!(f, "{}", add.pretty_format(max_indent, " = "))?;
            } else {
                writeln!(f, "{}", add.pretty_format(max_indent, " + "))?;
            }
        }
        writeln!(f, "{}", "-".repeat(max_indent + 3))?;
        writeln!(f, " ={space}{result}", space = " ".repeat(2), result = self.result)
    }
}

impl MultiplicationSteps {
    pub fn new(a: &BigInt, b: &BigInt) -> MultiplicationSteps {
        Self {
            a: a.clone(),
            b: b.clone(),
            steps: vec![],
            result: a.mul(b),
            base: 10,
        }
    }
    pub fn with_base(mut self, base: u32) -> Self {
        self.base = base;
        self
    }
    pub fn push_step(&mut self, step: ShiftAdd) {
        self.steps.push(step);
    }

    pub fn max_digits(&self) -> usize {
        self.steps.iter().map(|x| x.count_digits()).max().unwrap_or(0)
    }
}

pub fn v_mul_b10_short(a: &BigInt, b: &BigInt) -> MultiplicationSteps {
    let rhs = get_digits_rev(b, 10);
    let mut steps = MultiplicationSteps::new(a, b);
    for (tail_rhs, dy) in rhs.iter().enumerate() {
        steps.push_step(ShiftAdd::new(a.mul(BigInt::from(*dy)), tail_rhs))
    }
    steps
}

fn get_digits_rev(num: &BigInt, base: usize) -> Vec<usize> {
    let mut digits = Vec::new();
    let mut num = num.clone();
    while num.is_positive() {
        let (q, r) = num.div_rem(&BigInt::from(base));
        digits.push(r.to_usize().unwrap());
        num = q;
    }
    digits
}

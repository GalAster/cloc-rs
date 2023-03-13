use std::{
    fmt::{Display, Formatter},
    ops::Mul,
};

use num::{BigInt, Integer, Signed, ToPrimitive};

mod shift_add;
mod steps;

/// Get digits of a number in reverse order
#[derive(Debug)]
pub struct MultiplicationSteps {
    base: u32,
    lhs: BigInt,
    rhs: BigInt,
    result: BigInt,
    steps: Vec<ShiftAdd>,
}

/// Get digits of a number in reverse order
#[derive(Debug)]
pub struct ShiftAdd {
    /// result of two digits' multiplication
    result: BigInt,
    /// tailing zeros from a
    tail_digits: usize,
}

/// Vertical multiplication in details
pub fn v_mul_detailed(a: &BigInt, b: &BigInt, base: u32) -> MultiplicationSteps {
    let lhs = get_digits_rev(a, base);
    let rhs = get_digits_rev(b, base);
    let mut steps = MultiplicationSteps::new(a, b).with_base(base);
    for (tail_rhs, dx) in rhs.iter().enumerate() {
        for (tail_lhs, dy) in lhs.iter().enumerate() {
            steps.push_step(ShiftAdd::new(dx.mul(dy), tail_rhs + tail_lhs))
        }
    }
    steps
}

/// Vertical multiplication
pub fn v_mul(a: &BigInt, b: &BigInt, base: u32) -> MultiplicationSteps {
    let rhs = get_digits_rev(b, base);
    let mut steps = MultiplicationSteps::new(a, b).with_base(base);
    for (tail_rhs, dy) in rhs.iter().enumerate() {
        steps.push_step(ShiftAdd::new(a.mul(BigInt::from(*dy)), tail_rhs))
    }
    steps
}

fn get_digits_rev(num: &BigInt, base: u32) -> Vec<usize> {
    let mut digits = Vec::new();
    let mut num = num.clone();
    while num.is_positive() {
        let (q, r) = num.div_rem(&BigInt::from(base));
        digits.push(r.to_usize().unwrap());
        num = q;
    }
    digits
}

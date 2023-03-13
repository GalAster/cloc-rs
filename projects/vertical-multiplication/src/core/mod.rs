use std::fmt::{Display, Formatter};
use std::fmt::Write;
use std::ops::Mul;

use num::{BigInt, Integer, Signed, ToPrimitive};

pub mod shift_add;
pub mod display;

#[derive(Debug)]
pub struct ShiftAdd {
    /// result of two digits' multiplication
    result: BigInt,
    /// tailing zeros from a
    tail_digits: usize,
}


pub struct VerticalDisplay {
    a: BigInt,
    b: BigInt,
    result: BigInt,
    base: u32,
}

/// Vertical multiplication in base 10
pub fn v_mul_b10_long(a: &BigInt, b: &BigInt, lines: &mut String) -> std::fmt::Result {
    let lhs = get_digits_rev(a, 10);
    let rhs = get_digits_rev(b, 10);
    let mut shifts: Vec<ShiftAdd> = vec![];
    for (tail_rhs, dx) in rhs.iter().enumerate() {
        for (tail_lhs, dy) in lhs.iter().enumerate() {
            shifts.push(ShiftAdd {
                result: BigInt::from(*dx) * BigInt::from(*dy),
                tail_lhs,
                tail_rhs,
            })
        }
    }
    let max_indent = shifts.iter().max_by_key(|x| x.count_digits()).unwrap().count_digits() + 1;
    let a_len = a.to_string().len();
    let b_len = b.to_string().len();
    writeln!(lines, "{space}{}", a, space = " ".repeat(max_indent - a_len + 3))?;
    writeln!(lines, " ×{space}{}", b, space = " ".repeat(max_indent - b_len + 1))?;
    writeln!(lines, "{}", "-".repeat(max_indent + 4))?;
    for (i, add) in shifts.iter().enumerate() {
        if i == 0 {
            writeln!(lines, "{}", add.pretty_format(max_indent, " = "))?;
        } else {
            writeln!(lines, "{}", add.pretty_format(max_indent, " + "))?;
        }
    }
    writeln!(lines, "{}", "-".repeat(max_indent + 4))?;
    writeln!(lines, " ={space}{result}", space = " ".repeat(2), result = a * b)?;
    Ok(())
}

pub fn v_mul_b10_short(a: &BigInt, b: &BigInt, lines: &mut String) -> std::fmt::Result {
    let rhs = get_digits_rev(b, 10);
    let mut shifts: Vec<ShiftAdd> = vec![];
    for (tail_rhs, dx) in rhs.iter().enumerate() {
        shifts.push(ShiftAdd {
            result: a * BigInt::from(*dx),
            tail_lhs: 0,
            tail_rhs,
        })
    }
    let max_indent = shifts.iter().max_by_key(|x| x.count_digits()).unwrap().count_digits() + 1;
    let a_len = a.to_string().len();
    let b_len = b.to_string().len();
    writeln!(lines, "{space}{}", a, space = " ".repeat(max_indent - a_len + 3))?;
    writeln!(lines, " ×{space}{}", b, space = " ".repeat(max_indent - b_len + 1))?;
    writeln!(lines, "{}", "-".repeat(max_indent + 4))?;
    for (i, add) in shifts.iter().enumerate() {
        if i == 0 {
            writeln!(lines, "{}", add.pretty_format(max_indent, " = "))?;
        } else {
            writeln!(lines, "{}", add.pretty_format(max_indent, " + "))?;
        }
    }
    writeln!(lines, "{}", "-".repeat(max_indent + 4))?;
    writeln!(lines, " ={space}{result}", space = " ".repeat(2), result = a * b)?;
    Ok(())
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





use std::fmt::Display;
use std::fmt::Write as _;
use std::fs::File;
use std::io::Write;

use num::{BigInt, Integer, Signed, ToPrimitive};

use crate::core::v_mul_b10_short;
pub use crate::core::shift_add::ShiftAdd;

mod core;

#[test]
fn test() {
    let mut out = File::create("out.md").unwrap();
    let mut n = BigInt::from(2);
    for step in 1..=10 {
        let (i, s) = power2(&n, step);
        n = i;
        out.write_all(s.as_bytes()).unwrap();
    }
}

fn power2(n: &BigInt, step: usize) -> (BigInt, String) {
    let mut out = format!("## 第 {} 步\n", step);
    writeln!(out, "```js").unwrap();
    let i = n.pow(2);
    v_mul_b10_short(&n, &n, &mut out).unwrap();
    writeln!(out, "```\n\n").unwrap();
    (i, out)
}
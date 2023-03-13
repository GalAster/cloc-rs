#![forbid(missing_docs)]
#![doc = include_str!("../readme.md")]

pub use crate::steps::{v_mul, v_mul_detailed, MultiplicationSteps, ShiftAdd};

mod steps;
mod utils;

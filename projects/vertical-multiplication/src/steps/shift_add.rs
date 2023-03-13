use super::*;

impl Display for ShiftAdd {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let tail = ".".repeat(self.tail_digits);
        write!(f, "{}{}", self.result, tail)
    }
}

impl ShiftAdd {
    pub fn new<R>(result: R, tail: usize) -> ShiftAdd
    where
        R: Into<BigInt>,
    {
        ShiftAdd { result: result.into(), tail_digits: tail }
    }

    pub fn as_integer(&self, base: u32) -> BigInt {
        BigInt::from(self.tailing_power(base)).mul(&self.result)
    }
    pub fn tailing_power(&self, base: u32) -> BigInt {
        BigInt::from(base).pow(self.tail_digits as u32)
    }
    pub fn count_digits(&self, base: u32) -> usize {
        self.result.to_str_radix(base).len() + self.tail_digits
    }
    pub fn pretty_format(&self, width: usize, leading: &str, base: u32) -> String {
        let space = " ".repeat(width - self.count_digits(base) - leading.len());
        let number = self.result.to_str_radix(base);
        let tail = ".".repeat(self.tail_digits);
        format!("{}{}{}{}", leading, space, number, tail)
    }
}

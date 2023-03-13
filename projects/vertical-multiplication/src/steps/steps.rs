use super::*;

impl Display for MultiplicationSteps {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let max_indent = self.max_digits() + 3;
        let a = self.lhs.to_str_radix(self.base);
        let b = self.rhs.to_str_radix(self.base);
        let r = self.result.to_str_radix(self.base);
        writeln!(f, "{space}{}", a, space = " ".repeat(max_indent - a.len()))?;
        writeln!(f, " ×{space}{}", b, space = " ".repeat(max_indent - b.len() - 2))?;
        writeln!(f, "{}", "-".repeat(max_indent + 1))?;
        for (i, add) in self.steps.iter().enumerate() {
            if i == 0 {
                writeln!(f, "{}", add.pretty_format(max_indent, " = ", self.base))?;
            }
            else {
                writeln!(f, "{}", add.pretty_format(max_indent, " + ", self.base))?;
            }
        }
        writeln!(f, "{}", "-".repeat(max_indent + 1))?;
        write!(f, " = {result}", result = r)
    }
}

impl MultiplicationSteps {
    /// Create a new `MultiplicationSteps` instance
    pub fn new(a: &BigInt, b: &BigInt) -> MultiplicationSteps {
        Self { lhs: a.clone(), rhs: b.clone(), steps: vec![], result: a.mul(b), base: 10 }
    }
    /// Set the base of this multiplication
    pub fn with_base(mut self, base: u32) -> Self {
        assert!(base > 1);
        self.base = base;
        self
    }
    /// Get the base of this multiplication
    pub fn push_step(&mut self, step: ShiftAdd) {
        self.steps.push(step);
    }
    /// Get the number of digits in the largest step
    pub fn max_digits(&self) -> usize {
        self.steps.iter().map(|x| x.count_digits(self.base)).max().unwrap_or(0)
    }
}

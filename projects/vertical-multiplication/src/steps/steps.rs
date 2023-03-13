use super::*;

impl Display for MultiplicationSteps {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let max_indent = self.max_digits() + 2;
        let a = self.lhs.to_str_radix(self.base);
        let b = self.rhs.to_str_radix(self.base);
        let r = self.result.to_str_radix(self.base);
        writeln!(f, "{space}{}", a, space = " ".repeat(max_indent - a.len() + 2))?;
        writeln!(f, " ×{space}{}", b, space = " ".repeat(max_indent - b.len()))?;
        writeln!(f, "{}", "-".repeat(max_indent + 3))?;
        for (i, add) in self.steps.iter().enumerate() {
            if i == 0 {
                writeln!(f, "{}", add.pretty_format(max_indent, " = ", self.base))?;
            }
            else {
                writeln!(f, "{}", add.pretty_format(max_indent, " + ", self.base))?;
            }
        }
        writeln!(f, "{}", "-".repeat(max_indent + 3))?;
        write!(f, " ={space}{result}", space = " ".repeat(2), result = r)
    }
}

impl MultiplicationSteps {
    pub fn new(a: &BigInt, b: &BigInt) -> MultiplicationSteps {
        Self { lhs: a.clone(), rhs: b.clone(), steps: vec![], result: a.mul(b), base: 10 }
    }
    pub fn with_base(mut self, base: u32) -> Self {
        self.base = base;
        self
    }
    pub fn push_step(&mut self, step: ShiftAdd) {
        self.steps.push(step);
    }

    pub fn max_digits(&self) -> usize {
        self.steps.iter().map(|x| x.count_digits(self.base)).max().unwrap_or(0)
    }
}

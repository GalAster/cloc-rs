use super::*;

impl Display for MultiplicationSteps {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let max_indent = self.max_digits() + 2;
        let a_len = self.a.to_str_radix(self.base).len();
        let b_len = self.b.to_str_radix(self.base).len();
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
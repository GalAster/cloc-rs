use super::*;

impl Display for NarcissisticNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.base)
    }
}

impl PartialOrd for NarcissisticNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.base != other.base {
            return None;
        }
        Some(self.number.cmp(&other.number))
    }
}

impl NarcissisticNumber {
    pub fn new<N: Into<BigUint>>(n: N, base: u8) -> Self {
        Self { base: base as usize, number: n.into() }
    }

    pub fn get_digits(&self) -> Vec<u8> {
        vec![]
    }
    pub fn get_number(&self) -> BigUint {
        self.number.clone()
    }
    pub fn get_base(&self) -> usize {
        self.base
    }
}

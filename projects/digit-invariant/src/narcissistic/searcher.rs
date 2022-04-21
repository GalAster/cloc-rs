use super::*;

pub struct PluperfectDigitalInvariantSearcher {
    base: usize,
    digits: usize,
}

impl PluperfectDigitalInvariantSearcher {
    pub fn new(base: usize) -> Self {
        Self { base, digits: 0 }
    }

    pub fn is_valid(&self, n: &BigUint) -> bool {
        // let mut digits = Vec::with_capacity(self.digits);
        // let base = BigUint::from(self.base);
        // let mut rest = n.clone();
        // let mut digit = BigUint::zero();
        // loop {
        //     let (rest, digit) = rest.div_mod_floor(&base);
        //     println!("{:?}", (rest, digit));
        //     unsafe {
        //         digits.push(digit.to_u8().unwrap_unchecked());
        //     }
        //     if rest.le(&base) {
        //         break;
        //     }
        // }
        let _ = n;
        todo!()
    }
    pub fn to_digits(&self, n: &BigUint) -> Vec<u8> {
        let _ = n;
        vec![]
    }

    pub fn upper_bound(&self) -> usize {
        self.base
    }
}

impl Iterator for PluperfectDigitalInvariantSearcher {
    type Item = BigUint;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

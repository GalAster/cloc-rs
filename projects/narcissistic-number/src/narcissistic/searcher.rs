use super::*;

#[allow(dead_code)]
pub struct NarcissisticSearcher {
    base: usize,
    digits: usize,
}

impl NarcissisticSearcher {
    pub fn new(base: u8) -> Self {
        Self { base: base as usize, digits: 0 }
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

impl Iterator for NarcissisticSearcher {
    type Item = NarcissisticNumber;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

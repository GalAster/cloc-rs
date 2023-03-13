use num::{BigInt, Integer, Signed, ToPrimitive};

#[test]
fn test() {
    v_mul_b10(&BigInt::from(123), &BigInt::from(456));
}

///
fn v_mul_b10(a: &BigInt, b: &BigInt) {
    let lhs = get_digits_rev(a, 10);
    let rhs = get_digits_rev(b, 10);
    println!("{:?}", lhs);
    println!("{:?}", rhs);

}


fn get_digits_rev(num: &BigInt, base: usize) -> Vec<u8> {
    let mut digits = Vec::new();
    let mut num = num.clone();
    while num.is_positive() {
        let (q, r) = num.div_rem(&BigInt::from(base));
        digits.push(r.to_u8().unwrap());
        num = q;
    }
    digits
}
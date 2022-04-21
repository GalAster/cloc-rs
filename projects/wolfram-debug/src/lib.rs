use num::BigUint;
use std::str::FromStr;
use wolfram_library_link::{
    export,
    expr::{Expr, ExprKind},
};

#[export(wstp)]
fn encode_int(args: Vec<Expr>) -> Expr {
    match args.as_slice() {
        [s] => match s.kind() {
            ExprKind::String(s) => {
                Expr::list(BigUint::from_str(s).unwrap().to_bytes_le().iter().map(|n| Expr::from(*n)).collect())
            }
            _ => panic!(),
        },
        _ => panic!(),
    }
}

#[test]
fn main() {}

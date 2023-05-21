mod cst;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub calc1); // synthesized by LALRPOP

#[test]
fn calc1() {
    let src = include_str!("examples/json.kiki");
    calc1::FileParser::new()
        .parse(src)
        .expect("should parse correctly");
}

#[cfg(test)]
mod impl_cst {
    use crate::cst::*;

    impl OptItems {
        pub fn len(&self) -> usize {
            match self {
                OptItems::Nil => 0,
                OptItems::Cons(tail, _) => 1 + tail.len(),
            }
        }
    }
}

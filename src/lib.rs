mod cst;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub calc1); // synthesized by LALRPOP

#[test]
fn calc1() {
    assert_eq!(0, calc1::FileParser::new().parse(" ").unwrap().items.len());
    assert_eq!(
        1,
        calc1::FileParser::new()
            .parse("[item]")
            .unwrap()
            .items
            .len()
    );
    assert_eq!(
        2,
        calc1::FileParser::new()
            .parse("[item] [item]")
            .unwrap()
            .items
            .len()
    );

    assert!(calc1::FileParser::new().parse("foo").is_err());
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

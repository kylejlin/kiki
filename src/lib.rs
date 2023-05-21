mod cst;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub calc1); // synthesized by LALRPOP

#[test]
fn calc1() {
    assert_eq!(Ok(0), calc1::TermParser::new().parse("_"));
    assert_eq!(Ok(1), calc1::TermParser::new().parse("foo"));
    assert_eq!(Ok(2), calc1::TermParser::new().parse("(((($foo2))))"));

    assert!(calc1::TermParser::new().parse("2").is_err());
}

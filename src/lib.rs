mod cst;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub calc1); // synthesized by LALRPOP

#[test]
fn calc1() {
    assert!(calc1::TermParser::new().parse("_").is_ok());
    assert!(calc1::TermParser::new().parse("foo").is_ok());
    assert!(calc1::TermParser::new().parse("((((foo2))))").is_ok());
    assert!(calc1::TermParser::new().parse("2").is_err());
}

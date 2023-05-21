mod cst;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub calc1); // synthesized by LALRPOP

#[test]
fn calc1() {
    assert!(calc1::TermParser::new().parse("22").is_ok());
    assert!(calc1::TermParser::new().parse("(22)").is_ok());
    assert!(calc1::TermParser::new().parse("((((22))))").is_ok());
    assert!(calc1::TermParser::new().parse("((22)").is_err());
}

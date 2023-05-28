use super::*;

#[test]
fn json() {
    let src = include_str!("examples/json.kiki");
    parser::FileParser::new()
        .parse(src)
        .expect("should parse correctly");
}

#[test]
fn kiki() {
    let src = include_str!("examples/kiki.kiki");
    parser::FileParser::new()
        .parse(src)
        .expect("should parse correctly");
}

#[test]
fn balanced_parens() {
    let src = include_str!("examples/balanced_parens.kiki");
    parser::FileParser::new()
        .parse(src)
        .expect("should parse correctly");
}

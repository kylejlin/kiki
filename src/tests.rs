use super::*;

#[test]
fn json() {
    let src = include_str!("examples/json.kiki");
    assert_src_passes_pre_machine_validation(src);
}

#[test]
fn kiki() {
    let src = include_str!("examples/kiki.kiki");
    assert_src_passes_pre_machine_validation(src);
}

#[test]
fn balanced_parens() {
    let src = include_str!("examples/balanced_parens.kiki");
    assert_src_passes_pre_machine_validation(src);
}

#[test]
fn balanced_parens_esoteric() {
    let src = include_str!("examples/balanced_parens_esoteric.kiki");
    assert_src_passes_pre_machine_validation(src);
}

fn assert_src_passes_pre_machine_validation(src: &str) {
    let cst = parser::FileParser::new()
        .parse(src)
        .expect("should parse correctly");
    let ast: crate::data::ast::File = cst.into();
    crate::logic::ast_to_validated_file::ast_to_validated_file(ast)
        .expect("should pass pre-machine validation");
}

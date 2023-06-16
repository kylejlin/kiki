use super::*;

use crate::data::KikiErr;

#[test]
fn lowercase_nonterminal() {
    let src = include_str!("../examples/should_fail/lowercase_nonterminal.kiki");
    let err = assert_src_fails_pre_machine_validation(src);
    assert!(matches!(
        err,
        KikiErr::SymbolOrTerminalEnumNameFirstLetterNotUppercase(_)
    ));
}

#[test]
fn lowercase_nonterminal_variant_name() {
    let src = include_str!("../examples/should_fail/lowercase_nonterminal_variant_name.kiki");
    let err = assert_src_fails_pre_machine_validation(src);
    assert!(matches!(
        err,
        KikiErr::SymbolOrTerminalEnumNameFirstLetterNotUppercase(_)
    ));
}

#[test]
fn lowercase_terminal() {
    let src = include_str!("../examples/should_fail/lowercase_terminal.kiki");
    let err = assert_src_fails_pre_machine_validation(src);
    assert!(matches!(
        err,
        KikiErr::SymbolOrTerminalEnumNameFirstLetterNotUppercase(_)
    ));
}

#[test]
fn lowercase_terminal_enum_name() {
    let src = include_str!("../examples/should_fail/lowercase_terminal_enum_name.kiki");
    let err = assert_src_fails_pre_machine_validation(src);
    assert!(matches!(
        err,
        KikiErr::SymbolOrTerminalEnumNameFirstLetterNotUppercase(_)
    ));
}

#[test]
fn uppercase_field() {
    let src = include_str!("../examples/should_fail/uppercase_field.kiki");
    let err = assert_src_fails_pre_machine_validation(src);
    assert!(matches!(err, KikiErr::FieldFirstLetterNotLowercase(_)));
}

#[test]
fn undefined_start() {
    let src = include_str!("../examples/should_fail/undefined_start.kiki");
    let err = assert_src_fails_pre_machine_validation(src);
    assert!(matches!(err, KikiErr::UndefinedNonterminal(name, _) if name == "Fo"));
}

#[test]
fn undefined_child_nonterminal() {
    let src = include_str!("../examples/should_fail/undefined_child_nonterminal.kiki");
    let err = assert_src_fails_pre_machine_validation(src);
    assert!(matches!(err, KikiErr::UndefinedNonterminal(name, _) if name == "Bar"));
}

#[test]
fn undefined_child_terminal() {
    let src = include_str!("../examples/should_fail/undefined_child_terminal.kiki");
    let err = assert_src_fails_pre_machine_validation(src);
    assert!(matches!(err, KikiErr::UndefinedTerminal(name, _) if name.raw() == "Baz"));
}

#[test]
fn clash_nonterminal_terminal_enum_name() {
    let src = include_str!("../examples/should_fail/clash_nonterminal_terminal_enum_name.kiki");
    let err = assert_src_fails_pre_machine_validation(src);
    assert!(matches!(err, KikiErr::NameClash(name, _, _) if name == "Qux"));
}

#[test]
fn clash_nonterminal_nonterminal() {
    let src = include_str!("../examples/should_fail/clash_nonterminal_nonterminal.kiki");
    let err = assert_src_fails_pre_machine_validation(src);
    assert!(matches!(err, KikiErr::NameClash(name, _, _) if name == "Lorem"));
}

#[test]
fn clash_terminal_terminal() {
    let src = include_str!("../examples/should_fail/clash_terminal_terminal.kiki");
    let err = assert_src_fails_pre_machine_validation(src);
    assert!(matches!(err, KikiErr::NameClash(name, _, _) if name == "Num"));
}

#[test]
fn clash_terminal_nonterminal() {
    let src = include_str!("../examples/should_fail/clash_terminal_nonterminal.kiki");
    let err = assert_src_fails_pre_machine_validation(src);
    assert!(matches!(err, KikiErr::NameClash(name, _, _) if name == "Blah"));
}

#[test]
fn clash_terminal_terminal_enum_name() {
    let src = include_str!("../examples/should_fail/clash_terminal_terminal_enum_name.kiki");
    let err = assert_src_fails_pre_machine_validation(src);
    assert!(matches!(err, KikiErr::NameClash(name, _, _) if name == "Tok"));
}

#[test]
fn clash_nonterminal_enum_variant_name() {
    let src = include_str!("../examples/should_fail/clash_nonterminal_enum_variant_name.kiki");
    let err = assert_src_fails_pre_machine_validation(src);
    assert!(matches!(err, KikiErr::NonterminalEnumVariantNameClash(name, _, _) if name == "Ipsum"));
}

#[test]
fn clash_nonterminal_enum_variant_symbol_sequence() {
    let src =
        include_str!("../examples/should_fail/clash_nonterminal_enum_variant_symbol_sequence.kiki");
    let err = assert_src_fails_pre_machine_validation(src);
    let expected_seq = vec![
        Symbol::Nonterminal("Fraction".into()),
        Symbol::Nonterminal("Fraction".into()),
    ];
    assert!(
        matches!(err, KikiErr::NonterminalEnumVariantSymbolSequenceClash(seq, _, _) if seq == expected_seq)
    );
}

#[test]
fn no_start() {
    let src = include_str!("../examples/should_fail/no_start.kiki");
    let err = assert_src_fails_pre_machine_validation(src);
    assert!(matches!(err, KikiErr::NoStartSymbol));
}

#[test]
fn no_terminal_enum() {
    let src = include_str!("../examples/should_fail/no_terminal_enum.kiki");
    let err = assert_src_fails_pre_machine_validation(src);
    assert!(matches!(err, KikiErr::NoTerminalEnum));
}

#[test]
fn multiple_starts() {
    let src = include_str!("../examples/should_fail/multiple_starts.kiki");
    let err = assert_src_fails_pre_machine_validation(src);
    assert!(matches!(err, KikiErr::MultipleStartSymbols(starts) if starts.len() == 2));
}

#[test]
fn multiple_terminal_enums() {
    let src = include_str!("../examples/should_fail/multiple_terminal_enums.kiki");
    let err = assert_src_fails_pre_machine_validation(src);
    assert!(
        matches!(err, KikiErr::MultipleTerminalEnums(terminal_enums) if terminal_enums.len() == 2)
    );
}

fn assert_src_fails_pre_machine_validation(src: &str) -> KikiErr {
    let tokens = tokenize(src).expect("Should be able to tokenize correctly");
    let cst = parse(tokens).expect("should parse correctly");
    let ast: crate::data::ast::File = cst.into();
    crate::pipeline::validate_ast::validate_ast(ast)
        .expect_err("should fail pre-machine validation")
}

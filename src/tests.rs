use super::*;

mod should_succeed {
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
}

mod should_fail {
    use super::*;

    use crate::data::KikiErr;

    #[test]
    fn lowercase_nonterminal() {
        let src = include_str!("examples/should_fail/lowercase_nonterminal.kiki");
        let err = assert_src_fails_pre_machine_validation(src);
        assert!(matches!(err, KikiErr::SymbolFirstLetterNotUppercase(_)));
    }

    #[test]
    fn lowercase_terminal() {
        let src = include_str!("examples/should_fail/lowercase_terminal.kiki");
        let err = assert_src_fails_pre_machine_validation(src);
        assert!(matches!(err, KikiErr::SymbolFirstLetterNotUppercase(_)));
    }

    #[test]
    fn uppercase_field() {
        let src = include_str!("examples/should_fail/uppercase_field.kiki");
        let err = assert_src_fails_pre_machine_validation(src);
        assert!(matches!(err, KikiErr::FieldFirstLetterNotLowercase(_)));
    }

    #[test]
    fn undefined_start() {
        let src = include_str!("examples/should_fail/undefined_start.kiki");
        let err = assert_src_fails_pre_machine_validation(src);
        assert!(matches!(err, KikiErr::UndefinedNonterminal(name, _) if name == "Fo"));
    }

    #[test]
    fn undefined_child_nonterminal() {
        let src = include_str!("examples/should_fail/undefined_child_nonterminal.kiki");
        let err = assert_src_fails_pre_machine_validation(src);
        assert!(matches!(err, KikiErr::UndefinedNonterminal(name, _) if name == "Bar"));
    }

    fn assert_src_fails_pre_machine_validation(src: &str) -> KikiErr {
        let cst = parser::FileParser::new()
            .parse(src)
            .expect("should parse correctly");
        let ast: crate::data::ast::File = cst.into();
        crate::logic::ast_to_validated_file::ast_to_validated_file(ast)
            .expect_err("should fail pre-machine validation")
    }
}

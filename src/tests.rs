use super::*;

mod should_succeed {
    use super::*;

    #[test]
    fn json() {
        let src = include_str!("examples/json.kiki");
        let rust_src = generate(src).expect("should generate Rust code");
        insta::assert_debug_snapshot!(rust_src);
    }

    #[test]
    fn kiki() {
        let src = include_str!("examples/kiki.kiki");
        let rust_src = generate(src).expect("should generate Rust code");
        insta::assert_debug_snapshot!(rust_src);
    }

    #[test]
    fn balanced_parens() {
        let src = include_str!("examples/balanced_parens.kiki");
        let rust_src = generate(src).expect("should generate Rust code");
        insta::assert_debug_snapshot!(rust_src);
    }

    #[test]
    fn balanced_parens_esoteric() {
        let src = include_str!("examples/balanced_parens_esoteric.kiki");
        let rust_src = generate(src).expect("should generate Rust code");
        insta::assert_debug_snapshot!(rust_src);
    }
}

mod should_fail {
    use super::*;

    use crate::data::KikiErr;

    #[test]
    fn lowercase_nonterminal() {
        let src = include_str!("examples/should_fail/lowercase_nonterminal.kiki");
        let err = assert_src_fails_pre_machine_validation(src);
        assert!(matches!(
            err,
            KikiErr::SymbolOrTerminalEnumNameFirstLetterNotUppercase(_)
        ));
    }

    #[test]
    fn lowercase_nonterminal_variant_name() {
        let src = include_str!("examples/should_fail/lowercase_nonterminal_variant_name.kiki");
        let err = assert_src_fails_pre_machine_validation(src);
        assert!(matches!(
            err,
            KikiErr::SymbolOrTerminalEnumNameFirstLetterNotUppercase(_)
        ));
    }

    #[test]
    fn lowercase_terminal() {
        let src = include_str!("examples/should_fail/lowercase_terminal.kiki");
        let err = assert_src_fails_pre_machine_validation(src);
        assert!(matches!(
            err,
            KikiErr::SymbolOrTerminalEnumNameFirstLetterNotUppercase(_)
        ));
    }

    #[test]
    fn lowercase_terminal_enum_name() {
        let src = include_str!("examples/should_fail/lowercase_terminal_enum_name.kiki");
        let err = assert_src_fails_pre_machine_validation(src);
        assert!(matches!(
            err,
            KikiErr::SymbolOrTerminalEnumNameFirstLetterNotUppercase(_)
        ));
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

    #[test]
    fn undefined_child_terminal() {
        let src = include_str!("examples/should_fail/undefined_child_terminal.kiki");
        let err = assert_src_fails_pre_machine_validation(src);
        assert!(matches!(err, KikiErr::UndefinedTerminal(name, _) if name.raw() == "Baz"));
    }

    #[test]
    fn clash_nonterminal_terminal_enum_name() {
        let src = include_str!("examples/should_fail/clash_nonterminal_terminal_enum_name.kiki");
        let err = assert_src_fails_pre_machine_validation(src);
        assert!(matches!(err, KikiErr::NameClash(name, _, _) if name == "Qux"));
    }

    #[test]
    fn clash_nonterminal_nonterminal() {
        let src = include_str!("examples/should_fail/clash_nonterminal_nonterminal.kiki");
        let err = assert_src_fails_pre_machine_validation(src);
        assert!(matches!(err, KikiErr::NameClash(name, _, _) if name == "Lorem"));
    }

    #[test]
    fn clash_terminal_terminal() {
        let src = include_str!("examples/should_fail/clash_terminal_terminal.kiki");
        let err = assert_src_fails_pre_machine_validation(src);
        assert!(matches!(err, KikiErr::NameClash(name, _, _) if name == "Num"));
    }

    #[test]
    fn clash_terminal_nonterminal() {
        let src = include_str!("examples/should_fail/clash_terminal_nonterminal.kiki");
        let err = assert_src_fails_pre_machine_validation(src);
        assert!(matches!(err, KikiErr::NameClash(name, _, _) if name == "Blah"));
    }

    #[test]
    fn clash_terminal_terminal_enum_name() {
        let src = include_str!("examples/should_fail/clash_terminal_terminal_enum_name.kiki");
        let err = assert_src_fails_pre_machine_validation(src);
        assert!(matches!(err, KikiErr::NameClash(name, _, _) if name == "Tok"));
    }

    #[test]
    fn clash_nonterminal_enum_variant_name() {
        let src = include_str!("examples/should_fail/clash_nonterminal_enum_variant_name.kiki");
        let err = assert_src_fails_pre_machine_validation(src);
        assert!(
            matches!(err, KikiErr::NonterminalEnumVariantNameClash(name, _, _) if name == "Ipsum")
        );
    }

    #[test]
    fn clash_nonterminal_enum_variant_symbol_sequence() {
        let src = include_str!(
            "examples/should_fail/clash_nonterminal_enum_variant_symbol_sequence.kiki"
        );
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
        let src = include_str!("examples/should_fail/no_start.kiki");
        let err = assert_src_fails_pre_machine_validation(src);
        assert!(matches!(err, KikiErr::NoStartSymbol));
    }

    #[test]
    fn no_terminal_enum() {
        let src = include_str!("examples/should_fail/no_terminal_enum.kiki");
        let err = assert_src_fails_pre_machine_validation(src);
        assert!(matches!(err, KikiErr::NoTerminalEnum));
    }

    #[test]
    fn multiple_starts() {
        let src = include_str!("examples/should_fail/multiple_starts.kiki");
        let err = assert_src_fails_pre_machine_validation(src);
        assert!(matches!(err, KikiErr::MultipleStartSymbols(starts) if starts.len() == 2));
    }

    #[test]
    fn multiple_terminal_enums() {
        let src = include_str!("examples/should_fail/multiple_terminal_enums.kiki");
        let err = assert_src_fails_pre_machine_validation(src);
        assert!(
            matches!(err, KikiErr::MultipleTerminalEnums(terminal_enums) if terminal_enums.len() == 2)
        );
    }

    fn assert_src_fails_pre_machine_validation(src: &str) -> KikiErr {
        let cst = parser::FileParser::new()
            .parse(src)
            .expect("should parse correctly");
        let ast: crate::data::ast::File = cst.into();
        crate::pipeline::validate_ast::validate_ast(ast)
            .expect_err("should fail pre-machine validation")
    }
}

mod get_grammar_hash_tests {
    use super::*;

    #[test]
    fn balanced_parens() {
        let src = r#"// This code was generated by Kiki.
// Kiki is an open-source minimalist parser generator for Rust.
// You can read more at https://crates.io/crates/kiki
//
// This code was generated from a grammar with the following hash:
// @sha256 544841a802e160291407ce92621aabf4940055abd8d03e7964dde46f873aff2b"#;

        let expected = Some("544841a802e160291407ce92621aabf4940055abd8d03e7964dde46f873aff2b");

        let actual = get_grammar_hash(RustSrcRef(src));

        assert_eq!(actual, expected);
    }

    #[test]
    fn esoteric_balanced_parens() {
        let src = r#"// This code was generated by Kiki.
// Kiki is an open-source minimalist parser generator for Rust.
// You can read more at https://crates.io/crates/kiki
//
// This code was generated from a grammar with the following hash:
// @sha256 8b2fc853275605761dc2ff01845103bfe20bf78a0ed8c6ecb4bab49cb74ced46
"#;

        let expected = Some("8b2fc853275605761dc2ff01845103bfe20bf78a0ed8c6ecb4bab49cb74ced46");

        let actual = get_grammar_hash(RustSrcRef(src));

        assert_eq!(actual, expected);
    }
}

use super::*;
use crate::{test_utils::*, ByteIndex};

use pretty_assertions::assert_eq;

#[test]
fn balanced_parens() {
    let file = balanced_parens_input();
    let actual = validated_ast_to_machine(&file);
    let expected = balanced_parens_expected_output();
    assert_eq!(actual, expected);
}

pub fn balanced_parens_input() -> File {
    File {
        start: "Expr".to_owned(),
        terminal_enum: TerminalEnum {
            attributes: vec![positionless_attribute("#[derive(Clone, Debug)]")],
            name: "Token".to_string(),
            variants: vec![
                TerminalVariant {
                    dollarless_name: remove_dollars("LParen"),
                    type_: "()".to_string(),
                },
                TerminalVariant {
                    dollarless_name: remove_dollars("RParen"),
                    type_: "()".to_string(),
                },
            ],
        },
        nonterminals: vec![Nonterminal::Enum(Enum {
            attributes: vec![positionless_attribute("#[derive(Clone, Debug)]")],
            name: positionless_ident("Expr"),
            variants: vec![
                EnumVariant {
                    name: positionless_ident("Empty"),
                    fieldset: Fieldset::Empty,
                },
                EnumVariant {
                    name: positionless_ident("Wrap"),
                    fieldset: Fieldset::Tuple(TupleFieldset {
                        fields: vec![
                            TupleField::Used(IdentOrTerminalIdent::Terminal(
                                positionless_terminal_ident(&remove_dollars("LParen")),
                            )),
                            TupleField::Used(IdentOrTerminalIdent::Ident(positionless_ident(
                                "Expr",
                            ))),
                            TupleField::Used(IdentOrTerminalIdent::Terminal(
                                positionless_terminal_ident(&remove_dollars("RParen")),
                            )),
                        ],
                    }),
                },
            ],
        })],
    }
}

pub fn balanced_parens_expected_output() -> Machine {
    use crate::data::machine::{Lookahead::*, RuleIndex::*};

    normalize_machine(UnnormalizedMachine {
        states: vec![
            State {
                items: [
                    StateItem {
                        rule_index: Augmented,
                        lookahead: Eof,
                        dot: 0,
                    },
                    StateItem {
                        rule_index: Original(0),
                        lookahead: Eof,
                        dot: 0,
                    },
                    StateItem {
                        rule_index: Original(1),
                        lookahead: Eof,
                        dot: 0,
                    },
                ]
                .into_iter()
                .collect(),
            },
            State {
                items: [StateItem {
                    rule_index: Augmented,
                    lookahead: Eof,
                    dot: 1,
                }]
                .into_iter()
                .collect(),
            },
            State {
                items: [
                    StateItem {
                        rule_index: Original(1),
                        lookahead: Eof,
                        dot: 1,
                    },
                    StateItem {
                        rule_index: Original(0),
                        lookahead: Terminal(remove_dollars("RParen")),
                        dot: 0,
                    },
                    StateItem {
                        rule_index: Original(1),
                        lookahead: Terminal(remove_dollars("RParen")),
                        dot: 0,
                    },
                    StateItem {
                        rule_index: Original(1),
                        lookahead: Terminal(remove_dollars("RParen")),
                        dot: 1,
                    },
                ]
                .into_iter()
                .collect(),
            },
            State {
                items: [
                    StateItem {
                        rule_index: Original(1),
                        lookahead: Eof,
                        dot: 2,
                    },
                    StateItem {
                        rule_index: Original(1),
                        lookahead: Terminal(remove_dollars("RParen")),
                        dot: 2,
                    },
                ]
                .into_iter()
                .collect(),
            },
            State {
                items: [
                    StateItem {
                        rule_index: Original(1),
                        lookahead: Eof,
                        dot: 3,
                    },
                    StateItem {
                        rule_index: Original(1),
                        lookahead: Terminal(remove_dollars("RParen")),
                        dot: 3,
                    },
                ]
                .into_iter()
                .collect(),
            },
        ],
        transitions: [
            Transition {
                from: StateIndex(0),
                to: StateIndex(1),
                symbol: Symbol::Nonterminal("Expr".to_owned()),
            },
            Transition {
                from: StateIndex(0),
                to: StateIndex(2),
                symbol: Symbol::Terminal(remove_dollars("LParen")),
            },
            Transition {
                from: StateIndex(2),
                to: StateIndex(3),
                symbol: Symbol::Nonterminal("Expr".to_owned()),
            },
            Transition {
                from: StateIndex(2),
                to: StateIndex(2),
                symbol: Symbol::Terminal(remove_dollars("LParen")),
            },
            Transition {
                from: StateIndex(3),
                to: StateIndex(4),
                symbol: Symbol::Terminal(remove_dollars("RParen")),
            },
        ]
        .into_iter()
        .collect(),
    })
}

#[test]
fn balanced_parens_esoteric() {
    let file = balanced_parens_esoteric_input();
    let actual = validated_ast_to_machine(&file);
    let expected = balanced_parens_esoteric_expected_output();
    assert_eq!(actual, expected);
}

fn balanced_parens_esoteric_input() -> File {
    File {
        start: "Expr".to_owned(),
        terminal_enum: TerminalEnum {
            attributes: vec![positionless_attribute("#[derive(Clone, Debug)]")],
            name: "Token".to_string(),
            variants: vec![
                TerminalVariant {
                    dollarless_name: DollarlessTerminalName::remove_dollars("LParen"),
                    type_: "()".to_string(),
                },
                TerminalVariant {
                    dollarless_name: DollarlessTerminalName::remove_dollars("RParen"),
                    type_: "()".to_string(),
                },
            ],
        },
        nonterminals: vec![Nonterminal::Enum(Enum {
            attributes: vec![positionless_attribute("#[derive(Clone, Debug)]")],
            name: positionless_ident("Expr"),
            variants: vec![
                EnumVariant {
                    name: positionless_ident("Empty"),
                    fieldset: Fieldset::Empty,
                },
                EnumVariant {
                    name: positionless_ident("Wrap"),
                    fieldset: Fieldset::Named(NamedFieldset {
                        fields: vec![
                            NamedField {
                                name: IdentOrUnderscore::Underscore(ByteIndex(0)),
                                symbol: IdentOrTerminalIdent::Terminal(
                                    positionless_terminal_ident(
                                        &DollarlessTerminalName::remove_dollars("LParen"),
                                    ),
                                ),
                            },
                            NamedField {
                                name: IdentOrUnderscore::Ident(positionless_ident("inner")),
                                symbol: IdentOrTerminalIdent::Ident(positionless_ident("Expr")),
                            },
                            NamedField {
                                name: IdentOrUnderscore::Ident(positionless_ident("right")),
                                symbol: IdentOrTerminalIdent::Terminal(
                                    positionless_terminal_ident(
                                        &DollarlessTerminalName::remove_dollars("RParen"),
                                    ),
                                ),
                            },
                        ],
                    }),
                },
            ],
        })],
    }
}

fn balanced_parens_esoteric_expected_output() -> Machine {
    balanced_parens_expected_output()
}

// Since we can use `use` with struct namespaces,
// we must use a `const`, as a hack.
#[allow(non_upper_case_globals)]
const remove_dollars: fn(&str) -> DollarlessTerminalName = DollarlessTerminalName::remove_dollars;

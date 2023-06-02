use super::*;
use crate::{
    data::ast::{EnumDef, EnumVariant, Ident, TerminalIdent, TupleField},
    ByteIndex,
};

use pretty_assertions::assert_eq;

// Since we can use `use` with struct namespaces,
// we must use a `const`, as a hack.
#[allow(non_upper_case_globals)]
const remove_dollars: fn(&str) -> DollarlessTerminalName = DollarlessTerminalName::remove_dollars;

#[test]
fn balanced_parens_snapshot() {
    let file = File {
        start: "Expr".to_owned(),
        terminal_enum: TerminalEnum {
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
        nonterminals: vec![Nonterminal::Enum(EnumDef {
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
    };

    let machine = validated_ast_to_machine(&file);
    insta::assert_debug_snapshot!(machine);
}

#[test]
fn balanced_parens_manual() {
    let file = File {
        start: "Expr".to_owned(),
        terminal_enum: TerminalEnum {
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
        nonterminals: vec![Nonterminal::Enum(EnumDef {
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
    };

    let actual = validated_ast_to_machine(&file);

    use crate::data::machine::{Lookahead::*, RuleIndex::*};
    let expected = update_state_indices(
        vec![
            State {
                items: [
                    Item {
                        rule_index: Augmented,
                        lookahead: Eof,
                        dot: 0,
                    },
                    Item {
                        rule_index: Original(0),
                        lookahead: Eof,
                        dot: 0,
                    },
                    Item {
                        rule_index: Original(1),
                        lookahead: Eof,
                        dot: 0,
                    },
                ]
                .into_iter()
                .collect(),
            },
            State {
                items: [Item {
                    rule_index: Augmented,
                    lookahead: Eof,
                    dot: 1,
                }]
                .into_iter()
                .collect(),
            },
            State {
                items: [
                    Item {
                        rule_index: Original(1),
                        lookahead: Eof,
                        dot: 1,
                    },
                    Item {
                        rule_index: Original(0),
                        lookahead: Terminal(remove_dollars("RParen")),
                        dot: 0,
                    },
                    Item {
                        rule_index: Original(1),
                        lookahead: Terminal(remove_dollars("RParen")),
                        dot: 0,
                    },
                    Item {
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
                    Item {
                        rule_index: Original(1),
                        lookahead: Eof,
                        dot: 2,
                    },
                    Item {
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
                    Item {
                        rule_index: Original(1),
                        lookahead: Eof,
                        dot: 3,
                    },
                    Item {
                        rule_index: Original(1),
                        lookahead: Terminal(remove_dollars("RParen")),
                        dot: 3,
                    },
                ]
                .into_iter()
                .collect(),
            },
        ],
        [
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
    );

    assert_eq!(actual, expected);
}

fn positionless_ident(s: &str) -> Ident {
    Ident {
        name: s.to_owned(),
        position: ByteIndex(0),
    }
}

fn positionless_terminal_ident(s: &DollarlessTerminalName) -> TerminalIdent {
    TerminalIdent {
        dollared_name: format!("${}", s.raw()),
        position: ByteIndex(0),
    }
}
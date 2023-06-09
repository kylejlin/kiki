use crate::examples::balanced_parens_esoteric::{parse, Expr, Token};

use pretty_assertions::assert_eq;

use std::fmt::Debug;

#[test]
fn empty() {
    let actual = parse([]).unwrap();
    let expected = Expr::Empty;
    assert_eq!(expected, actual)
}

#[test]
fn balanced_1() {
    let actual = parse([lparen(), rparen()]).unwrap();
    let expected = Expr::Wrap {
        inner: Box::new(Expr::Empty),
        right: (),
    };
    assert_eq!(expected, actual)
}

#[test]
fn balanced_2() {
    let actual = parse([lparen(), lparen(), rparen(), rparen()]).unwrap();
    let expected = Expr::Wrap {
        inner: Box::new(Expr::Wrap {
            inner: Box::new(Expr::Empty),
            right: (),
        }),
        right: (),
    };
    assert_eq!(expected, actual)
}

#[test]
fn unexpected_eof() {
    let actual = parse([lparen()]).unwrap_err();
    let expected = None;
    assert_eq!(expected, actual)
}

#[test]
fn unexpected_lparen() {
    let actual = parse([lparen(), lparen(), rparen(), lparen()]).unwrap_err();
    let expected = Some(lparen());
    assert_eq!(expected, actual)
}

#[test]
fn unexpected_rparen() {
    let actual = parse([rparen(), lparen()]).unwrap_err();
    let expected = Some(rparen());
    assert_eq!(expected, actual)
}

fn lparen() -> Token {
    Token::LParen(())
}

fn rparen() -> Token {
    Token::RParen(())
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Expr::Empty, Expr::Empty) => true,
            (
                Expr::Wrap {
                    inner: a_inner,
                    right: (),
                },
                Expr::Wrap {
                    inner: b_inner,
                    right: (),
                },
            ) => a_inner == b_inner,
            _ => false,
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::LParen(()), Token::LParen(())) => true,
            (Token::RParen(()), Token::RParen(())) => true,
            _ => false,
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::LParen(()) => write!(f, "LParen"),
            Token::RParen(()) => write!(f, "RParen"),
        }
    }
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Empty => write!(f, "Empty"),
            Expr::Wrap { inner, right: () } => f.debug_tuple("Wrap").field(inner).finish(),
        }
    }
}

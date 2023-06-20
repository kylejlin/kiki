use crate::examples::balanced_parens_with_outer_attributes::{parse, Expr, Token};

use pretty_assertions::assert_eq;

#[test]
fn empty() {
    let actual = parse([]).unwrap();
    let expected = Expr::Empty;
    assert_eq!(expected, actual)
}

#[test]
fn balanced_1() {
    let actual = parse([lparen(), rparen()]).unwrap();
    let expected = Expr::Wrap(Box::new(Expr::Empty));
    assert_eq!(expected, actual)
}

#[test]
fn balanced_2() {
    let actual = parse([lparen(), lparen(), rparen(), rparen()]).unwrap();
    let expected = Expr::Wrap(Box::new(Expr::Wrap(Box::new(Expr::Empty))));
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

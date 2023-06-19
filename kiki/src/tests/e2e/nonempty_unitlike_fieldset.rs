use crate::examples::nonempty_unitlike_fieldset::{
    parse, Foo, NumberPair, Pair, StringPair, Token,
};

use pretty_assertions::assert_eq;

#[test]
fn empty() {
    let actual = parse([]).unwrap();
    let expected = Foo::Empty;
    assert_eq!(expected, actual)
}

#[test]
fn number() {
    let actual = parse([num(5)]).unwrap();
    let expected = Foo::Number;
    assert_eq!(expected, actual)
}

#[test]
fn string_pair() {
    let actual = parse([string("foo"), string("bar")]).unwrap();
    let expected = Foo::Pair {
        val: Box::new(Pair::StringPair(Box::new(StringPair))),
    };
    assert_eq!(expected, actual)
}

#[test]
fn number_pair() {
    let actual = parse([num(8), num(9)]).unwrap();
    let expected = Foo::Pair {
        val: Box::new(Pair::NumberPair(Box::new(NumberPair))),
    };
    assert_eq!(expected, actual)
}

#[test]
fn standalone_string() {
    let actual = parse([string("baz")]).unwrap_err();
    let expected = None;
    assert_eq!(expected, actual)
}

#[test]
fn number_string() {
    let actual = parse([num(3), string("qux")]).unwrap_err();
    let expected = Some(string("qux"));
    assert_eq!(expected, actual)
}

#[test]
fn string_number() {
    let actual = parse([string("quack"), num(4)]).unwrap_err();
    let expected = Some(num(4));
    assert_eq!(expected, actual)
}

fn string(s: &str) -> Token {
    Token::String(s.to_string())
}

fn num(n: isize) -> Token {
    Token::Number(n)
}

impl PartialEq for Foo {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Foo::Empty, Foo::Empty) => true,
            (Foo::Number, Foo::Number) => true,
            (Foo::Pair { val: a }, Foo::Pair { val: b }) => a == b,
            _ => false,
        }
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Pair::StringPair(a), Pair::StringPair(b)) => a == b,
            (Pair::NumberPair(a), Pair::NumberPair(b)) => a == b,
            _ => false,
        }
    }
}

impl PartialEq for StringPair {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (StringPair, StringPair) => true,
        }
    }
}

impl PartialEq for NumberPair {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NumberPair, NumberPair) => true,
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::String(a), Token::String(b)) => a == b,
            (Token::Number(a), Token::Number(b)) => a == b,
            _ => false,
        }
    }
}

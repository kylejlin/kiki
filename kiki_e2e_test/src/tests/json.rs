use crate::examples::json::{
    parse, Arr, Elements, Entries, Entry, Expr, Json, Obj, OptElements, OptEntries, Token,
};

use pretty_assertions::assert_eq;

use std::fmt::Debug;

#[test]
fn empty_obj() {
    let actual = parse([lcurly(), rcurly()]).unwrap();
    let expected = Json::Obj(Box::new(Obj {
        entries: Box::new(OptEntries::None),
    }));
    assert_eq!(expected, actual)
}

#[test]
fn empty_arr() {
    let actual = parse([lsquare(), rsquare()]).unwrap();
    let expected = Json::Arr(Box::new(Arr {
        elements: Box::new(OptElements::None),
    }));
    assert_eq!(expected, actual)
}

#[test]
fn student_data() {
    // ```json
    // {
    //     "name": "Taro",
    //     "age": 30,
    //     "skills": ["C", {"name": "Scheme", "level": 3}, "Python"],
    //     "married": false
    // }
    // ```
    let actual = parse([
        lcurly(),
        string("name"),
        colon(),
        string("Taro"),
        comma(),
        string("age"),
        colon(),
        num("30"),
        comma(),
        string("skills"),
        colon(),
        lsquare(),
        string("C"),
        comma(),
        lcurly(),
        string("name"),
        colon(),
        string("Scheme"),
        comma(),
        string("level"),
        colon(),
        num("3"),
        rcurly(),
        comma(),
        string("Python"),
        rsquare(),
        comma(),
        string("married"),
        colon(),
        bool("false"),
        rcurly(),
    ])
    .unwrap();
    let expected = {
        let skills = Expr::Arr(Box::new(Arr {
            elements: Box::new(OptElements::Some(Box::new(Elements::Many(
                Box::new(Elements::Many(
                    Box::new(Elements::One(Box::new(Expr::String("C".to_string())))),
                    Box::new(Expr::Obj(Box::new(Obj {
                        entries: Box::new(OptEntries::Some(Box::new(Entries::Many(
                            Box::new(Entries::One(Box::new(Entry {
                                key: "name".to_string(),
                                val: Box::new(Expr::String("Scheme".to_string())),
                            }))),
                            Box::new(Entry {
                                key: "level".to_string(),
                                val: Box::new(Expr::Num("3".to_string())),
                            }),
                        )))),
                    }))),
                )),
                Box::new(Expr::String("Python".to_string())),
            )))),
        }));
        Json::Obj(Box::new(Obj {
            entries: Box::new(OptEntries::Some(Box::new(Entries::Many(
                Box::new(Entries::Many(
                    Box::new(Entries::Many(
                        Box::new(Entries::One(Box::new(Entry {
                            key: "name".to_string(),
                            val: Box::new(Expr::String("Taro".to_string())),
                        }))),
                        Box::new(Entry {
                            key: "age".to_string(),
                            val: Box::new(Expr::Num("30".to_string())),
                        }),
                    )),
                    Box::new(Entry {
                        key: "skills".to_string(),
                        val: Box::new(skills),
                    }),
                )),
                Box::new(Entry {
                    key: "married".to_string(),
                    val: Box::new(Expr::Bool("false".to_string())),
                }),
            )))),
        }))
    };

    assert_eq!(expected, actual)
}

#[test]
fn empty() {
    let actual = parse([]).unwrap_err();
    let expected = None;
    assert_eq!(expected, actual)
}

#[test]
fn expecting_rcurly() {
    let actual = parse([lcurly()]).unwrap_err();
    let expected = None;
    assert_eq!(expected, actual)
}

#[test]
fn expecting_rsquare() {
    let actual = parse([lsquare()]).unwrap_err();
    let expected = None;
    assert_eq!(expected, actual)
}

#[test]
fn unexpected_rcurly() {
    let actual = parse([rcurly()]).unwrap_err();
    let expected = Some(rcurly());
    assert_eq!(expected, actual)
}

#[test]
fn unexpected_rsquare() {
    let actual = parse([rsquare()]).unwrap_err();
    let expected = Some(rsquare());
    assert_eq!(expected, actual)
}

#[test]
fn unexpected_lcurly() {
    let actual = parse([lcurly(), lcurly(), rcurly(), rcurly()]).unwrap_err();
    let expected = Some(lcurly());
    assert_eq!(expected, actual)
}

#[test]
fn unexpected_lsquare() {
    let actual = parse([lcurly(), lsquare(), rsquare(), rcurly()]).unwrap_err();
    let expected = Some(lsquare());
    assert_eq!(expected, actual)
}

#[test]
fn unexpected_string() {
    let actual = parse([string("foo")]).unwrap_err();
    let expected = Some(string("foo"));
    assert_eq!(expected, actual)
}

#[test]
fn unexpected_num() {
    let actual = parse([lcurly(), num("42"), colon(), num("9"), rcurly()]).unwrap_err();
    let expected = Some(num("42"));
    assert_eq!(expected, actual)
}

fn lcurly() -> Token {
    Token::LCurly("{".to_string())
}

fn rcurly() -> Token {
    Token::RCurly("}".to_string())
}

fn lsquare() -> Token {
    Token::LSquare("[".to_string())
}

fn rsquare() -> Token {
    Token::RSquare("]".to_string())
}

fn colon() -> Token {
    Token::Colon(":".to_string())
}

fn comma() -> Token {
    Token::Comma(",".to_string())
}

fn string(s: &str) -> Token {
    Token::String(s.to_string())
}

fn num(s: &str) -> Token {
    Token::Num(s.to_string())
}

fn bool(s: &str) -> Token {
    Token::Bool(s.to_string())
}

impl PartialEq for Json {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Json::Obj(a_obj), Json::Obj(b_obj)) => a_obj == b_obj,
            (Json::Arr(a_arr), Json::Arr(b_arr)) => a_arr == b_arr,
            _ => false,
        }
    }
}

impl PartialEq for Obj {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Obj { entries: a_entries }, Obj { entries: b_entries }) => a_entries == b_entries,
        }
    }
}

impl PartialEq for OptEntries {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (OptEntries::None, OptEntries::None) => true,
            (OptEntries::Some(a_entries), OptEntries::Some(b_entries)) => a_entries == b_entries,
            _ => false,
        }
    }
}

impl PartialEq for Entries {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Entries::One(a_entry), Entries::One(b_entry)) => a_entry == b_entry,
            (Entries::Many(a_entries, a_entry), Entries::Many(b_entries, b_entry)) => {
                a_entries == b_entries && a_entry == b_entry
            }
            _ => false,
        }
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Entry {
                    key: a_key,
                    val: a_val,
                },
                Entry {
                    key: b_key,
                    val: b_val,
                },
            ) => a_key == b_key && a_val == b_val,
        }
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Expr::Obj(a_obj), Expr::Obj(b_obj)) => a_obj == b_obj,
            (Expr::Arr(a_arr), Expr::Arr(b_arr)) => a_arr == b_arr,
            (Expr::String(a_string), Expr::String(b_string)) => a_string == b_string,
            (Expr::Num(a_num), Expr::Num(b_num)) => a_num == b_num,
            (Expr::Bool(a_bool), Expr::Bool(b_bool)) => a_bool == b_bool,
            _ => false,
        }
    }
}

impl PartialEq for Arr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Arr {
                    elements: a_elements,
                },
                Arr {
                    elements: b_elements,
                },
            ) => a_elements == b_elements,
        }
    }
}

impl PartialEq for OptElements {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (OptElements::None, OptElements::None) => true,
            (OptElements::Some(a_elements), OptElements::Some(b_elements)) => {
                a_elements == b_elements
            }
            _ => false,
        }
    }
}

impl PartialEq for Elements {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Elements::One(a_expr), Elements::One(b_expr)) => a_expr == b_expr,
            (Elements::Many(a_elements, a_expr), Elements::Many(b_elements, b_expr)) => {
                a_elements == b_elements && a_expr == b_expr
            }
            _ => false,
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::LCurly(a), Token::LCurly(b)) => a == b,
            (Token::RCurly(a), Token::RCurly(b)) => a == b,
            (Token::LSquare(a), Token::LSquare(b)) => a == b,
            (Token::RSquare(a), Token::RSquare(b)) => a == b,
            (Token::Colon(a), Token::Colon(b)) => a == b,
            (Token::Comma(a), Token::Comma(b)) => a == b,
            (Token::String(a), Token::String(b)) => a == b,
            (Token::Num(a), Token::Num(b)) => a == b,
            (Token::Bool(a), Token::Bool(b)) => a == b,
            _ => false,
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::LCurly(s) => write!(f, "LCurly({s})"),
            Token::RCurly(s) => write!(f, "RCurly({s})"),
            Token::LSquare(s) => write!(f, "LSquare({s})"),
            Token::RSquare(s) => write!(f, "RSquare({s})"),
            Token::Colon(s) => write!(f, "Colon({s})"),
            Token::Comma(s) => write!(f, "Comma({s})"),
            Token::String(s) => write!(f, "String({s})"),
            Token::Num(s) => write!(f, "Num({s})"),
            Token::Bool(s) => write!(f, "Bool({s})"),
        }
    }
}

impl Debug for Json {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Json::Obj(obj) => f.debug_tuple("Obj").field(obj).finish(),
            Json::Arr(arr) => f.debug_tuple("Arr").field(arr).finish(),
        }
    }
}

impl Debug for Obj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Obj")
            .field("entries", &self.entries)
            .finish()
    }
}

impl Debug for OptEntries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptEntries::None => f.debug_tuple("None").finish(),
            OptEntries::Some(entries) => f.debug_tuple("Some").field(entries).finish(),
        }
    }
}

impl Debug for Entries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Entries::One(entry) => f.debug_tuple("One").field(entry).finish(),
            Entries::Many(entries, entry) => {
                f.debug_tuple("Many").field(entries).field(entry).finish()
            }
        }
    }
}

impl Debug for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Entry")
            .field("key", &self.key)
            .field("val", &self.val)
            .finish()
    }
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Obj(obj) => f.debug_tuple("Obj").field(obj).finish(),
            Expr::Arr(arr) => f.debug_tuple("Arr").field(arr).finish(),
            Expr::String(s) => f.debug_tuple("String").field(s).finish(),
            Expr::Num(s) => f.debug_tuple("Num").field(s).finish(),
            Expr::Bool(s) => f.debug_tuple("Bool").field(s).finish(),
        }
    }
}

impl Debug for Arr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Arr")
            .field("elements", &self.elements)
            .finish()
    }
}

impl Debug for OptElements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptElements::None => f.debug_tuple("None").finish(),
            OptElements::Some(elements) => f.debug_tuple("Some").field(elements).finish(),
        }
    }
}

impl Debug for Elements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Elements::One(expr) => f.debug_tuple("One").field(expr).finish(),
            Elements::Many(elements, expr) => {
                f.debug_tuple("Many").field(elements).field(expr).finish()
            }
        }
    }
}

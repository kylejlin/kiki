use crate::examples::json_with_outer_attributes::{
    parse, Arr, Elements, Entries, Entry, Expr, Json, Obj, OptElements, OptEntries, Token,
};

use pretty_assertions::assert_eq;

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

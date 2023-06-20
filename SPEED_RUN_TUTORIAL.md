# Kiki "speed run" tutorial

You can learn Kiki in under ten minutes.
The fastest way is just to look at Kiki code
side-by-side with the resulting Rust code.

This tutorial does not explain anything.
The whole idea is that **it's faster to learn by example**.
However, if you want an explanation,
read the [user guide](./USER_GUIDE.md).

Having said that, let's get started!

## Table of contents

- [`json_parser.kiki`](#json_parserkiki)
- [`json_parser.rs`](#json_parserrs)
- [Epilogue](#epilogue)

## `json_parser.kiki`

```kiki
#[derive(Debug, Clone)]
terminal Token {
    $String: String
    $Num: String
    $Bool: String
    $LCurly: String
    $RCurly: String
    $LSquare: String
    $RSquare: String
    $Colon: String
    $Comma: String
}

start Json

enum Json {
    Obj(Obj)
    Arr(Arr)
}

struct Obj {
    _: $LCurly
    entries: OptEntries
    _: $RCurly
}

enum OptEntries {
    None
    Some(Entries)
}

enum Entries {
    One(Entry)
    Many(
        Entries
        _: $Comma
        Entry
    )
}

struct Entry {
    key: $String
    _: $Colon
    val: Expr
}

enum Expr {
    Obj(Obj)
    Arr(Arr)
    String($String)
    Num($Num)
    Bool($Bool)
}

struct Arr {
    _: $LSquare
    elements: OptElements
    _: $RSquare
}

enum OptElements {
    None
    Some(Elements)
}

enum Elements {
    One(Expr)
    Many(
        Elements
        _: $Comma
        Expr
    )
}
```

## `json_parser.rs`

```rust
#[derive(Debug, Clone)]
pub enum Token {
    String(String),
    Num(String),
    Bool(String),
    LCurly(String),
    RCurly(String),
    LSquare(String),
    RSquare(String),
    Colon(String),
    Comma(String),
}

pub enum Json {
    Obj(Box<Obj>),
    Arr(Box<Arr>),
}

pub struct Obj {
    pub entries: Box<OptEntries>,
}

pub enum OptEntries {
    None,
    Some(Box<Entries>),
}

pub enum Entries {
    One(Box<Entry>),
    Many(Box<Entries>, Box<Entry>),
}

pub struct Entry {
    pub key: String,
    pub val: Box<Expr>,
}

pub enum Expr {
    Obj(Box<Obj>),
    Arr(Box<Arr>),
    String(String),
    Num(String),
    Bool(String),
}

pub struct Arr {
    pub elements: Box<OptElements>,
}

pub enum OptElements {
    None,
    Some(Box<Elements>),
}

pub enum Elements {
    One(Box<Expr>),
    Many(Box<Elements>, Box<Expr>),
}

/// If the parser encounters an unexpected token `t`, it will return `Err(Some(t))`.
/// If the parser encounters an unexpected end of input, it will return `Err(None)`.
pub fn parse<S>(src: S) -> Result<Json, Option<Token>>
where S: IntoIterator<Item = Token> {
    // ...
}
```

## Epilogue

That's all folks!
If you're still confused,
you can read the [guide](./USER_GUIDE.md).
After that, if you still have questions, feel free to
open an issue.

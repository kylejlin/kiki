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
```

## `json_parser.rs`

The following Rust code is a simplified version
of what Kiki generates using the above grammar.
We omit the `#[derive(...)]` and `pub` clauses,
in order to make the code easier to read.

```rust
enum Token {
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

enum Json {
    Obj(Box<Obj>),
    Arr(Box<Arr>),
}

struct Obj {
    entries: Box<OptEntries>,
}

enum OptEntries {
    None,
    Some(Box<Entries>),
}

enum Entries {
    One(Box<Entry>),
    Many(Box<Entries>, Box<Entry>),
}

struct Entry {
    key: String,
    val: Box<Expr>,
}

enum Expr {
    Obj(Box<Obj>),
    Arr(Box<Arr>),
    String(String),
    Num(String),
    Bool(String),
}

struct Arr {
    elements: Box<OptElements>,
}

enum OptElements {
    None,
    Some(Box<Elements>),
}

enum Elements {
    One(Box<Expr>),
    Many(Box<Elements>, Box<Expr>),
}

/// If the parser encounters an unexpected token `t`, it will return `Err(Some(t))`.
/// If the parser encounters an unexpected end of input, it will return `Err(None)`.
fn parse<S>(src: S) -> Result<Json, Option<Token>>
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

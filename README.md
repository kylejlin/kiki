# Kiki

Kiki is a minimalist parser generator for Rust.
Kiki's primary design goal is simplicity.
If you've used other parser generators before (e.g., Bison/yacc),
you can **learn Kiki in under 5 minutes**.

## Feature overview

- Supports LALR(1) grammars
- The generated parser builds a _concrete_ (i.e., not abstract) syntax tree
- No semantic actions

## Example

`zero_or_more_balanced_parens.kiki`:

```kiki
start Balanced

enum Balanced {
    Empty
    Wrap(
        _: $Left
        Balanced
        _: $Right
    )
}

terminal Token {
    $Left: String
    $Right: String
}
```

## More examples

`json.kiki`:

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

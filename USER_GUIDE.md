# Kiki user guide

This guide is meant to be a (mostly) comprehensive reference.
It is not meant to be a quick read.
If you want to learn Kiki quickly as possible,
click [here](./QUICKSTART.md).

This guide is incomplete.
It is full of inconsistencies, typos, and grammar mistakes.
However, an experienced programmer should be able to fill in
the holes.
I don't plan to revise the guide, since virtually nobody uses Kiki
(at the time of writing)
and I'm very busy.
However, if the number of users increases in the future,
I will consider revising the guide.

## Table of contents

- [Prerequisites](#prerequisites)
- [Statements](#statements)
  - [`terminal` declarations](#terminal-declarations)
  - [`struct` declarations](#struct-declarations)
  - [`enum` declarations](#enum-declarations)
  - [`start` declarations](#start-declarations)
- [Identifier names](#identifier-names)
- [Terminal names](#terminal-names)
- [Capitalization](#capitalization)
- [Comments](#comments)

## Prerequisites

This guide assumes that you know what a [context-free grammar](https://en.wikipedia.org/wiki/Context-free_grammar) is.
You must understand the following terms:

- Terminal symbol (henceforth abbreviated as "terminal")
- Nonterminal symbol (henceforth abbreviated as "nonterminal")
- Production rule (henceforth abbreviated as "rule")
- Start symbol

## Statements

A Kiki file consists of multiple _statements_.
A statement can be a

- `terminal` declaration
- `struct` declaration
- `enum` declaration
- `start` declaration

A file must have **exactly one** `terminal` declaration
and **exactly one** `start` declaration.
A file can have unlimited `enum` and `struct` declarations.

## `terminal` declarations

`terminal` declarations let you
define each terminal's Rust type.
For example, suppose we have a lexer
that spits out three kinds of terminals:

1. `StringLiteral`, with a Rust type of `String`.
2. `NumberLiteral`, with a Rust type of `isize`.
3. `Comma`, with a Rust type of `()` (the unit type).

We write:

```kiki
terminal Token {
    $StringLiteral: String
    $NumberLiteral: isize
    $Comma: ()
}
```

This results in the following Rust:

```rs
enum Token {
    StringLiteral(String),
    NumberLiteral(isize),
    Comma(()),
}
```

Terminal names must begin with `$`.
The first letter of a terminal name must be uppercase.

### Crate-defined types

Up until now, we've only used types defined in
Rust's `core` or `std`.
Suppose we want our terminals to have types
that _we_ define.
For example, we write the following `lib.rs`:

```rs
struct Ident {
    name: String,
    span: (usize, usize),
}

struct NumLit {
    val: isize,
    span: (usize, usize),
}

struct StrLit {
    val: String,
    span: (usize, usize),
}
```

We can write the following Kiki:

```kiki
terminal Token {
    $Ident: crate::Ident
    $NumLit: crate::NumLit
    $StrLit: crate::StrLit
}
```

Custom types must use fully qualified syntax (i.e., `crate::foo::bar::...`).

### Outer attributes

You can write zero or more _outer_ attributes
before the terminal enum declaration.
An outer attribute has the form `#[...]`,
where `...` is a _balanced bracket string_.
We explain what a balanced bracket string is later.

But first, let's see an example of adding
an outer attribute to a struct:

```kiki
#[derive(Clone, Copy, Debug)] // <-- This is the outer attribute
terminal Token {
    $LParen: ()
    $RParen: ()
}
```

The above code results in the Rust code below:

```rs
#[derive(Clone, Copy, Debug)]
pub enum Token {
    LParen(()),
    RParen(()),
}
```

Observe that the outer attribute
(i.e., `#[derive(Clone, Copy, Debug)]`)
gets copied verbatim into the generated Rust code.

#### Balanced bracket strings

A balanced bracket string consists of
balanced brackets (i.e., `()`, `[]`, `{}`).
A balanced bracket string may also
include non-bracket characters.
However, the non-bracket characters do not influence
a string's balancedness status.

Here are some examples.

The following strings _are_ balanced:

- `"hello world"`
- `"()"`
- `"hel(lo world)"`
- `"derive(Debug, Clone, Foo { target = Bar })"`
- `""` (the empty string)
- `([{}], {} {[]}, ())`

The following strings are **not** balanced:

- `(` (reason: expected `)` but got end of input)
- `)` (reason: unexpected `)`)
- `([)]` (reason: expected `]` but got `)`)

## `struct` declarations

Suppose we have the following grammar rule

```bnf
<CommaSeparatedNumberPair> ::= NumberLiteral Comma NumberLiteral
```

...and we want to store it in the Rust data type

```rs
struct CommaSeparatedNumberPair {
    left: isize,
    comma: (),
    right: isize,
}
```

We can write the following Kiki:

```kiki
struct CommaSeparatedNumberPair {
    left: $NumberLiteral
    comma: $Comma
    right: $NumberLiteral
}

terminal Token {
    $Comma: ()
    $NumberLiteral: isize
}
```

Observe how the data type definition implicitly defines the grammar.

### Using `_` to omit fields

The `comma: ()` field is rather useless--in Rust,
there's never a point in storing a zero sized type like `()`.
We can tell Kiki to omit that field using underscore syntax:

```kiki
struct CommaSeparatedNumberPair {
    left: $NumberLiteral
    _: $Comma
    right: $NumberLiteral
}

terminal Token {
    $Comma: ()
    $NumberLiteral: isize
}
```

This results in the following Rust:

```rust
struct CommaSeparatedNumberPair {
    left: isize,
    right: isize,
}
```

### Tuple structs

You can also use tuple syntax, just like in Rust:

```kiki
struct CommaSeparatedNumberPair(
    $NumberLiteral
    _: $Comma
    $NumberLiteral
)

terminal Token {
    $NumberLiteral: isize
}
```

This results in

```rust
struct CommaSeparatedNumberPair(isize, isize);
```

Observe that you can use `_` syntax to omit tuple fields,
like we do in `_: $Comma`.

### Unit-like structs

For empty productions, you can use unit-like structs.
In practice, this is never useful.
However, here's an example:

```kiki
struct Epsilon
```

This corresponds to the following grammar:

```bnf
<Epsilon> ::=
```

Notice the RHS is empty.

The above Kiki generates the following Rust:

```rs
struct Epsilon;
```

### Outer attributes before structs

You can write zero or more _outer_ attributes
before a struct.
You use the same syntax as when you write [outer attributes before a terminal enum declaration](#outer-attributes).

Example:

```kiki
#[derive(Clone, Debug)]
struct ReturnStatement(
    _: $ReturnKw
    Expr
    _: $Semicolon
)
```

The above code results in the Rust code below:

```rs
#[derive(Clone, Debug)]
pub struct ReturnStatement(Box<Expr>);
```

## `enum` declarations

Up until now, we've only considered nonterminals
with one production rule.
What if there are multiple production rules?
For example, suppose we have the following grammar...

```bnf
<OneOrMoreNumbers> ::= NumLit
                     | <OneOrMoreNumbers> NumLit
```

...and we want to store it in the Rust type:

```rs
enum OneOrMoreNumbers {
    One(isize),
    Many(Box<OneOrMoreNumbers>, isize)
}
```

We can write the following Kiki:

```kiki
enum OneOrMoreNumbers {
    One($NumLit)
    Many(
        OneOrMoreNumbers
        $NumLit
    )
}
```

### `_` syntax

Just like with `struct` declarations, you can use `_` to omit fields.
For example:

```kiki
enum CommaSeparatedNumbers {
    One($NumLit)
    Many(
        CommaSeparatedNumbers
        _: $Comma
        $NumLit
    )
}

terminal Token {
    $NumLit: isize
    $Comma: ()
}
```

This results in:

```rust
enum CommaSeparatedNumbers {
    One(isize),
    Many(
        Box<CommaSeparatedNumbers>,
        isize,
    ),
}
```

### Named variant fields

An enum variant can use named fields, just like in Rust:

```kiki
enum OneOrMoreNumbers {
    One { val: $NumLit }
    Many {
        left: OneOrMoreNumbers
        right: $NumLit
    }
}

terminal Token {
    $NumLit: isize
}
```

This results in the following Rust:

```rs
enum OneOrMoreNumbers {
    One {
        val: isize
    },
    Many {
        left: Box<OneOrMoreNumbers>,
        right: isize,
    },
}
```

### Unit-like variants

An enum variant can use unit-like syntax, just like in Rust:

```kiki
enum ZeroOrOneComma {
    None
    One(_: $Comma)
}
```

This results in :

```rs
enum ZeroOrOneComma {
    None,
    One,
}
```

### Outer attributes before enums

You can write zero or more _outer_ attributes
before a struct.
You use the same syntax as when you write [outer attributes before a terminal enum declaration](#outer-attributes).

Example:

```kiki
#[derive(Clone, Copy, Debug)]
enum ZeroOrOneComma {
    None
    One(_: $Comma)
}
```

The above code results in the Rust code below:

```rs
#[derive(Clone, Copy, Debug)]
enum ZeroOrOneComma {
    None,
    One,
}
```

## `start` declarations

You must specify a starting symbol for the grammar.
Use a `start` declaration, like so:

```kiki
start FooBar
```

The start symbol must be a nonterminal symbol.

Note that a `start` declaration does _not_ define the referenced
symbol (i.e., `FooBar`, in this case).
You must define it separately using a `struct` or `enum` definition,
as explained in previous sections.

## Identifier names

Identifier names can begin with an ASCII letter or underscore,
followed by zero or more ASCII letters, digits, or underscores.
You cannot name an identifier `terminal`, `start`, `struct` , or `enum`.
These four words are Kiki's only reserved words.

You cannot name an identifier any word that is reserved in Rust.
This is not enforced by Kiki.
That is, if you name an identifier using a reserved word
(e.g., `impl`), Kiki will _not_ produce an error.
However, the generated Rust code will almost certainly fail to compile.

Good:

- `FuncDef`
- `lhs`
- `rhs`

Bad:

- `terminal`
- `start`
- `struct`
- `enum`
- `impl`
- `mod`
- `123`
- `!#$@`

## Terminal names

A terminal name is valid iff it is of the form `$<ident_name>`,
where `<ident_name>` is a valid identifier name.

Good:

- `$Comma`
- `$LParen`
- `$NumberLiteral`

Bad:

- `$start`
- `$123`
- `$$`

## Capitalization

The following must start with an uppercase letter:

- Terminal names

  ```kiki
  // RIGHT
  terminal Token {
      $NumLit: isize
  }

  // WRONG
  terminal Token {
      $num_lit: isize
  }
  ```

- Nonterminal names

  ```kiki
  // RIGHT
  struct Foo($NumLit)
  enum BarBaz {
      Qux($StrLit)
  }

  // WRONG
  struct foo($NumLit)
  enum bar_baz {
      Qux($StrLit)
  }
  ```

- Enum variants

  ```kiki
  // RIGHT
  enum Lorem {
      Ipsum($StrLit)
      Sit($StrLit)
      Amet($StrLit)
  }

  // WRONG
  enum Lorem {
      ipsum($StrLit)
      sit($StrLit)
      amet($StrLit)
  }
  ```

The following must start with a lowercase letter:

- Struct field names

  ```kiki
  // RIGHT
  struct Foo {
      bar: $NumLit
      _: $Comma
      qux: $NumLit
  }

  // WRONG
  struct Foo {
      Bar: $NumLit
      _: $Comma
      Qux: $NumLit
  }
  ```

- Enum variant field names

  ```kiki
  // RIGHT
  enum Lorem {
      Ipsum {
          bar: $NumLit
          _: $Comma
          qux: $NumLit
      }
  }

  // WRONG
  enum Lorem {
      Ipsum {
          Bar: $NumLit
          _: $Comma
          Qux: $NumLit
      }
  }
  ```

## Comments

You can write single-line comments with `//`.

```kiki
// This file parses the language of balanced parentheses.
// The language includes the empty string.

start Expr

enum Expr {
    Empty
    Wrap(
        _: $LParen
        Expr
        _: $RParen
    )
}

terminal Token {
    // You can write a comment on its own line.
    $LParen: () // Or, you can write a comment at the end of a line.
    $RParen: ()
}
```

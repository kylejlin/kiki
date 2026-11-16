# Azuki user guide

This guide is meant to be a (mostly) comprehensive reference.
It is not meant to be a quick read.
If you want to learn Azuki as quickly as possible,
click [here](./QUICKSTART.md).

## Table of contents

- [Prerequisites](#prerequisites)
- [Statements](#statements)
  - [Generated terminal declarations](#generated-terminal-declarations)
  - [`manual terminal` declarations](#manual-terminal-declarations)
  - [`struct` declarations](#struct-declarations)
  - [`enum` declarations](#enum-declarations)
  - [Manual nonterminal declarations](#manual-nonterminal-declarations)
  - [`start` declarations](#start-declarations)
  - [`bridge` declarations](#bridge-declarations)
- [Lifetimes](#lifetimes)
- [`Self`](#self)
- [Generated Rust](#generated-rust)
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

An Azuki file consists of multiple _statements_.
A statement can be a

- Generated terminal declaration (`terminal Name ...`)
- `manual terminal` declaration
- `struct` declaration
- `enum` declaration
- Manual nonterminal declaration (`manual Name ...`)
- `start` declaration
- `bridge` declaration

A file must have **exactly one** terminal declaration,
either `terminal` or `manual terminal`,
and **exactly one** `start` declaration
and **exactly one** `bridge` declaration.
A file can have unlimited `enum`, `struct`, and manual nonterminal declarations.
The declarations do not have to appear in a particular order.

Generated terminal declarations let Azuki generate the terminal enum.
Likewise, `struct` and `enum` declarations define generated nonterminals:
Azuki generates their Rust types.
Manual terminal and manual nonterminal declarations use types supplied by the bridge.

Most examples below show only the declarations relevant to that section.
The [quickstart](./QUICKSTART.md) contains complete projects.
When showing generated Rust, we omit parser internals and bridge import aliases
so that the relevant type definitions are easier to read.
Unless otherwise stated, the examples assume the generated parser
is in `crate::parser`.

## Generated terminal declarations

Generated terminal declarations use the `terminal` keyword.
They let you define each terminal's Rust type and generate the terminal enum.
For example, suppose we have a lexer
that spits out three kinds of terminals:

1. `StringLiteral`, with a Rust type of `String`.
2. `NumberLiteral`, with a Rust type of `isize`.
3. `Comma`, with a Rust type of `()` (the unit type).

We write:

```azuki
terminal Token {
    $StringLiteral = @StringLiteral
    $NumberLiteral = @NumberLiteral
    $Comma = @Comma
}

bridge crate::azuki_bridge
```

We also write the following Rust in `azuki_bridge.rs`:

```rs
pub type StringLiteral = String;
pub type NumberLiteral = isize;
pub type Comma = ();
```

This results in a Rust terminal enum equivalent to:

```rs
pub enum Token {
    StringLiteral(crate::azuki_bridge::StringLiteral),
    NumberLiteral(crate::azuki_bridge::NumberLiteral),
    Comma(crate::azuki_bridge::Comma),
}
```

Terminal names must begin with `$`.
The first letter after `$` must be uppercase.
Each entry has the form `$TerminalName = @TypeName`.
The name on the left identifies the terminal.
The name after `@` selects its Rust type from the bridge module.

### Sharing terminal types

What if two terminals have the same Rust type?
We can select the same bridge type for both terminals:

```azuki
terminal Token {
    $LParen = @Unit
    $RParen = @Unit
}
```

In the bridge module, we write:

```rs
pub type Unit = ();
```

Both variants store `Unit`.
The variant names are still `LParen` and `RParen`.
The terminal's name and its bridge type's name do not have to match.

### Shorthand for matching type names

If the terminal and bridge type have the same name,
you can omit `= @TypeName`.
For example, the full declaration at the beginning of this section
can be shortened to:

```azuki
terminal Token {
    $StringLiteral
    $NumberLiteral
    $Comma
}

bridge crate::azuki_bridge
```

Here, `$StringLiteral` is shorthand for `$StringLiteral = @StringLiteral`,
and similarly for the other two terminals.

### Crate-defined types

Up until now, we've only used types defined in
Rust's `core` or `std`.
Suppose we want our terminals to have types
that _we_ define.
For example, we write the following `lib.rs`:

```rs
mod azuki_bridge;
mod parser;

pub struct Ident {
    pub name: String,
    pub span: (usize, usize),
}

pub struct NumLit {
    pub val: isize,
    pub span: (usize, usize),
}

pub struct StrLit {
    pub val: String,
    pub span: (usize, usize),
}
```

We can write the following Azuki:

```azuki
terminal Token {
    $Ident = @Ident
    $NumLit = @NumLit
    $StrLit = @StrLit
}
```

In `azuki_bridge.rs`, we re-export the types:

```rs
pub use crate::{Ident, NumLit, StrLit};
```

Azuki gets these types from the bridge module.
You do not write Rust type paths directly in the grammar.

### Rust prefixes

You can write a _Rust prefix_ before a generated terminal declaration.
A Rust prefix has the form `rust"..."`,
where `...` is Rust code that gets copied verbatim
before the generated declaration.

For example:

```azuki
rust"#[derive(Clone, Copy, Debug)]"
terminal Token {
    $LParen = @Unit
    $RParen = @Unit
}
```

The above code results in Rust equivalent to:

```rs
#[derive(Clone, Copy, Debug)]
pub enum Token {
    LParen(crate::azuki_bridge::Unit),
    RParen(crate::azuki_bridge::Unit),
}
```

Observe that the outer attribute
(i.e., `#[derive(Clone, Copy, Debug)]`)
gets copied verbatim into the generated Rust code.
Unlike Kiki, you must put it inside a Rust prefix.

#### Rust string literals

A Rust string literal begins with `rust`,
followed by zero or more `#` characters and a double quote.
It ends at the first double quote followed by the same number of `#` characters.
The contents are copied verbatim, including newlines.
Backslashes do not introduce escapes.

For example, these two prefixes have the same contents:

```azuki
rust"#[derive(Debug)]"
rust#"#[derive(Debug)]"#
```

If the contents contain double quotes, use `#` delimiters:

```azuki
rust#"
#[derive(Debug)]
#[doc = "A pair of parentheses."]
"#
struct Parens(_: $LParen _: $RParen)
```

A declaration can have at most one Rust prefix.
If you want multiple attributes, put them in the same string.
Prefixes are supported before `terminal`, `struct`, and `enum` declarations.
They are forbidden before manual terminal and manual nonterminal declarations.

## `manual terminal` declarations

Use `manual terminal` when the bridge supplies the terminal enum:

```azuki
manual terminal Token = @LexToken {
    $Word = @Text
    $Comma = @Unit
}

bridge crate::azuki_bridge
```

In the bridge module, we write:

```rs
pub type Text = String;
pub type Unit = ();

#[derive(Debug)]
pub enum LexToken {
    Word(Text),
    Comma(Unit),
}
```

Azuki re-exports this enum as `Token` instead of generating a fresh enum.
The generated Rust is equivalent to:

```rs
pub use crate::azuki_bridge::LexToken as Token;
```

The body has exactly the same syntax and meaning as a `terminal` body.
Each entry still declares a terminal and selects its payload type from the bridge.
The supplied enum must have exactly the declared variants,
each with one tuple field of the declared payload type.
Even a unit payload uses a tuple variant, such as `Comma(())`, rather than `Comma`.
Rust checks that the supplied enum matches these declarations.
The enum's Rust variant order does not have to match the grammar's declaration order.

As with manual nonterminals, a matching bridge name can be omitted:
`manual terminal Token { ... }` means `manual terminal Token = @Token { ... }`.
Payload overrides and their matching-name shorthand work as usual.
The optional global lifetime list goes after the override, before `{`,
as in `manual terminal Token = @LexToken 'src { $Word = @Text 'src }`.
The [lifetime rules](#lifetimes) are the same for both terminal forms.

Rust prefixes are forbidden on manual terminal declarations,
just as they are on manual nonterminal declarations.
Put enum attributes, such as derives, on the bridge enum's Rust definition.

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

We can write the following Azuki:

```azuki
struct CommaSeparatedNumberPair {
    left: $NumberLiteral
    comma: $Comma
    right: $NumberLiteral
}

terminal Token {
    $Comma = @Unit
    $NumberLiteral = @Int
}
```

Here, the bridge module defines `pub type Int = isize;`
and `pub type Unit = ();`.

Observe how the data type definition implicitly defines the grammar.

### Using `_` to omit fields

The `comma: ()` field is rather useless in this example.
We can tell Azuki to omit that field using underscore syntax:

```azuki
struct CommaSeparatedNumberPair {
    left: $NumberLiteral
    _: $Comma
    right: $NumberLiteral
}

terminal Token {
    $Comma = @Unit
    $NumberLiteral = @Int
}
```

This results in the following Rust:

```rust
pub struct CommaSeparatedNumberPair {
    pub left: isize,
    pub right: isize,
}
```

### Tuple structs

You can also use tuple syntax, just like in Rust:

```azuki
struct CommaSeparatedNumberPair(
    $NumberLiteral
    _: $Comma
    $NumberLiteral
)

terminal Token {
    $NumberLiteral = @Int
    $Comma = @Unit
}
```

This results in

```rust
pub struct CommaSeparatedNumberPair(pub isize, pub isize);
```

Observe that you can use `_` syntax to omit tuple fields,
like we do in `_: $Comma`.

### Unit-like structs

For empty productions, you can use unit-like structs.
In practice, this is never useful.
However, here's an example:

```azuki
struct Epsilon
```

This corresponds to the following grammar:

```bnf
<Epsilon> ::=
```

Notice the RHS is empty.

The above Azuki generates the following Rust:

```rs
pub struct Epsilon;
```

### Rust prefixes before structs

You can write a Rust prefix before a struct.
You use the same syntax as when you write [a Rust prefix before a generated terminal declaration](#rust-prefixes).

Example:

```azuki
rust"#[derive(Clone, Debug)]"
struct ReturnStatement(
    _: $ReturnKw
    Expr
    _: $Semicolon
)
```

The above code results in the Rust code below:

```rs
#[derive(Clone, Debug)]
pub struct ReturnStatement(pub Expr);
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
    Many(Box<OneOrMoreNumbers>, isize),
}
```

We can write the following Azuki:

```azuki
enum OneOrMoreNumbers {
    One($NumLit)
    Many(
        BoxedNumbers
        $NumLit
    )
}

manual BoxedNumbers = @BoxedNumbers {
    @boxed_numbers(OneOrMoreNumbers)
}

terminal Token {
    $NumLit = @Int
}
```

In the bridge module, we write:

```rs
use crate::parser::non_terminals::OneOrMoreNumbers;

pub type Int = isize;
pub type BoxedNumbers = Box<OneOrMoreNumbers>;

pub fn boxed_numbers(inner: OneOrMoreNumbers) -> BoxedNumbers {
    Box::new(inner)
}
```

Observe that Azuki does not automatically insert `Box`.
If we had used `OneOrMoreNumbers` directly in the `Many` variant,
the generated Rust type would have infinite size.
The `manual BoxedNumbers = @BoxedNumbers` declaration tells Azuki how to box the value.
We explain manual nonterminal declarations in detail [below](#manual-nonterminal-declarations).

### `_` syntax

Just like with `struct` declarations, you can use `_` to omit fields.
For example, we can change the preceding enum to parse commas between numbers:

```azuki
enum OneOrMoreNumbers {
    One($NumLit)
    Many(
        BoxedNumbers
        _: $Comma
        $NumLit
    )
}

terminal Token {
    $NumLit = @Int
    $Comma = @Unit
}
```

Keep the same `manual BoxedNumbers = @BoxedNumbers` declaration and bridge code,
and add `pub type Unit = ();` to the bridge.

This results in a Rust enum equivalent to:

```rust
pub enum OneOrMoreNumbers {
    One(isize),
    Many(
        Box<OneOrMoreNumbers>,
        isize,
    ),
}
```

### Named variant fields

An enum variant can use named fields, just like in Rust:

```azuki
enum OneOrMoreNumbers {
    One { val: $NumLit }
    Many {
        left: BoxedNumbers
        right: $NumLit
    }
}
```

Once again, keep the same `manual BoxedNumbers = @BoxedNumbers` declaration,
terminal declaration, and bridge code.

This results in a Rust enum equivalent to:

```rs
pub enum OneOrMoreNumbers {
    One {
        val: isize,
    },
    Many {
        left: Box<OneOrMoreNumbers>,
        right: isize,
    },
}
```

### Unit-like variants

An enum variant can use unit-like syntax, just like in Rust:

```azuki
enum ZeroOrOneComma {
    None
    One(_: $Comma)
}
```

This results in:

```rs
pub enum ZeroOrOneComma {
    None,
    One,
}
```

### Rust prefixes before enums

You can write a Rust prefix before an enum.
You use the same syntax as when you write [a Rust prefix before a generated terminal declaration](#rust-prefixes).

Example:

```azuki
rust"#[derive(Clone, Copy, Debug)]"
enum ZeroOrOneComma {
    None
    One(_: $Comma)
}
```

The above code results in the Rust code below:

```rs
#[derive(Clone, Copy, Debug)]
pub enum ZeroOrOneComma {
    None,
    One,
}
```

## Manual nonterminal declarations

`struct` and `enum` declarations let Azuki generate Rust types for you.
What if you want to use a type that you define yourself?
For example, suppose we want a sequence of numbers
stored in a `Vec<isize>`.

We write:

```azuki
manual Numbers = @NumberList {
    @empty
    @push(Numbers $NumLit)
}

terminal Token {
    $NumLit = @Int
}
```

In the bridge module, we write:

```rs
pub type Int = isize;
pub type NumberList = Vec<Int>;

pub fn empty() -> NumberList {
    Vec::new()
}

pub fn push(mut numbers: NumberList, value: Int) -> NumberList {
    numbers.push(value);
    numbers
}
```

This corresponds to the following grammar:

```bnf
<Numbers> ::=
            | <Numbers> NumLit
```

Unlike a `struct` or `enum` declaration,
a manual nonterminal declaration does not generate a new Rust type.
`Numbers` names the grammar symbol,
and `= @NumberList` selects its Rust type from the bridge module.
The generated parser re-exports `NumberList` as `non_terminals::Numbers`.
Each variant names a bridge function to call.
The function takes the stored inputs, in order,
and returns the manual nonterminal's Rust type.

Here, `@empty` calls `empty()` with no arguments.
`@push(Numbers $NumLit)` calls `push(numbers, value)`.

Manual nonterminal inputs use tuple syntax.
You can use `_: Symbol` to parse an input without passing it to the function.
For example:

```azuki
manual ParenthesizedNumber = @Int {
    @identity(_: $LParen $NumLit _: $RParen)
}
```

In the bridge module, we write:

```rs
pub fn identity(value: Int) -> Int {
    value
}
```

Here, `= @Int` selects the bridge type `Int` for `ParenthesizedNumber`.
The generated parser re-exports `Int` as `non_terminals::ParenthesizedNumber`.

A manual nonterminal declaration must have at least one variant.
Two variants of the same manual nonterminal declaration cannot have the same symbol sequence,
even if they call different functions or skip different inputs.
Similarly, two variants of the same enum cannot have the same symbol sequence.

Azuki checks the grammar's names and symbol sequences.
Rust checks that the bridge types and functions exist,
and that their signatures match the grammar.
Manual nonterminal functions return the nonterminal's value directly;
there is no special syntax for returning a parser error from a manual nonterminal function.

### Shorthand for matching type names

If the grammar symbol and bridge type have the same name,
you can omit `= @TypeName`.
For example, suppose the bridge also defines `pub type Numbers = NumberList;`.
We can write the full declaration:

```azuki
manual Numbers = @Numbers {
    @empty
    @push(Numbers $NumLit)
}
```

Or, we can use the shorthand:

```azuki
manual Numbers {
    @empty
    @push(Numbers $NumLit)
}
```

Both declarations select the bridge type `Numbers`.

## `start` declarations

You must specify a starting symbol for the grammar.
Use a `start` declaration, like so:

```azuki
start FooBar
```

The start symbol must be a nonterminal symbol.

Note that a `start` declaration does _not_ define the referenced
symbol (i.e., `FooBar`, in this case).
You must define it separately using a `struct`, `enum`, or manual nonterminal definition,
as explained in previous sections.

## `bridge` declarations

You must specify a Rust module that supplies terminal types,
manual terminal enums, manual nonterminal types, and manual nonterminal functions.
Use a `bridge` declaration, like so:

```azuki
bridge crate::azuki_bridge
```

In `lib.rs`, declare the module:

```rs
mod azuki_bridge;
mod parser;
```

The `bridge` declaration does _not_ create the module.
You must write it yourself.
The types and functions used by the generated parser must be visible to it.
The examples use `pub` aliases, re-exports, and functions for this purpose.

A bridge path consists of names separated by `::`.
The generated parser uses this path in a Rust `use` statement.
Paths beginning with `crate::` are convenient,
since they do not depend on where the generated parser module is located.

## Lifetimes

A terminal type can borrow from the input.
For example, suppose the bridge module contains:

```rs
pub type Text<'src> = &'src str;
```

We write:

```azuki
terminal Token 'src {
    $Word = @Text 'src
    $Comma = @Unit
}
```

The lifetime list after `Token` declares the lifetimes available in the grammar.
The lifetime list after a terminal variant specifies the arguments
for that terminal's bridge type.
Here, `$Word` has Rust type `Text<'src>`,
and `$Comma` has Rust type `Unit`, with no lifetime arguments.
The bridge also needs `pub type Unit = ();`.

A manual nonterminal declaration can specify lifetime arguments too:

```azuki
manual Words = @Words 'src {
    @empty
    @push(Words $Word)
}
```

In the bridge module, we write:

```rs
pub type Words<'src> = Vec<Text<'src>>;

pub fn empty<'src>() -> Words<'src> {
    Vec::new()
}

pub fn push<'src>(mut words: Words<'src>, word: Text<'src>) -> Words<'src> {
    words.push(word);
    words
}
```

The bridge type's name goes after `@`, followed by its lifetime arguments.
For example, we can name the grammar symbol `WordList`
while still selecting the bridge type `Words`:

```azuki
manual WordList = @Words 'src {
    @empty
    @push(WordList $Word)
}
```

You do not write lifetime arguments on grammar symbol occurrences.
For example, use `Words`, not `Words 'src`, in a struct field:

```azuki
struct Document {
    words: Words
}
```

Azuki infers the lifetimes of generated structs and enums
from the fields they store.
The above declaration results in a Rust struct equivalent to:

```rs
pub struct Document<'src> {
    pub words: crate::azuki_bridge::Words<'src>,
}
```

Skipped fields do not contribute lifetimes to a generated struct or enum.
A manual nonterminal type uses exactly the lifetime arguments you declare,
regardless of its function inputs.
For example, a manual nonterminal function may take borrowed text
and return an owned `String`.
In that case, the manual nonterminal type does not need `'src`.

Every lifetime used by a terminal variant or manual nonterminal declaration
must be declared in the terminal declaration's global lifetime list
(whether it uses `terminal` or `manual terminal`).
A lifetime list cannot contain duplicates.
If there are multiple lifetimes, separate them with whitespace:

```azuki
terminal Token 'src 'other {
    $Pair = @Pair 'other 'src
}
```

For terminal payload types and manual nonterminal bridge types,
lifetime arguments stay in the order you write them.
For generated structs and enums, inferred lifetimes appear in the order
of the global lifetime list.
The terminal enum includes only lifetimes used by its terminal variants.
For `manual terminal`, the bridge enum must accept those same lifetime arguments,
in global declaration order; unused global lifetimes are omitted.

The matching-name shorthand also works with lifetimes:
`manual Words = @Words 'src { ... }` can be shortened to `manual Words 'src { ... }`.
Likewise, `$Text = @Text 'src` can be shortened to `$Text 'src`.

## `Self`

We can refer to a nonterminal by its name inside its own declaration.
For example, here is the full form of the number-list example:

```azuki
manual Numbers = @NumberList {
    @empty
    @push(Numbers $NumLit)
}
```

Inside a `struct`, `enum`, or manual nonterminal declaration,
we can use `Self` as shorthand for the enclosing nonterminal's name:

```azuki
manual Numbers = @NumberList {
    @empty
    @push(Self $NumLit)
}
```

The two declarations have the same meaning.
`Self` can appear wherever a grammar symbol can appear in a fieldset or manual nonterminal input.
Inside an enum variant, `Self` means the enum, not the variant.
Using `Self` does not insert a `Box`.

## Generated Rust

Suppose Azuki generates `parser.rs`, and `lib.rs` contains `mod parser;`.
The terminal enum and `parse` function are in `parser`.
The nonterminal types are in `parser::non_terminals`.
For example:

```rs
use crate::parser::non_terminals::Expr;
use crate::parser::{Token, parse};
```

Generated structs and their stored fields are public.
Generated enums are public.
A field's Rust type is the type of the referenced grammar symbol.
Azuki does not automatically insert `Box` around nonterminals.
If recursive Rust types need indirection, provide it through a manual nonterminal,
as in the [enum example](#enum-declarations).

The `parse` function accepts any `IntoIterator` of terminal enum values.
For a grammar with `start Expr` and no lifetimes,
its signature is equivalent to:

```rs
pub fn parse<S>(src: S) -> Result<Expr, Option<Token>>
where
    S: IntoIterator<Item = Token>,
```

It returns `Ok(value)` if parsing succeeds,
`Err(Some(token))` for an unexpected token,
or `Err(None)` for unexpected end of input.
The generated parser does not require Azuki as a runtime dependency.

To generate Rust in a build script, call
`azuki::convenience::azuki_to_rust(FileSource::new(&source), file_name)`.
Import `FileSource` from `azuki::source_text`.
On success, the generated code is in the returned value's `raw` field.
On failure, the returned error displays a human-readable diagnostic.
The [quickstart](./QUICKSTART.md) includes a complete build script.

## Identifier names

Identifier names begin with an ASCII letter,
followed by zero or more ASCII letters, digits, or underscores.
Unlike Kiki, an identifier cannot begin with an underscore.
The bare `_` is reserved for skipped fields.
You cannot name an identifier `terminal`, `start`, `struct`, `enum`,
`manual`, `bridge`, or `Self`.

You cannot name an identifier any word that is reserved in Rust.
This is not enforced by Azuki.
That is, if you name an identifier using a reserved word
(e.g., `impl`), Azuki will _not_ produce an error.
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
- `manual`
- `bridge`
- `Self`
- `_hidden`
- `impl`
- `mod`
- `123`
- `!#$@`

## Terminal names

A terminal name is valid iff it is of the form `$<ident_name>`,
where `<ident_name>` is a valid identifier name beginning with an uppercase letter.

Good:

- `$Comma`
- `$LParen`
- `$NumberLiteral`

Bad:

- `$start`
- `$123`
- `$$`

## Capitalization

The following must start with an uppercase letter
(after `$` or `@`, where applicable):

- Terminal names

  ```azuki
  // RIGHT
  terminal Token {
      $NumLit = @Int
  }

  // WRONG
  terminal Token {
      $num_lit = @Int
  }
  ```

- Terminal enum names, nonterminal names, and bridge type names

  ```azuki
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

  ```azuki
  // RIGHT
  enum Lorem {
      Ipsum($StrLit)
      Sit($NumLit)
      Amet(_: $Comma)
  }

  // WRONG
  enum Lorem {
      ipsum($StrLit)
      sit($NumLit)
      amet(_: $Comma)
  }
  ```

The following must start with a lowercase letter:

- Struct field names

  ```azuki
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

  ```azuki
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

Manual nonterminal function names must have the form `@lower_name`.
Lifetime names must have the form `'lower_name`.
The names can contain uppercase letters after their first letter.

## Comments

You can write single-line comments with `//`.

```azuki
// This file parses the language of balanced parentheses.
// The language includes the empty string.

start Expr

enum Expr {
    Empty
    Wrap(
        _: $LParen
        BoxedExpr
        _: $RParen
    )
}

manual BoxedExpr = @BoxedExpr {
    @boxed(Expr)
}

terminal Token {
    // You can write a comment on its own line.
    $LParen = @Unit // Or, you can write a comment at the end of a line.
    $RParen = @Unit
}

bridge crate::azuki_bridge
```

The bridge supplies `Unit`, `BoxedExpr`, and `boxed`,
as in the [quickstart](./QUICKSTART.md).

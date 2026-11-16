# Azuki user guide

Azuki generates Rust parsers from type definitions.
The type definitions describe the grammar too.
The parser reads tokens from a lexer you provide.

This guide assumes you know Rust and context-free grammars.
Read the [quickstart](./QUICKSTART.md) for project setup.
The examples below show only the declarations under discussion.

## Table of contents

- [`terminal` declarations](#terminal-declarations)
- [`struct` declarations](#struct-declarations)
- [`enum` declarations](#enum-declarations)
- [The start symbol](#the-start-symbol)
- [Using the generated parser](#using-the-generated-parser)
- [Rust prefixes](#rust-prefixes)
- [Manual nonterminal declarations](#manual-nonterminal-declarations)
  - [`Self`](#self)
  - [Recursive types](#recursive-types)
- [`manual terminal` declarations](#manual-terminal-declarations)
- [Lifetimes](#lifetimes)
- [Grammar files](#grammar-files)
- [Names](#names)
- [Comments](#comments)

## `terminal` declarations

Suppose our lexer produces three kinds of tokens:

- A number literal, with a value of type `isize`.
- A string literal, with a value of type `String`.
- A comma, with a value of type `()`.

We expose these types in a Rust module called `azuki_bridge`:

```rs
pub type Int = isize;
pub type Text = String;
pub type Unit = ();
```

Then we write the following Azuki:

```azuki
terminal Token {
    $NumberLiteral = @Int
    $StringLiteral = @Text
    $Comma = @Unit
}

bridge crate::azuki_bridge
```

Azuki generates a Rust enum equivalent to:

```rs
pub enum Token {
    NumberLiteral(isize),
    StringLiteral(String),
    Comma(()),
}
```

Each terminal has a name and a Rust type.
The name begins with `$`.
The type comes from the bridge module.
For example, `@Int` refers to the bridge's `Int` type.
Here, we have expanded the aliases in the generated Rust.

The `bridge` declaration names the module.
You must write that module yourself.
We use it for Rust functions too, as explained later.

### Other Rust types

You can use your own types in the same way.
For example, suppose your crate defines an `Ident` type.
Re-export it from the bridge:

```rs
pub use crate::Ident;
```

Then use `$Ident = @Ident` in the terminal declaration.
You do not write Rust type paths in the grammar.

Two terminals can share a type.
For example, both `$LParen` and `$RParen` can use `@Unit`.

If a terminal and its bridge type share a name, omit `= @TypeName`.
For example, `$Ident` means `$Ident = @Ident`.

## `struct` declarations

Suppose we want to parse a pair of numbers separated by a comma:

```bnf
<NumberPair> ::= NumberLiteral Comma NumberLiteral
```

We want to store the result in this Rust type:

```rs
pub struct NumberPair {
    pub left: isize,
    pub comma: (),
    pub right: isize,
}
```

We write:

```azuki
struct NumberPair {
    left: $NumberLiteral
    comma: $Comma
    right: $NumberLiteral
}
```

The fields describe what to parse, in order.
They also describe what to store.
We have defined the grammar and the Rust type together.

A field can refer to another nonterminal too.
Use its name without `$`, as in `pair: NumberPair`.
The field stores a value of that nonterminal's type.

### Omitting fields

The comma separates the numbers.
Its value tells us nothing.
Use `_` to leave it out of the Rust type:

```azuki
struct NumberPair {
    left: $NumberLiteral
    _: $Comma
    right: $NumberLiteral
}
```

This generates:

```rs
pub struct NumberPair {
    pub left: isize,
    pub right: isize,
}
```

The parser still expects the comma.
It just discards its value.

### Tuple and unit-like structs

You can use tuple syntax, just like in Rust:

```azuki
struct NumberPair(
    $NumberLiteral
    _: $Comma
    $NumberLiteral
)
```

This generates:

```rs
pub struct NumberPair(pub isize, pub isize);
```

For an empty production, use a unit-like struct:

```azuki
struct Epsilon
```

This parses an empty sequence and generates `pub struct Epsilon;`.
A struct with all fields skipped also becomes unit-like.

## `enum` declarations

So far, each nonterminal has had one production rule.
Suppose we want to accept either a number or a string.
We want a separate Rust variant for each case:

```rs
pub enum Literal {
    Number(isize),
    String(String),
}
```

We write:

```azuki
enum Literal {
    Number($NumberLiteral)
    String($StringLiteral)
}
```

Each variant defines a rule.
`Number` parses a number literal.
`String` parses a string literal.

Variants can have named fields, just like structs:

```azuki
enum Literal {
    Number {
        value: $NumberLiteral
    }
    String($StringLiteral)
}
```

Use `_` to omit fields here too.

### Empty rules and skipped fields

Suppose a comma is optional:

```azuki
enum OptionalComma {
    None
    Some(_: $Comma)
}
```

`None` parses an empty sequence.
`Some` parses a comma and discards its value.
Both Rust variants are unit-like:

```rs
pub enum OptionalComma {
    None,
    Some,
}
```

## The start symbol

Choose the nonterminal you want the parser to return:

```azuki
start NumberPair
```

This selects `NumberPair` as the start symbol.
You must define it separately.
Use a `struct`, `enum`, or manual nonterminal declaration.

## Using the generated parser

Suppose Azuki writes the parser to `parser.rs`.
Declare both Rust modules in `lib.rs`:

```rs
mod azuki_bridge;
mod parser;
```

The terminal enum and `parse` function are in `parser`.
Nonterminal types are in `parser::non_terminals`.
Generated types and struct fields are public.

For the number-pair grammar, we can write:

```rs
use crate::parser::non_terminals::NumberPair;
use crate::parser::{Token, parse};

let result: Result<NumberPair, Option<Token>> = parse([
    Token::NumberLiteral(12),
    Token::Comma(()),
    Token::NumberLiteral(34),
]);
```

`parse` accepts any `IntoIterator` of terminal values.
It returns `Ok(pair)` on success.
The pair stores `12` and `34` in its two fields.

An unexpected token produces `Err(Some(token))`.
Unexpected end of input produces `Err(None)`.
The generated parser has no runtime dependency on Azuki.

## Rust prefixes

Suppose we want to print a parsed pair with `Debug`.
We need a derive on the generated struct.
Put a Rust prefix before the declaration:

```azuki
rust"#[derive(Debug)]"
struct NumberPair {
    left: $NumberLiteral
    _: $Comma
    right: $NumberLiteral
}
```

Azuki copies the prefix before the Rust declaration.
Here, it adds `#[derive(Debug)]` to `NumberPair`.
Prefixes work before `terminal`, `struct`, and `enum` declarations.
Each declaration can have one prefix.
Put multiple attributes in the same string.

### Quotes inside a prefix

Use `#` delimiters when the Rust code contains quotes:

```azuki
rust#"#[doc = "A pair of numbers."]"#
struct NumberPair($NumberLiteral _: $Comma $NumberLiteral)
```

You can use any number of `#` characters.
The closing quote must have the same number after it.
The string ends at the first such closing delimiter.
Newlines are allowed.
Backslashes do not introduce escapes.

## Manual nonterminal declarations

So far, Azuki has generated our nonterminal types.
Suppose we want to parse a sequence of numbers.
We want to store them in a `Vec<isize>`.
A manual nonterminal lets us choose that type and build its values ourselves.

First, add the type and two functions to the bridge:

```rs
pub type NumberList = Vec<Int>;

pub fn empty() -> NumberList {
    Vec::new()
}

pub fn push(mut numbers: NumberList, value: Int) -> NumberList {
    numbers.push(value);
    numbers
}
```

Then write:

```azuki
manual Numbers = @NumberList {
    @empty
    @push(Numbers $NumberLiteral)
}
```

`Numbers` is the grammar symbol.
`NumberList` is its Rust type.
Azuki re-exports that type as `non_terminals::Numbers`.

Each rule names a bridge function.
The first rule parses an empty sequence and calls `empty()`.
The second parses a list followed by a number.
It calls `push(numbers, value)` to extend the list.

The function receives its inputs in grammar order.
Use `_: Symbol` to skip an input here too.
Skipped inputs are parsed but are not passed to the function.

Rust checks the function's arguments and return type.
The function must return the nonterminal's value directly.
There is no special syntax for returning a parser error.

If the bridge type is also called `Numbers`, omit `= @NumberList`.
This follows the same rule as terminal payloads.

### `Self`

Inside a `struct`, `enum`, or manual nonterminal, `Self` names the enclosing nonterminal.
For example, `@push(Self $NumberLiteral)` means `@push(Numbers $NumberLiteral)` above.

### Recursive types

Generated fields store nonterminal values directly.
Azuki does not insert `Box` for you.
For recursive Rust types, you can add a manual nonterminal that boxes its input.

For example, suppose we want this Rust type:

```rs
pub enum Expr {
    Empty,
    Wrapped(Box<Expr>),
}
```

Add the following to the bridge:

```rs
use crate::parser::non_terminals::Expr;

pub type BoxedExpr = Box<Expr>;

pub fn boxed(inner: Expr) -> BoxedExpr {
    Box::new(inner)
}
```

Declare `$LParen` and `$RParen` with type `@Unit`.
Then write:

```azuki
enum Expr {
    Empty
    Wrapped(_: $LParen BoxedExpr _: $RParen)
}

manual BoxedExpr {
    @boxed(Expr)
}
```

The parser calls `boxed` after parsing the inner `Expr`.
The [quickstart](./QUICKSTART.md) includes the full grammar and project setup.

## `manual terminal` declarations

Your lexer may already return a suitable token enum.
You can use that enum directly.
For example, suppose the bridge supplies:

```rs
pub enum LexToken {
    NumberLiteral(Int),
    StringLiteral(Text),
    Comma(Unit),
}
```

We write:

```azuki
manual terminal Token = @LexToken {
    $NumberLiteral = @Int
    $StringLiteral = @Text
    $Comma = @Unit
}
```

Azuki re-exports `LexToken` as `Token`.
The body follows the same rules as a generated terminal declaration.
If the bridge enum is also named `Token`, omit `= @LexToken`.

The supplied enum must have exactly the declared variants.
Their order does not matter.
Each variant must have one tuple field of the declared type.
Even `Comma` needs a field, with type `()`.
Rust checks that the enum matches these declarations.

Put attributes on the Rust enum itself.
Manual declarations cannot have Rust prefixes.

## Lifetimes

So far, our string literals have owned their text.
Suppose we want to borrow that text from the input.
Change the bridge alias to:

```rs
pub type Text<'src> = &'src str;
```

Then write:

```azuki
terminal Token 'src {
    $NumberLiteral = @Int
    $StringLiteral = @Text 'src
    $Comma = @Unit
}
```

The list after `Token` is the grammar's global lifetime list.
After `@Text`, `'src` is an argument to the bridge type.
Here, string literals have type `Text<'src>`.
The other payload types have no lifetime arguments.

### Generated types

Now put a string literal in a struct:

```azuki
struct Message {
    text: $StringLiteral
}
```

Azuki infers the struct's lifetimes from its stored fields.
This generates a type equivalent to:

```rs
pub struct Message<'src> {
    pub text: &'src str,
}
```

Azuki also infers `'src` for types that store a `Message`.
Write `message: Message` in the grammar.
Leave lifetime arguments off symbol references.
Skipped fields do not contribute lifetimes.

### Manual types

Suppose we want to return the borrowed string itself.
A manual nonterminal can do that:

```azuki
manual BorrowedText = @Text 'src {
    @identity($StringLiteral)
}
```

The bridge function simply returns its input:

```rs
pub fn identity<'src>(text: Text<'src>) -> Text<'src> {
    text
}
```

List the lifetime arguments after the bridge type.
These arguments determine the manual type's lifetimes.
Azuki does not infer them from the function inputs.
A function can accept borrowed text and return an owned value.
In that case, the manual type does not need `'src`.

Matching-name shorthand works with lifetimes too.
`$Text 'src` means `$Text = @Text 'src`.
Likewise, `manual Text 'src { ... }` selects the bridge type `Text<'src>`.

### Multiple lifetimes

A bridge type may need more than one lifetime.
Declare all lifetimes in the terminal declaration.
Separate them with whitespace.
For example:

```azuki
terminal Token 'left 'right {
    $Pair = @Pair 'right 'left
}
```

Type arguments keep the order you write them.
Here, the bridge type receives `'right` before `'left`.
Inferred lifetimes on generated structs and enums follow global declaration order.
The terminal enum follows global order too.
It includes only lifetimes used by its payloads.

Each lifetime list must be free of duplicates.
Every lifetime argument must appear in the global list.

### Manual terminal enums

For `manual terminal`, put the global list after the optional type override:

```azuki
manual terminal Token = @LexToken 'src {
    $NumberLiteral = @Int
    $StringLiteral = @Text 'src
    $Comma = @Unit
}
```

The bridge enum must accept the payload lifetimes in global order.
Unused global lifetimes are omitted.

## Grammar files

A file must contain exactly one of each:

- A terminal declaration, either `terminal` or `manual terminal`.
- A `start` declaration.
- A `bridge` declaration.

You can add any number of `struct`, `enum`, and manual nonterminal declarations.
Declarations can appear in any order.

A bridge path consists of names separated by `::`.
Azuki uses it in a Rust `use` statement.
A path starting with `crate::` works from any parser module.
The bridge's types and functions must be visible to the parser.

Every referenced symbol must be defined.
Terminal enum names and nonterminal names must be distinct.
Variant names must be distinct within each enum.
Field names must be distinct within each named fieldset.
You can repeat `_`.

An enum or manual nonterminal must have at least one variant.
Its variants must have different symbol sequences.
Changing field names or skipping fields does not change the sequence.
The grammar must be LALR(1).

## Names

Identifiers start with an ASCII letter.
The remaining characters can be ASCII letters, digits, or underscores.
An identifier cannot start with `_`.
The bare `_` is reserved for skipped fields.

The first letter must be uppercase in these names:

- Terminal enum names: `Token`.
- Nonterminal names: `NumberPair`.
- Enum variant names: `Number`.
- Terminal names: `$NumberLiteral`.
- Bridge type names: `@Int`.

Field names start with a lowercase letter, as in `left`.
Bridge function names do too, as in `@push`.
Lifetime names use the same convention, with a leading quote: `'src`.
Later letters can have either case.

Azuki reserves `terminal`, `start`, `struct`, `enum`, `manual`, `bridge`, and `Self`.
Avoid Rust reserved words too.
Azuki does not reject all of them.
They can make the generated Rust fail to compile.

## Comments

Use `//` for a single-line comment:

```azuki
// A pair of numbers separated by a comma.
struct NumberPair {
    left: $NumberLiteral
    _: $Comma // Parse the comma without storing it.
    right: $NumberLiteral
}
```

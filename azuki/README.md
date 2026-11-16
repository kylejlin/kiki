# Azuki

[![crates.io](https://img.shields.io/crates/v/azuki.svg)](https://crates.io/crates/azuki)

Azuki is a minimalist parser generator for Rust.

Azuki is an experimental successor to [Kiki](https://crates.io/crates/kiki).
It is mostly a generalization of Kiki, but it is not fully backwards compatible
(e.g., there is no auto-`Box`-ing).

## Table of contents

- [How to use Azuki](#how-to-use-azuki)
- [Why use Azuki](#why-use-azuki)
- [Azuki's limitations](#azukis-limitations)
- [Example](#example)
- [Guide](#guide)
- [Contributing](#contributing)

## How to use Azuki?

Read the [quickstart](./QUICKSTART.md).

## Why use Azuki?

- **Easy to learn.**
  If you've used other parser generators before (e.g., Bison/yacc),
  you can [learn Azuki in under 10 minutes](./QUICKSTART.md).
- **Easy to write.**
  Tools like Bison or lalrpop force you to write
  a grammar, semantic actions, and syntax tree type definitions.
  Azuki lets you write type definitions for most of your grammar.
  Azuki infers the grammar and semantic actions from the
  type definitions.
  When you need more control, manual nonterminal declarations let you
  use your own Rust types and functions.
- **Easy to read.**
  Azuki has a minimalist syntax.
  This makes it easy to learn, and easy to read.

## Azuki's limitations

- Azuki only supports LALR(1) grammars.
- Azuki parses token sequences, not strings.
  - In other words, you must provide your own lexer.
    You can either implement the lexer by hand,
    or use a lexer generator (e.g., [logos](https://crates.io/crates/logos)).

## Example

In this section, we build a toy parser that
recognizes arithmetic expressions.
For example:

- `42`
- `42 + 53`
- `29 + (893 * 7)`

For simplicity, this language does not have operator precedence.
Instead, you must use parentheses (e.g., `29 + (893 * 7)`).

Let's compare how we build the parser using Bison and Azuki.

### With Bison

Suppose Bison hypothetically supported Rust (instead of only C/C++).
Then you might write:

```yacc
%{
    enum Expr {
        Num(i32),
        Op {
            left: Box<Expr>,
            kind: OpKind,
            right: Box<Expr>,
        },
    }

    enum OpKind {
        Add,
        Sub,
        Mul,
        Div,
    }
}

%token <i32> NUM

// Other than NUM, the rest of the tokens
// only have one possible value each.
// So, we set their type to the unit type (`()`).
%token <()> PLUS
%token <()> MINUS
%token <()> STAR
%token <()> SLASH
%token <()> LPAREN
%token <()> RPAREN

%start expr

%%

expr
    : term
        {
            $$ = $1;
        }
    | term PLUS term
        {
            $$ = Expr::Op {
                left: Box::new($1),
                kind: OpKind::Add,
                right: Box::new($3),
            };
        }
    | term MINUS term
        {
            $$ = Expr::Op {
                left: Box::new($1),
                kind: OpKind::Sub,
                right: Box::new($3),
            };
        }
    | term STAR term
        {
            $$ = Expr::Op {
                left: Box::new($1),
                kind: OpKind::Mul,
                right: Box::new($3),
            };
        }
    | term SLASH term
        {
            $$ = Expr::Op {
                left: Box::new($1),
                kind: OpKind::Div,
                right: Box::new($3),
            };
        }
;

term
    : NUM
        {
            $$ = Expr::Num($1);
        }
    | LPAREN expr RPAREN
        {
            $$ = $2;
        }
;
```

Observe that there are _three_ things you must write:

1. The grammar (i.e., `expr : term ...;` and `term : NUM ...;`).
2. The semantic actions (e.g., `$$ = Expr::Op {...};`).
3. The syntax tree type definitions (i.e., `enum Expr {...}` and `enum OpKind {...}`).

### With Azuki

In Azuki, you write:

```azuki
terminal Token {
    $Num = @Int
    $Plus = @Unit
    $Minus = @Unit
    $Star = @Unit
    $Slash = @Unit
    $LParen = @Unit
    $RParen = @Unit
}

start Expr

enum Expr {
    Term(Term)
    Op {
        left: Term
        kind: OpKind
        right: Term
    }
}

enum OpKind {
    Add(_: $Plus)
    Sub(_: $Minus)
    Mul(_: $Star)
    Div(_: $Slash)
}

enum Term {
    Num($Num)
    Parenthesized(
        _: $LParen
        BoxedExpr
        _: $RParen
    )
}

manual BoxedExpr = @BoxedExpr {
    @boxed(Expr)
}

bridge crate::azuki_bridge
```

You also write the following Rust in `azuki_bridge.rs`:

```rs
use crate::parser::non_terminals::Expr;

pub type Int = i32;
pub type Unit = ();
pub type BoxedExpr = Box<Expr>;

pub fn boxed(inner: Expr) -> BoxedExpr {
    Box::new(inner)
}
```

Here, we assume the generated parser is in a module called `parser`,
and `lib.rs` declares `mod azuki_bridge;`.
The [quickstart](./QUICKSTART.md) shows how to set up these files.

Observe that most of the grammar and semantic actions
are inferred from the syntax tree type definitions.
The `manual BoxedExpr = @BoxedExpr` declaration is the exception:
it tells Azuki to call our `boxed` function when it parses an `Expr`.
This lets us choose where to use `Box`.

## Guide

You can read the user guide [here](./USER_GUIDE.md).
The guide explains Azuki in detail.
It is _not_ a fast read.

If your goal is to grok Azuki as quickly as possible,
click [here](./QUICKSTART.md).

## Contributing

Contributions are welcome.
Simply open a [new issue](https://github.com/kylejlin/azuki/issues/new) or [pull request](https://github.com/kylejlin/azuki/pulls), and I'll take a look.
All forms of contribution (e.g., bugfixes, tests, documentation, typo correction) are helpful.

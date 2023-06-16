# Kiki

[![crates.io](https://img.shields.io/crates/v/kiki.svg)](https://crates.io/crates/kiki)

Kiki is a minimalist parser generator for Rust.

## Table of contents

- [Why use Kiki](#why-use-kiki)
- [Kiki's limitation](#kikis-limitations)
- [Example](#example)
- [Guide](#guide)
- [Contributing](#contributing)

## Why use Kiki?

- **Easy to learn.**
  If you've used other parser generators before (e.g., Bison/yacc),
  you can learn Kiki in under 10 minutes.
- **Easy to write.**
  Tools like Bison or lalrpop force you to write
  a grammar, semantic actions, and syntax tree type definitions.
  Kiki lets you write only the type definitions.
  Kiki infers the grammar and semantic actions from the
  type definitions.
- **Easy to read.**
  Kiki has a minimalist syntax.
  This makes it easy to learn, and easy to read.

## Kiki's limitations

- Kiki only supports LALR(1) grammars.
- Kiki parses token sequences, not strings.
  - In other words, you must provide your own lexer.
    You can either implement the lexer by hand,
    or use a lexer generator (e.g., [logos](https://crates.io/crates/logos)).

## Example

In this section, we build a toy parser that
recognizes the arithmetic expressions.
For example:

- `42`
- `42 + 53`
- `29 + (893 * 7)`

For simplicity, this language does not have operator precedence.
Instead, you must use parentheses (e.g., `29 + (893 * 7)`).

Let's compare how we build the parser using Bison and Kiki.

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
        Cons,
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
                kind: OpKind::Cons,
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

### With Kiki

In Kiki, you write:

```kiki
terminal Token {
    $Num: i32
    $Plus: ()
    $Minus: ()
    $Star: ()
    $Slash: ()
    $LParen: ()
    $RParen: ()
}

start Expr

enum Expr {
    Term(Term)
    Op {
        left: Expr
        kind: OpKind
        right: Expr
    }
}

enum OpKind {
    Add(_: $Plus)
    Sub(_: $Minus)
    Cons(_: $Star)
    Div(_: $Div)
}

enum Term {
    Num($Num)
    Parenthesized(
        _: $LParen
        Expr
        _: $RParen
    )
}
```

Observe this code is much simpler and shorter.
Instead of having to write the
grammar, semantic actions, and syntax tree type definition,
**you only need to write the syntax tree type definition.**
Kiki infers the grammar and semantic action from the type definition.

## Guide

You can read the user guide [here](./USER_GUIDE.md).
The example from the previous section omits many details.
The user guide explains these details.

## Contributing

Contributions are welcome.
Simply open a [new issue](https://github.com/kylejlin/kiki/issues/new) or pull request, and I'll take a look.
All forms of contribution (e.g., bugfixes, tests, documentation, typo correction) are helpful.

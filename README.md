# Kiki

Kiki is a minimalist parser generator for Rust.

## Why use Kiki?

- **Easy to learn.**
  If you've used other parser generators before (e.g., Bison/yacc),
  you can learn Kiki in _under 10 minutes_.
- **Easy to write.**
  Tools like Bison or lalrpop often force you to write boilerplate.
  Specifically, you must code the data structures
  (i.e., the abstract the syntax tree), in addition to coding the grammar.
  Kiki is based on _concrete_ syntax trees, which eliminates
  the need for this boilerplate.
- **Easy to read.**
  Kiki's has a minimalist syntax.
  This makes it easy to learn, and easy to read.

## Why _not_ use Kiki?

There are a few cases where you probably should _not_ use Kiki:

- **Your grammar is not LALR(1).**
  Kiki only supports LALR(1) grammars.
  While this is perfectly fine for 90% of use cases,
  it's sometimes insufficient for some particularly complex
  or poorly written grammars.
- **You need semantic actions.**
  Kiki does not support custom semantic actions.
- **You want to generate the AST directly**.

  Often, you ultimately want an _abstract_ syntax tree (AST)--a tree
  that has a different structure than the grammar.
  Kiki's parsers produce a _concrete_ syntax tree (CST)--a tree
  that directly corresponds to the grammar.

  This means you must manually implement a CST-to-AST conversion algorithm
  yourself.
  If you are okay with this, then you can still use Kiki.

  However, if you want to generate the AST in one pass
  (without the intermediate CST), you should use lalrpop (or another more complex tool).

## Comparison with Bison

Let's build a toy parser that
recognizes the arithmetic expressions.
For example:

- `42`
- `42 + 53`
- `29 + (893 * 7)`

For simplicity, this language does not have operator precedence.
That is, you must explicitly use parentheses (e.g., `29 + (893 * 7)`).

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
        Mul,
        Div,
    }
}

%token <i32> NUM
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

Observe that you must write separate code for the...

- syntax tree data structure (i.e., `enum Expr {...}`)
- and the grammar (i.e., `expr : term ...;`, `term : NUM ...;`).

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
    Mul(_: $Star)
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

Observe that you define the syntax tree data structure
and the grammar at the same time.

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

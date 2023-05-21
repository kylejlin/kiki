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
- **You want an _abstract_ syntax tree (AST)**.
  Kiki does One Thing Well™: parsing.

  Syntax analysis often requires at least 2 jobs.
  The first is to parse the input.
  The second is to convert the parse tree (i.e., the concrete syntax tree)
  into a more developer-friendly form (i.e., the abstract syntax tree).

  Many tools, like Bison, handle both of these jobs in one pass.
  This sometimes leads to better performance.
  However, this comes at the cost of greater tool complexity,
  and weaker separation of concerns.

  Kiki _only_ produces the parse tree.
  If the developer wants to transform that tree
  into another form (i.e., an AST), they must
  implement that code themselves.

- **You want a built-in lexer.**
  As stated above, Kiki does One Thing Well™: parsing.

## Quasi-tutorial

The fastest way to learn is by example.
In this section, we translate
a Bison file into Kiki.
We don't give detailed explanations,
which is why it's a _quasi_ tutorial.
We will save the explanation for future sections.

### A toy parser

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

Observe this code is much simpler.
The same code defines both the data structures and the grammar.
This powerful simplification is only possible because
Kiki parsers produce _concrete_ syntax trees.

This limitation is a double-edged sword.
On one hand, it greatly simplifies the code.
On the other hand, it prevents you from having a
different tree structure than the grammar.

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

`kiki.kiki`:

```kiki
start File

struct File {
    items: OptItems
}

enum OptItems {
    Nil
    Cons(
        OptItems
        Item
    )
}

enum Item {
    Start(
        _: $StartKw
        $Ident
    )
    Struct(Struct)
    Enum(Enum)
    Terminal(Terminal)
}

struct Struct {
    _: $StructKw
    name: $Ident
    fieldset: Fieldset
}

struct Enum {
    _: $EnumKw
    name: $Ident
    _: $LCurly
    variants: OptEnumVariants
    _: $RCurly
}

struct Terminal {
    _: $TerminalKw
    name: $Ident
    _: $LCurly
    variants: OptTerminalVariants
    _: $RCurly
}

enum Fieldset {
    Empty
    Named(NamedFieldset)
    Tuple(TupleFieldset)
}

struct NamedFieldset {
    _: $LCurly
    fields: NamedFields
    _: $RCurly
}

enum NamedFields {
    One(NamedField)
    Mul {
        left: NamedFields
        right: NamedField
    }
}

struct NamedField {
    name: IdentOrUnderscore
    _: $Colon
    symbol: IdentOrTerminalIdent
}

struct TupleFieldset {
    _: $LParen
    fields: TupleFields
    _: $RParen
}

enum TupleFields {
    One(TupleField)
    Cons {
        left: TupleFields
        right: TupleField
    }
}

enum TupleField {
    Used(IdentOrTerminalIdent)
    Skipped(
        _: $Underscore
        _: $Colon
        IdentOrTerminalIdent
    )
}

enum OptEnumVariants {
    Nil
    Cons {
        left: OptEnumVariants
        right: EnumVariant
    }
}

struct EnumVariant {
    name: $Ident
    fields: Fieldset
}

enum OptTerminalVariants {
    Nil
    Cons {
        left: OptTerminalVariants
        right: TerminalVariant
    }
}

struct TerminalVariant {
    name: $TerminalIdent
    _: $Colon
    type_: Type
}

enum Type {
    Unit(
        _: $LParen
        _: $RParen
    )
    Path(Path)
    Complex(ComplexType)
}

enum Path {
    One($Ident)
    Cons {
        left: Path
        _: $DoubleColon
        right: $Ident
    }
}

struct ComplexType {
    callee: Type
    _: $LAngle
    Types
    _: $RAngle
}

enum CommaSeparatedTypes {
    One(Type)
    Cons {
        left: CommaSeparatedTypes
        _: $Comma
        right: Type
    }
}

enum IdentOrUnderscore {
    Ident($Ident)
    Underscore($Underscore)
}

enum IdentOrTerminalIdent {
    Ident($Ident)
    Terminal($TerminalIdent)
}

terminal Token {
    $Underscore: ()
    $Ident: String // Does not start with "$"
    $TerminalIdent: String // Starts with "$"

    $StartKw: ()
    $StructKw: ()
    $EnumKw: ()
    $TerminalKw: ()

    $Colon: ()
    $DoubleColon: ()
    $Comma: ()

    $LParen: ()
    $RParen: ()
    $LCurly: ()
    $RCurly: ()
    $LAngle: ()
    $RAngle: ()
}
```

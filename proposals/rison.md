# Proposal: Rison

Rison is a Rust parody of Bison.
It is unaffiliated with Bison's maintainers.

> This is really a separate project idea,
> but until it takes off, I'll keep it inside Kiki.

```rison
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

%token Token {
    NUM(i32),
    PLUS,
    MINUS,
    STAR,
    SLASH,
    LPAREN,
    RPAREN,
}

%start expr

%%

expr: Expr =
    | term
        {
            $1
        }
    | term PLUS term
        {
            Expr::Op {
                left: Box::new($1),
                kind: OpKind::Add,
                right: Box::new($3),
            }
        }
    | term MINUS term
        {
            Expr::Op {
                left: Box::new($1),
                kind: OpKind::Sub,
                right: Box::new($3),
            }
        }
    | term STAR term
        {
            Expr::Op {
                left: Box::new($1),
                kind: OpKind::Mul,
                right: Box::new($3),
            }
        }
    | term SLASH term
        {
            Expr::Op {
                left: Box::new($1),
                kind: OpKind::Div,
                right: Box::new($3),
            }
        }
;

term: Expr =
    | NUM
        {
            Expr::Num($1)
        }
    | LPAREN expr RPAREN
        {
            $2
        }
;
```

## `->` syntax

Alternatively, we could use `symbol -> Type : rhs1 | rhs2 ...`,
instead of `symbol: Type = rhs1 | rhs2 ...`.

```rison
term -> Expr
    : NUM
        {
            Expr::Num($1)
        }
    | LPAREN expr RPAREN
        {
            $2
        }
;
```

This is more consistent with Bison.
Now, the only difference is we add a `-> Type` clause.

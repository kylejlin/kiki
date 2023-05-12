# Proposal: semantic actions

```kiki
start OptNumbers

@type Vec<i32>
enum OptNumbers {
    None
        {
            vec![]
        }
    Some(Numbers)
        {
            @0
        }
}

@type Vec<i32>
enum Numbers {
    One($Number)
        {
            vec![@0]
        }
    Cons(Numbers _:$Comma $Number)
        {
            let mut out = @0;
            out.push(@2);
            out
        }
}

terminal Token {
    $Number: i32
    $Comma: ()
}
```

This proposal introduces semantic actions, similar to Bison.
The difference is we use `@0` instead of Bison's `$1`.
The is because we want to be consistent with Rust's tuple indexing convention (which is also zero-based).

## Named `@field`s

```kiki
// ...

@type Vec<i32>
enum Numbers {
    One($Number)
        {
            vec![@0]
        }
    Cons {
        left: Numbers
        _:$Comma
        right: $Number
    }
        {
            let mut out = @left;
            out.push(@right);
            out
        }
}

// ...
```

Not sure if this makes sense.
Why would you bother naming fields
if you're using a custom type anyway?

## `custom` keyword

```kiki
// ...

custom Numbers: Vec<i32> {
    $Number {
        vec![$1]
    }

    Numbers $Comma $Number {
        let mut out = $1;
        out.push($3);
        out
    }
}

// ...
```

We could still use `@0`, but I changed it to `$1`
for this example just to see how it'd look.
The advantage of `$1` syntax is the Bison people are already
familiar with it.
The disadvantage is that it conflicts with `$Token` syntax.
We'd probably have to change the token syntax.

We could alternatively use `type` or another keyword.
We could also use `fn` before differnt cases,
to illustrate the parallel with Rust functions.

```kiki
// ...

type Numbers: Vec<i32> {
    fn @Number {
        vec![$1]
    }

    fn Numbers @Comma @Number {
        let mut out = $1;
        out.push($3);
        out
    }
}

terminal Token {
    @Number: i32
    @Comma: _
}
```

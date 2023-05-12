# Proposal: builtins

```kiki
type Option<T>: std::option::Option<T> {
    fn T {
        Some($1)
    }

    fn {
        None
    }
}

type Vec<T>: std::vec::Vec<T> {
    fn Vec<T> T {
        let mut out = $1;
        out.push($2);
        out
    }

    fn {
        vec![]
    }
}
```

We could add syntax sugar. `Foo?` is `Option<Foo>`.
`Foo*` is `Vec<Foo>`.

## Kleene plus

```kiki
type NonEmptyVec<T>: std::vec::Vec<T> {
    fn Vec<T> T {
        let mut out = $1;
        out.push($2);
        out
    }

    fn T {
        vec![$1]
    }
}
```

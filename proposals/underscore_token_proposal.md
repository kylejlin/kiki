# Underscore token proposal

This Kiki code...

```kiki
terminal Token {
    $Number: i32
    $Comma: ()
}
```

...results in the below Rust code:

```rust
enum Token {
    Number(i32),
    Comma(()),
}
```

It's undesirable to have a `()` as the Comma variant's field.
It would ideally have no field at all:

```rust
enum Token {
    Number(i32),
    Comma,
}
```

However, this would be obviously inconsistent.
(Since Kiki `$Foo: Bar` should result in Rust `Foo(Bar)`.)
This might create an annoying edge case for automation
developers down the line.

Instead, we should give the user the choice between the two, but make the fieldless variant the default.

## Proposal

```kiki
terminal Token {
    $Number: i32
    $Comma: _
}
```

If you use the underscore, no field is emitted for that variant.

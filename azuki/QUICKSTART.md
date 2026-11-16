# Azuki quickstart tutorial

This tutorial teaches you Azuki in under ten minutes.
For brevity, this tutorial skips many details.
Later, if you want clarification regarding those details,
you can read the [user guide](./USER_GUIDE.md).

This document contains an _Essential Tutorial_
plus several _Bonus Activities_.
The Essential Tutorial is the only part you need to complete.
It should take less than ten minutes.

The Bonus Activities are optional.
However, they introduce concepts that are not
covered in the Essential Tutorial.
So, I recommend that you read them if you have time.

## Table of contents

- [Essential tutorial](#essential-tutorial)
- [Bonus activity 1: using named fieldsets](#bonus-activity-1-using-named-fieldsets)
- [Bonus activity 2: using structs](#bonus-activity-2-using-structs)
- [Bonus activity 3: custom terminal types](#bonus-activity-3-custom-terminal-types)
- [Bonus activity 4: borrowing from the input](#bonus-activity-4-borrowing-from-the-input)

## Essential tutorial

You need Rust 1.98 or newer.

1. Initialize the project using Cargo.
   To do this, open a terminal and run:

   ```sh
   cargo new --lib azuki_quickstart
   cd azuki_quickstart
   ```

2. Add the following to `Cargo.toml`:

   ```toml
   [build-dependencies]
   azuki = "0.1"
   walkdir = "2"
   ```

3. Create a `build.rs` file
   in the **root directory** of the project.
   Do **not** create the file inside
   the `src` directory.
4. Add the following to `build.rs`:

   ```rs
   use azuki::convenience::azuki_to_rust;
   use azuki::source_text::FileSource;
   use walkdir::WalkDir;

   use std::ffi::OsStr;
   use std::fs;
   use std::path::Path;

   fn main() {
       for entry in WalkDir::new("./src").follow_links(true) {
           let entry = entry.unwrap();

           if entry.path().extension() == Some(OsStr::new("azuki")) {
               let file_contents = fs::read_to_string(entry.path()).unwrap();

               let rs_path = entry
                   .path()
                   .parent()
                   .unwrap()
                   .join(Path::new(entry.path().file_stem().unwrap()).with_extension("rs"));

               let file_path = entry.path().display().to_string();
               let rust_src = match azuki_to_rust(FileSource::new(&file_contents), &file_path) {
                   Ok(s) => s,
                   Err(err) => {
                       panic!("Invalid Azuki file {file_path}. Error: {err}");
                   }
               };
               if let Err(err) = fs::write(&rs_path, &rust_src.raw) {
                   let rs_path = rs_path.display();
                   panic!("Cannot write to \"{rs_path}\". Error: {err:#?}")
               };
           }
       }
   }

   ```

   This script searches the `src` directory for
   `.azuki` files, and runs Azuki on those files.

5. In the `src` directory, create a `balanced_parens.azuki` file:

   ```azuki
   start Expr

   rust"#[derive(Clone, Debug, PartialEq, Eq)]"
   enum Expr {
       Empty
       Wrapped(
           _: $LParen
           BoxedExpr
           _: $RParen
       )
   }

   manual BoxedExpr = @BoxedExpr {
       @boxed(Expr)
   }

   rust"#[derive(Clone, Copy, Debug, PartialEq, Eq)]"
   terminal Token {
       $LParen = @Unit
       $RParen = @Unit
   }

   bridge crate::azuki_bridge
   ```

   This is an Azuki _grammar_.
   Azuki uses this file to generate a file
   called `balanced_parens.rs`.

6. In the `src` directory, create an `azuki_bridge.rs` file:

   ```rs
   use crate::balanced_parens::non_terminals::Expr;

   pub type Unit = ();
   pub type BoxedExpr = Box<Expr>;

   pub fn boxed(inner: Expr) -> BoxedExpr {
       Box::new(inner)
   }
   ```

   This is the Rust module referenced by the `bridge` declaration.
   Azuki gets terminal types and manual functions from this module.
   Unlike Kiki, Azuki does not automatically wrap nonterminals in `Box`.
   The `manual BoxedExpr = @BoxedExpr` declaration tells it to use
   the bridge type `BoxedExpr` and our `boxed` function.

7. Delete the original contents of `lib.rs`.
   Then, paste the following:

   ```rs
   mod azuki_bridge;
   mod balanced_parens;

   #[cfg(test)]
   mod tests {
       use crate::balanced_parens::non_terminals::Expr;
       use crate::balanced_parens::{Token, parse};

       #[test]
       fn wrap_2() {
           let actual = parse(vec![left(), left(), right(), right()]);
           let expected = Ok(Expr::Wrapped(Box::new(Expr::Wrapped(Box::new(
               Expr::Empty,
           )))));
           assert_eq!(expected, actual);
       }

       fn left() -> Token {
           Token::LParen(())
       }

       fn right() -> Token {
           Token::RParen(())
       }
   }
   ```

8. Run `cargo test`.
   This should result in exactly one test (`wrap_2`) being run.
   That test should pass.

Congratulations! You created your first Azuki project.

The full form `manual BoxedExpr = @BoxedExpr` names both
the grammar symbol (on the left) and its bridge type (after `@`).
Since those names are the same, you can omit `= @BoxedExpr`:

```azuki
manual BoxedExpr {
    @boxed(Expr)
}
```

This is shorthand for the full declaration above.

## Bonus activity 1: using named fieldsets

1.  In `src/balanced_parens.azuki`, replace the definition
    of `Expr` with

    ```azuki
    rust"#[derive(Clone, Debug, PartialEq, Eq)]"
    enum Expr {
        Empty
        Wrapped {
            left: $LParen
            inner: BoxedExpr
            right: $RParen
        }
    }
    ```

2.  In `src/lib.rs`, replace the definition of `wrap_2` with:

    ```rs
    #[test]
    fn wrap_2() {
        let actual = parse(vec![left(), left(), right(), right()]);
        let expected = Ok(Expr::Wrapped {
            left: (),
            inner: Box::new(Expr::Wrapped {
                left: (),
                inner: Box::new(Expr::Empty),
                right: (),
            }),
            right: (),
        });
        assert_eq!(expected, actual);
    }
    ```

3.  Run `cargo test`.
    Once again, there should be exactly one test (`wrap_2`),
    and that test should pass.

In this example, we introduced _named fieldsets_.
In the previous example, we used _tuple fieldsets_.
Notice the differing syntax.

## Bonus activity 2: using structs

1.  Delete all the contents of `src/balanced_parens.azuki`,
    and replace it with:

    ```azuki
    start Expr

    rust"#[derive(Clone, Debug, PartialEq, Eq)]"
    enum Expr {
        Empty
        Wrapped(Wrapped)
    }

    rust"#[derive(Clone, Debug, PartialEq, Eq)]"
    struct Wrapped {
        _: $LParen
        inner: BoxedExpr
        _: $RParen
    }

    manual BoxedExpr = @BoxedExpr {
        @boxed(Expr)
    }

    rust"#[derive(Clone, Copy, Debug, PartialEq, Eq)]"
    terminal Token {
        $LParen = @Unit
        $RParen = @Unit
    }

    bridge crate::azuki_bridge
    ```

2.  Delete all the contents of `src/lib.rs`,
    and replace it with:

    ```rs
    mod azuki_bridge;
    mod balanced_parens;

    #[cfg(test)]
    mod tests {
        use crate::balanced_parens::non_terminals::{Expr, Wrapped};
        use crate::balanced_parens::{Token, parse};

        #[test]
        fn wrap_2() {
            let actual = parse(vec![left(), left(), right(), right()]);
            let expected = Ok(Expr::Wrapped(Wrapped {
                inner: Box::new(Expr::Wrapped(Wrapped {
                    inner: Box::new(Expr::Empty),
                })),
            }));
            assert_eq!(expected, actual);
        }

        fn left() -> Token {
            Token::LParen(())
        }

        fn right() -> Token {
            Token::RParen(())
        }
    }
    ```

3.  Run `cargo test`.
    Once again, there should be exactly one test (`wrap_2`),
    and that test should pass.

In this example, we introduced _structs_.
The struct in this example has a named fieldset.
However, structs may also use tuple fieldsets.

## Bonus activity 3: custom terminal types

1.  Delete all the contents of `src/balanced_parens.azuki`,
    and replace it with:

    ```azuki
    start Expr

    rust"#[derive(Clone, Debug, PartialEq, Eq)]"
    enum Expr {
        Empty
        Wrapped {
            left: $LParen
            inner: BoxedExpr
            right: $RParen
        }
    }

    manual BoxedExpr = @BoxedExpr {
        @boxed(Expr)
    }

    rust"#[derive(Clone, Debug, PartialEq, Eq)]"
    terminal Token {
        $LParen = @RawToken
        $RParen = @RawToken
    }

    bridge crate::azuki_bridge
    ```

    Observe that we changed the type of `$LParen` and `$RParen`.
    Previously, the type was `()` (i.e., the unit type).
    Now, the type is `RawToken`, which we will define in Rust.
    The `@RawToken` name refers to a type in the bridge module.

2.  Add the following to `src/azuki_bridge.rs`:

    ```rs
    pub use crate::RawToken;
    ```

3.  Delete all the contents of `src/lib.rs`,
    and replace it with:

    ```rs
    mod azuki_bridge;
    mod balanced_parens;

    #[cfg(test)]
    mod tests {
        use crate::balanced_parens::non_terminals::Expr;
        use crate::balanced_parens::{Token, parse};
        use crate::RawToken;

        #[test]
        fn wrap_2() {
            let tokens = lex("(())");
            let actual = parse(tokens);
            let expected = Ok(Expr::Wrapped {
                left: RawToken {
                    content: "(".to_string(),
                    span: (0, 1),
                },
                inner: Box::new(Expr::Wrapped {
                    left: RawToken {
                        content: "(".to_string(),
                        span: (1, 2),
                    },
                    inner: Box::new(Expr::Empty),
                    right: RawToken {
                        content: ")".to_string(),
                        span: (2, 3),
                    },
                }),
                right: RawToken {
                    content: ")".to_string(),
                    span: (3, 4),
                },
            });
            assert_eq!(expected, actual);
        }

        fn lex(s: &str) -> Vec<Token> {
            let mut out = vec![];
            for (i, c) in s.char_indices() {
                match c {
                    '(' => out.push(Token::LParen(RawToken {
                        content: c.to_string(),
                        span: (i, i + c.len_utf8()),
                    })),

                    ')' => out.push(Token::RParen(RawToken {
                        content: c.to_string(),
                        span: (i, i + c.len_utf8()),
                    })),

                    _ => panic!("Unexpected character: {c:?}"),
                }
            }
            out
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct RawToken {
        pub content: String,
        pub span: (usize, usize),
    }
    ```

4.  Run `cargo test`.
    Once again, there should be exactly one test (`wrap_2`),
    and that test should pass.

## Bonus activity 4: borrowing from the input

So far, the terminal types have owned their data.
What if we want to borrow from the input instead?
For example, suppose we want to parse a sequence of words
and store them in a `Vec<&str>`.

1.  Delete all the contents of `src/balanced_parens.azuki`,
    and replace it with:

    ```azuki
    start Document

    rust"#[derive(Clone, Debug, PartialEq, Eq)]"
    struct Document {
        words: Words
    }

    manual Words = @Words 'src {
        @empty
        @push(Words $Word)
    }

    rust"#[derive(Clone, Debug, PartialEq, Eq)]"
    terminal Token 'src {
        $Word = @Text 'src
    }

    bridge crate::azuki_bridge
    ```

2.  Delete all the contents of `src/azuki_bridge.rs`,
    and replace it with:

    ```rs
    pub type Text<'src> = &'src str;
    pub type Words<'src> = Vec<Text<'src>>;

    pub fn empty<'src>() -> Words<'src> {
        Vec::new()
    }

    pub fn push<'src>(mut words: Words<'src>, word: Text<'src>) -> Words<'src> {
        words.push(word);
        words
    }
    ```

3.  Delete all the contents of `src/lib.rs`,
    and replace it with:

    ```rs
    mod azuki_bridge;
    mod balanced_parens;

    #[cfg(test)]
    mod tests {
        use crate::balanced_parens::non_terminals::Document;
        use crate::balanced_parens::{Token, parse};

        #[test]
        fn words() {
            let source = String::from("hello azuki");
            let tokens = source.split_whitespace().map(Token::Word);
            let actual = parse(tokens);
            let expected = Ok(Document {
                words: vec!["hello", "azuki"],
            });
            assert_eq!(expected, actual);
        }
    }
    ```

4.  Run `cargo test`.
    This time, there should be exactly one test (`words`),
    and that test should pass.

Observe that `Words` is a manual nonterminal,
just like `BoxedExpr` in the Essential Tutorial.
This time, it has two rules:
`@empty` creates an empty vector,
and `@push(Words $Word)` adds a word to an existing vector.

The `terminal Token 'src` declaration introduces the lifetime `'src`.
The `$Word` and `manual Words` declarations say that their Rust types
use that lifetime.
Azuki infers that `Document` also needs `'src`,
since it stores a `Words`.
You do not write the lifetime on `Document` or on the `words` field.

Now that we've seen the full form, we can shorten the declaration:

```azuki
manual Words 'src {
    @empty
    @push(Self $Word)
}
```

We omitted `= @Words` because the grammar symbol and bridge type
have the same name.
We also used `Self` as shorthand for the enclosing nonterminal, `Words`.

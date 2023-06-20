# Kiki quickstart tutorial

This tutorial teaches you Kiki in under ten minutes.
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

## Essential tutorial

1. Initialize the project using Cargo.
   To do this, open a terminal and run:

   ```sh
   cargo new --lib kiki_quickstart
   cd kiki_quickstart
   ```

2. Add the following to `Cargo.toml`:

   ```toml
   [build-dependencies]
   kiki = "6"
   walkdir = "2"
   ```

3. Create a `build.rs` file
   in the **root directory** of the project.
   Do **not** create the file inside
   the `src` directory.
4. Add the following to `build.rs`:

   ```rs
   extern crate kiki;
   extern crate walkdir;

   use walkdir::WalkDir;

   use std::ffi::OsStr;
   use std::fs;
   use std::path::Path;

   fn main() {
       for entry in WalkDir::new("./src").follow_links(true) {
           let entry = entry.unwrap();

           if entry.path().extension() == Some(OsStr::new("kiki")) {
               let file_contents = fs::read_to_string(entry.path()).unwrap();

               let rs_path = entry
                   .path()
                   .parent()
                   .unwrap()
                   .join(Path::new(entry.path().file_stem().unwrap()).with_extension("rs"));

               let rust_src = match kiki::generate(&file_contents) {
                   Ok(s) => s,
                   Err(err) => {
                       let file_path = entry.path().display();
                       panic!("Invalid Kiki file {file_path}. Error: {err:#?}");
                   }
               };
               if let Err(err) = fs::write(&rs_path, &rust_src.0) {
                   let rs_path = rs_path.display();
                   panic!("Cannot write to \"{rs_path}\". Error: {err:#?}")
               };
           }
       }
   }

   ```

   This script searches the `src` directory for
   `.kiki` files, and runs Kiki on those files.

5. In the `src` directory, create a `balanced_parens.kiki` file:

   ```
   start Expr

   #[derive(Clone, Debug, PartialEq, Eq)]
   enum Expr {
       Empty
       Wrapped(
           _: $LParen
           Expr
           _: $RParen
       )
   }

   #[derive(Clone, Copy, Debug, PartialEq, Eq)]
   terminal Token {
       $LParen: ()
       $RParen: ()
   }
   ```

   This is a Kiki _grammar_.
   Kiki uses this file to generate a file
   called `balanced_parens.rs`.

6. Delete the original contents of `lib.rs`.
   Then, paste the following:

   ```rs
   mod balanced_parens;

   #[cfg(test)]
   mod tests {
       use crate::balanced_parens::{parse, Expr, Token};

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

7. Run `cargo test`.
   This should result in exactly one test (`wrap_2`) being ran.
   That test should pass.

Congratulations! You created your first Kiki project.

## Bonus activity 1: using named fieldsets

1.  In `src/balanced_parens.kiki`, replace the definition
    of `Expr` with

    ```kiki
    #[derive(Clone, Debug, PartialEq, Eq)]
    enum Expr {
        Empty
        Wrapped {
            left: $LParen
            inner: Expr
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

1.  Delete all the contents of `src/balanced_parens.kiki`,
    and replace it with:

    ```kiki
    start Expr

    #[derive(Clone, Debug, PartialEq, Eq)]
    enum Expr {
        Empty
        Wrapped(Wrapped)
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Wrapped {
        _: $LParen
        inner: Expr
        _: $RParen
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    terminal Token {
        $LParen: ()
        $RParen: ()
    }
    ```

2.  Delete all the contents of `src/lib.rs`,
    and replace it with:

    ```rs
    mod balanced_parens;

    #[cfg(test)]
    mod tests {
        use crate::balanced_parens::{parse, Expr, Token, Wrapped};

        #[test]
        fn wrap_2() {
            let actual = parse(vec![left(), left(), right(), right()]);
            let expected = Ok(Expr::Wrapped(Box::new(Wrapped {
                inner: Box::new(Expr::Wrapped(Box::new(Wrapped {
                    inner: Box::new(Expr::Empty),
                }))),
            })));
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

1.  Delete all the contents of `src/balanced_parens.kiki`,
    and replace it with:

    ```kiki
    start Expr

    #[derive(Clone, Debug, PartialEq, Eq)]
    enum Expr {
        Empty
        Wrapped {
            left: $LParen
            inner: Expr
            right: $RParen
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    terminal Token {
        $LParen: crate::RawToken
        $RParen: crate::RawToken
    }
    ```

    Observe that we changed the type of `$LParen` and `$RParen`.
    Previously, the type was `()` (i.e., the unit type).
    Now, the type is `crate::RawToken`.

2.  Delete all the contents of `src/lib.rs`,
    and replace it with:

    ```rs
    mod balanced_parens;

    #[cfg(test)]
    mod tests {
        use crate::balanced_parens::{parse, Expr, Token};
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

3.  Run `cargo test`.
    Once again, there should be exactly one test (`wrap_2`),
    and that test should pass.

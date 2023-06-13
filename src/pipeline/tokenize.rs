use crate::{
    data::{
        token::Token,
        token::{Ident, TerminalIdent},
        ByteIndex, KikiErr,
    },
    DollarlessTerminalName,
};

pub fn tokenize(src: &str) -> Result<Vec<Token>, KikiErr> {
    let tokenizer = Tokenizer::new(src);
    tokenizer.tokenize()
}

struct Tokenizer<'a> {
    src: &'a str,
    out: Vec<Token>,
    state: State,
}

impl<'a> Tokenizer<'a> {
    fn new(src: &'a str) -> Self {
        Tokenizer {
            src,
            out: vec![],
            state: State::Main,
        }
    }
}

impl Tokenizer<'_> {
    fn tokenize(mut self) -> Result<Vec<Token>, KikiErr> {
        for (c_index, c) in self.src.char_indices() {
            self.handle_char(c, ByteIndex(c_index))?;
        }
        Ok(self.out)
    }

    fn handle_char(&mut self, current: char, current_index: ByteIndex) -> Result<(), KikiErr> {
        match self.state {
            State::Main => self.handle_char_given_state_is_whitespace(current, current_index),
            State::Slash(start) => {
                self.handle_char_given_state_is_slash(current, current_index, start)
            }
            State::SingleLineComment => {
                self.handle_char_given_state_is_single_line_comment(current, current_index)
            }
            State::Ident(start, end) => {
                self.handle_char_given_state_is_ident(current, current_index, start, end)
            }
            State::Dollar(start) => {
                self.handle_char_given_state_is_dollar(current, current_index, start)
            }
            State::TerminalIdent(start, end) => {
                self.handle_char_given_state_is_terminal_ident(current, current_index, start, end)
            }
            State::Colon(start) => {
                self.handle_char_given_state_is_colon(current, current_index, start)
            }
        }
    }

    fn handle_char_given_state_is_whitespace(
        &mut self,
        current: char,
        current_index: ByteIndex,
    ) -> Result<(), KikiErr> {
        if current.is_whitespace() {
            Ok(())
        } else if current == '/' {
            self.state = State::Slash(current_index);
            Ok(())
        } else if current.is_ascii_alphabetic() || current == '_' {
            self.state = State::Ident(
                current_index,
                ByteIndex(current_index.0 + current.len_utf8()),
            );
            Ok(())
        } else if current == '$' {
            self.state = State::Dollar(current_index);
            Ok(())
        } else if current == ':' {
            self.state = State::Colon(current_index);
            Ok(())
        } else {
            Err(KikiErr::Lex(current_index, Some(current)))
        }
    }

    fn handle_char_given_state_is_slash(
        &mut self,
        current: char,
        current_index: ByteIndex,
        existing_slash_index: ByteIndex,
    ) -> Result<(), KikiErr> {
        if current == '/' {
            self.state = State::SingleLineComment;
            Ok(())
        } else {
            Err(KikiErr::Lex(existing_slash_index, Some('/')))
        }
    }

    fn handle_char_given_state_is_single_line_comment(
        &mut self,
        current: char,
        current_index: ByteIndex,
    ) -> Result<(), KikiErr> {
        if current == '\n' {
            self.state = State::Main;
            Ok(())
        } else {
            Ok(())
        }
    }

    fn handle_char_given_state_is_ident(
        &mut self,
        current: char,
        current_index: ByteIndex,
        start: ByteIndex,
        end: ByteIndex,
    ) -> Result<(), KikiErr> {
        if current.is_ascii_alphanumeric() || current == '_' {
            self.state = State::Ident(start, ByteIndex(end.0 + current.len_utf8()));
            Ok(())
        } else {
            let name = self.src[start.0..end.0].to_string();
            self.out.push(Token::Ident(Ident {
                name,
                position: start,
            }));
            self.state = State::Main;
            self.handle_char(current, current_index)
        }
    }

    fn handle_char_given_state_is_dollar(
        &mut self,
        current: char,
        current_index: ByteIndex,
        dollar_index: ByteIndex,
    ) -> Result<(), KikiErr> {
        if current.is_ascii_alphabetic() || current == '_' {
            self.state =
                State::TerminalIdent(dollar_index, ByteIndex(dollar_index.0 + current.len_utf8()));
            Ok(())
        } else {
            Err(KikiErr::Lex(dollar_index, Some('$')))
        }
    }

    fn handle_char_given_state_is_terminal_ident(
        &mut self,
        current: char,
        current_index: ByteIndex,
        dollar_index: ByteIndex,
        end: ByteIndex,
    ) -> Result<(), KikiErr> {
        if current.is_ascii_alphanumeric() || current == '_' {
            self.state = State::TerminalIdent(dollar_index, ByteIndex(end.0 + current.len_utf8()));
            Ok(())
        } else {
            let name = DollarlessTerminalName::remove_dollars(&self.src[dollar_index.0..end.0]);
            let dollarless_position = ByteIndex(dollar_index.0 + "$".len());
            self.out.push(Token::TerminalIdent(TerminalIdent {
                name,
                dollarless_position,
            }));
            self.state = State::Main;
            self.handle_char(current, current_index)
        }
    }

    fn handle_char_given_state_is_colon(
        &mut self,
        current: char,
        current_index: ByteIndex,
        start: ByteIndex,
    ) -> Result<(), KikiErr> {
        if current == ':' {
            self.out.push(Token::DoubleColon(start));
            self.state = State::Main;
            Ok(())
        } else {
            self.out.push(Token::Colon(start));
            self.state = State::Main;
            self.handle_char(current, current_index)
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum State {
    Main,
    Slash(ByteIndex),
    SingleLineComment,
    Ident(ByteIndex, ByteIndex),
    Dollar(ByteIndex),
    TerminalIdent(ByteIndex, ByteIndex),
    Colon(ByteIndex),
}

// match {
//     r"//[^\n]*" => {},
//     r"\s+" => {}
// } else {
//     _
// }

// pub StartKw: () = "start" => ();
// pub StructKw: () = "struct" => ();
// pub EnumKw: () = "enum" => ();
// pub TerminalKw: () = "terminal" => ();

// pub DoubleColon: () = "::" => ();
// pub Colon: () = ":" => ();
// pub Comma: () = "," => ();

// pub LParen: () = "(" => ();
// pub RParen: () = ")" => ();
// pub LCurly: () = "{" => ();
// pub RCurly: () = "}" => ();
// pub LAngle: () = "<" => ();
// pub RAngle: () = ">" => ();

// pub Underscore: () = "_" => ();

// pub Ident: Ident = <s:r"[a-zA-Z_][a-zA-Z_0-9]*"> => Ident {
//     name: s.to_owned(),
//     // TODO
//     position: ByteIndex(0),
// };
// pub TerminalIdent: TerminalIdent = <s:r"\$[a-zA-Z_][a-zA-Z_0-9]*"> => TerminalIdent {
//     name: DollarlessTerminalName::remove_dollars(s),
//     // TODO
//     dollarless_position: ByteIndex(1),
// };

// #[inline]
// Epsilon: () = ();

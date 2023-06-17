use crate::{
    data::{
        token::Token,
        token::{Attribute, Ident, TerminalIdent},
        ByteIndex, KikiErr,
    },
    DollarlessTerminalName,
};

use std::num::NonZeroUsize;

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

        self.push_pending_token_and_reset_state(None, ByteIndex(self.src.len()))?;

        Ok(self.out)
    }

    fn handle_char(&mut self, current: char, current_index: ByteIndex) -> Result<(), KikiErr> {
        match self.state {
            State::Main => self.handle_char_given_state_is_main(current, current_index),
            State::Slash(start) => self.handle_char_given_state_is_slash(current, start),
            State::SingleLineComment => {
                self.handle_char_given_state_is_single_line_comment(current)
            }
            State::Ident(start, end) => {
                self.handle_char_given_state_is_ident(current, current_index, start, end)
            }
            State::Dollar(start) => self.handle_char_given_state_is_dollar(current, start),
            State::TerminalIdent(start, end) => {
                self.handle_char_given_state_is_terminal_ident(current, current_index, start, end)
            }
            State::Colon(start) => {
                self.handle_char_given_state_is_colon(current, current_index, start)
            }
            State::Pound(start) => {
                self.handle_char_given_state_is_pound(current, current_index, start)
            }
            State::OuterAttribute(start, left_count, end) => self
                .handle_char_given_state_is_outer_attribute(
                    current,
                    current_index,
                    start,
                    left_count,
                    end,
                ),
        }
    }

    fn handle_char_given_state_is_main(
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
        } else if current == '#' {
            self.state = State::Pound(current_index);
            Ok(())
        } else if let Some(kind) = get_single_char_punctuation_kind(current) {
            self.out
                .push(get_single_char_punctuation_token(kind, current_index));
            Ok(())
        } else {
            Err(KikiErr::Lex(current_index, Some(current)))
        }
    }

    fn handle_char_given_state_is_slash(
        &mut self,
        current: char,
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
    ) -> Result<(), KikiErr> {
        if current == '\n' {
            self.state = State::Main;
        }

        Ok(())
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
            self.push_pending_token_and_reset_state(Some(current), current_index)?;
            self.handle_char(current, current_index)
        }
    }

    fn handle_char_given_state_is_dollar(
        &mut self,
        current: char,
        dollar_index: ByteIndex,
    ) -> Result<(), KikiErr> {
        if current.is_ascii_alphabetic() || current == '_' {
            self.state = State::TerminalIdent(
                dollar_index,
                ByteIndex(dollar_index.0 + '$'.len_utf8() + current.len_utf8()),
            );
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
            self.push_pending_token_and_reset_state(Some(current), current_index)?;
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
            self.push_pending_token_and_reset_state(Some(current), current_index)?;
            self.handle_char(current, current_index)
        }
    }

    fn handle_char_given_state_is_pound(
        &mut self,
        current: char,
        current_index: ByteIndex,
        start: ByteIndex,
    ) -> Result<(), KikiErr> {
        if current == '[' {
            self.state = State::OuterAttribute(
                start,
                LeftBracketCount(NonZeroUsize::new(1).unwrap()),
                ByteIndex(current_index.0 + "[".len()),
            );
            Ok(())
        } else {
            self.push_pending_token_and_reset_state(Some(current), current_index)?;
            self.handle_char(current, current_index)
        }
    }

    fn handle_char_given_state_is_outer_attribute(
        &mut self,
        current: char,
        current_index: ByteIndex,
        start: ByteIndex,
        left_count: LeftBracketCount,
        end: ByteIndex,
    ) -> Result<(), KikiErr> {
        match current {
            '(' | '[' | '{' => {
                self.state = State::OuterAttribute(
                    start,
                    LeftBracketCount(left_count.0.saturating_add(1)),
                    ByteIndex(end.0 + current.len_utf8()),
                );
                Ok(())
            }

            ')' | ']' | '}' => {
                if left_count.0.get() == 1 {
                    let end = ByteIndex(end.0 + current.len_utf8());
                    return self.finish_outer_attribute(start, end);
                }

                self.state = State::OuterAttribute(
                    start,
                    LeftBracketCount(NonZeroUsize::new(left_count.0.get() - 1).unwrap()),
                    ByteIndex(end.0 + current.len_utf8()),
                );
                Ok(())
            }

            '\n' => Err(KikiErr::Lex(current_index, Some(current))),

            _ => {
                self.state = State::OuterAttribute(start, left_count, ByteIndex(end.0 + 1));
                Ok(())
            }
        }
    }

    fn finish_outer_attribute(&mut self, start: ByteIndex, end: ByteIndex) -> Result<(), KikiErr> {
        let mut stack = Vec::new();
        let bracket_start = ByteIndex(start.0 + "#".len());
        for (current_index, current) in self.src[bracket_start.0..end.0].char_indices() {
            match current {
                '(' | '[' | '{' => {
                    stack.push(current);
                }

                ')' | ']' | '}' => {
                    let Some(top) = stack.pop() else {
                        return Err(KikiErr::Lex(
                            ByteIndex(current_index),
                            Some(current),
                        ));
                    };
                    match (top, current) {
                        ('(', ')') | ('[', ']') | ('{', '}') => {}

                        _ => {
                            return Err(KikiErr::Lex(ByteIndex(current_index), Some(current)));
                        }
                    }
                }

                _ => {}
            }
        }

        self.state = State::Main;
        self.out.push(Token::OuterAttribute(Attribute {
            src: self.src[start.0..end.0].to_string(),
            position: start,
        }));
        Ok(())
    }

    /// This function only resets the state if the pending token is valid.
    fn push_pending_token_and_reset_state(
        &mut self,
        current: Option<char>,
        current_index: ByteIndex,
    ) -> Result<(), KikiErr> {
        match self.state {
            State::Main => Ok(()),

            State::Slash(slash_index) => Err(KikiErr::Lex(slash_index, Some('/'))),

            State::SingleLineComment => Ok(()),

            State::Ident(start, end) => {
                let name = &self.src[start.0..end.0];

                if let Some(kind) = get_reserved_word_kind(name) {
                    self.out.push(get_reserved_word_token(kind, start));
                } else {
                    self.out.push(Token::Ident(Ident {
                        name: name.to_string(),
                        position: start,
                    }));
                }

                Ok(())
            }

            State::Dollar(dollar_index) => Err(KikiErr::Lex(dollar_index, Some('$'))),

            State::TerminalIdent(start, end) => {
                let name = DollarlessTerminalName::remove_dollars(&self.src[start.0..end.0]);

                if get_reserved_word_kind(name.raw()).is_some() {
                    return Err(KikiErr::Lex(current_index, current));
                }

                let dollarless_position = ByteIndex(start.0 + "$".len());
                self.out.push(Token::TerminalIdent(TerminalIdent {
                    name,
                    dollarless_position,
                }));
                Ok(())
            }

            State::Colon(colon_index) => {
                self.out.push(Token::Colon(colon_index));
                self.state = State::Main;
                Ok(())
            }

            State::Pound(start) => Err(KikiErr::Lex(start, Some('#'))),

            State::OuterAttribute(start, _, end) => self.finish_outer_attribute(start, end),
        }?;

        self.state = State::Main;
        Ok(())
    }
}

#[derive(Debug, Clone)]
enum State {
    Main,
    Slash(ByteIndex),
    SingleLineComment,
    Ident(ByteIndex, ByteIndex),
    Dollar(ByteIndex),
    TerminalIdent(ByteIndex, ByteIndex),
    Colon(ByteIndex),
    Pound(ByteIndex),
    OuterAttribute(ByteIndex, LeftBracketCount, ByteIndex),
}

#[derive(Debug, Clone, Copy)]
struct LeftBracketCount(NonZeroUsize);

#[derive(Debug, Clone, Copy)]
enum ReservedWordKind {
    Underscore,
    Start,
    Struct,
    Enum,
    Terminal,
}

#[derive(Debug, Clone, Copy)]
enum SingleCharPunctuationKind {
    Colon,
    Comma,
    LParen,
    RParen,
    LCurly,
    RCurly,
    LAngle,
    RAngle,
}

fn get_reserved_word_kind(s: &str) -> Option<ReservedWordKind> {
    match s {
        "_" => Some(ReservedWordKind::Underscore),
        "start" => Some(ReservedWordKind::Start),
        "struct" => Some(ReservedWordKind::Struct),
        "enum" => Some(ReservedWordKind::Enum),
        "terminal" => Some(ReservedWordKind::Terminal),
        _ => None,
    }
}

fn get_reserved_word_token(kind: ReservedWordKind, index: ByteIndex) -> Token {
    match kind {
        ReservedWordKind::Underscore => Token::Underscore(index),
        ReservedWordKind::Start => Token::StartKw(index),
        ReservedWordKind::Struct => Token::StructKw(index),
        ReservedWordKind::Enum => Token::EnumKw(index),
        ReservedWordKind::Terminal => Token::TerminalKw(index),
    }
}

fn get_single_char_punctuation_kind(c: char) -> Option<SingleCharPunctuationKind> {
    match c {
        ':' => Some(SingleCharPunctuationKind::Colon),
        ',' => Some(SingleCharPunctuationKind::Comma),
        '(' => Some(SingleCharPunctuationKind::LParen),
        ')' => Some(SingleCharPunctuationKind::RParen),
        '{' => Some(SingleCharPunctuationKind::LCurly),
        '}' => Some(SingleCharPunctuationKind::RCurly),
        '<' => Some(SingleCharPunctuationKind::LAngle),
        '>' => Some(SingleCharPunctuationKind::RAngle),
        _ => None,
    }
}

fn get_single_char_punctuation_token(kind: SingleCharPunctuationKind, index: ByteIndex) -> Token {
    match kind {
        SingleCharPunctuationKind::Colon => Token::Colon(index),
        SingleCharPunctuationKind::Comma => Token::Comma(index),
        SingleCharPunctuationKind::LParen => Token::LParen(index),
        SingleCharPunctuationKind::RParen => Token::RParen(index),
        SingleCharPunctuationKind::LCurly => Token::LCurly(index),
        SingleCharPunctuationKind::RCurly => Token::RCurly(index),
        SingleCharPunctuationKind::LAngle => Token::LAngle(index),
        SingleCharPunctuationKind::RAngle => Token::RAngle(index),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    // ## Lexing should succeed

    #[test]
    fn finish_ident() {
        let src = "foo";
        let actual = tokenize(src).unwrap();
        let expected = vec![Token::Ident(Ident {
            name: "foo".to_string(),
            position: ByteIndex(0),
        })];
        assert_eq!(expected, actual);
    }

    #[test]
    fn finish_terminal_ident() {
        let src = "$foo";
        let actual = tokenize(src).unwrap();
        let expected = vec![Token::TerminalIdent(TerminalIdent {
            name: DollarlessTerminalName::remove_dollars("foo"),
            dollarless_position: ByteIndex(1),
        })];
        assert_eq!(expected, actual);
    }

    #[test]
    fn double_double_single_colon() {
        let src = ":::::";
        let actual = tokenize(src).unwrap();
        let expected = vec![
            Token::DoubleColon(ByteIndex(0)),
            Token::DoubleColon(ByteIndex(2)),
            Token::Colon(ByteIndex(4)),
        ];
        assert_eq!(expected, actual);
    }

    #[test]
    fn single_colons() {
        let src = ": : : : :";
        let actual = tokenize(src).unwrap();
        let expected = vec![
            Token::Colon(ByteIndex(0)),
            Token::Colon(ByteIndex(2)),
            Token::Colon(ByteIndex(4)),
            Token::Colon(ByteIndex(6)),
            Token::Colon(ByteIndex(8)),
        ];
        assert_eq!(expected, actual);
    }

    #[test]
    fn finish_colon() {
        let src = ":";
        let actual = tokenize(src).unwrap();
        let expected = vec![Token::Colon(ByteIndex(0))];
        assert_eq!(expected, actual);
    }

    #[test]
    fn empty_comment() {
        let src = "//";
        let actual = tokenize(src).unwrap();
        let expected: Vec<Token> = vec![];
        assert_eq!(expected, actual);
    }

    #[test]
    fn whitespace_only_comment() {
        let src = "// ";
        let actual = tokenize(src).unwrap();
        let expected: Vec<Token> = vec![];
        assert_eq!(expected, actual);
    }

    #[test]
    fn comment() {
        let src = "// Hello world!";
        let actual = tokenize(src).unwrap();
        let expected: Vec<Token> = vec![];
        assert_eq!(expected, actual);
    }

    // ## Lexing should fail

    #[test]
    fn slash() {
        let src = "/";
        let actual = tokenize(src).unwrap_err().lex_err().unwrap();
        let expected = (ByteIndex(0), Some('/'));
        assert_eq!(expected, actual);
    }

    #[test]
    fn slash_space() {
        let src = "/ /";
        let actual = tokenize(src).unwrap_err().lex_err().unwrap();
        let expected = (ByteIndex(0), Some('/'));
        assert_eq!(expected, actual);
    }

    #[test]
    fn slash_alpha() {
        let src = "/hello";
        let actual = tokenize(src).unwrap_err().lex_err().unwrap();
        let expected = (ByteIndex(0), Some('/'));
        assert_eq!(expected, actual);
    }

    #[test]
    fn slash_dollar() {
        let src = "/$foo";
        let actual = tokenize(src).unwrap_err().lex_err().unwrap();
        let expected = (ByteIndex(0), Some('/'));
        assert_eq!(expected, actual);
    }

    #[test]
    fn slash_lcurly() {
        let src = "/{";
        let actual = tokenize(src).unwrap_err().lex_err().unwrap();
        let expected = (ByteIndex(0), Some('/'));
        assert_eq!(expected, actual);
    }

    #[test]
    fn slash_rcurly() {
        let src = "/}";
        let actual = tokenize(src).unwrap_err().lex_err().unwrap();
        let expected = (ByteIndex(0), Some('/'));
        assert_eq!(expected, actual);
    }

    #[test]
    fn dollar() {
        let src = "$";
        let actual = tokenize(src).unwrap_err().lex_err().unwrap();
        let expected = (ByteIndex(0), Some('$'));
        assert_eq!(expected, actual);
    }

    #[test]
    fn dollar_dollar() {
        let src = "$$";
        let actual = tokenize(src).unwrap_err().lex_err().unwrap();
        let expected = (ByteIndex(0), Some('$'));
        assert_eq!(expected, actual);
    }

    #[test]
    fn dollar_comma() {
        let src = "$,";
        let actual = tokenize(src).unwrap_err().lex_err().unwrap();
        let expected = (ByteIndex(0), Some('$'));
        assert_eq!(expected, actual);
    }

    #[test]
    fn dollar_lcurly() {
        let src = "${";
        let actual = tokenize(src).unwrap_err().lex_err().unwrap();
        let expected = (ByteIndex(0), Some('$'));
        assert_eq!(expected, actual);
    }

    #[test]
    fn dollar_number() {
        let src = "$9";
        let actual = tokenize(src).unwrap_err().lex_err().unwrap();
        let expected = (ByteIndex(0), Some('$'));
        assert_eq!(expected, actual);
    }

    #[test]
    fn number() {
        let src = "4";
        let actual = tokenize(src).unwrap_err().lex_err().unwrap();
        let expected = (ByteIndex(0), Some('4'));
        assert_eq!(expected, actual);
    }

    #[test]
    fn number_letters() {
        let src = "4ever";
        let actual = tokenize(src).unwrap_err().lex_err().unwrap();
        let expected = (ByteIndex(0), Some('4'));
        assert_eq!(expected, actual);
    }

    #[test]
    fn dollar_keyword() {
        let src = "$start";
        let actual = tokenize(src).unwrap_err().lex_err().unwrap();
        let expected = (ByteIndex(src.len()), None);
        assert_eq!(expected, actual);
    }

    impl KikiErr {
        fn lex_err(self) -> Option<(ByteIndex, Option<char>)> {
            match self {
                KikiErr::Lex(i, c) => Some((i, c)),
                _ => None,
            }
        }
    }
}

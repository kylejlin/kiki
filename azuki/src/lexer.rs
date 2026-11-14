use crate::internal_prelude::*;

use logos::{Lexer as LogosLexer, Logos};

pub fn lex(source: &str) -> impl Iterator<Item = Result<Token<'_>, LexError>> {
    let logos_lexer = LogosToken::lexer_with_extras(
        source,
        LogosExtras {
            position: TextPosition { byte_index: 0 },
        },
    );
    Lexer {
        logos_lexer,
        is_done: false,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LexError {
    pub character: UnexpectedCharOrEndOfInput,
    pub position: TextPosition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TextPosition {
    pub byte_index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnexpectedCharOrEndOfInput {
    EndOfInput,
    Char(char),
}

struct Lexer<'a> {
    logos_lexer: LogosLexer<'a, LogosToken<'a>>,
    is_done: bool,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }

        let out = self.get_next_without_doneness_check();

        self.is_done = matches!(out, None | Some(Err(_)));

        out
    }
}

impl<'a> Lexer<'a> {
    fn get_next_without_doneness_check(&mut self) -> Option<Result<Token<'a>, LexError>> {
        let out = self.get_next_without_doneness_check_or_lookaround_check();

        if let Some(Ok(token)) = out {
            if let Err(e) = self.validate_lookarounds(token) {
                return Some(Err(e));
            }
        }

        out
    }

    fn get_next_without_doneness_check_or_lookaround_check(
        &mut self,
    ) -> Option<Result<Token<'a>, LexError>> {
        loop {
            let result = self.logos_lexer.next()?;

            let logos_token = match result {
                Ok(t) => t,

                Err(()) => {
                    return Some(Err(LexError {
                        character: match self.logos_lexer.source()
                            [self.logos_lexer.extras.position.byte_index..]
                            .chars()
                            .next()
                        {
                            Some(c) => UnexpectedCharOrEndOfInput::Char(c),
                            None => UnexpectedCharOrEndOfInput::EndOfInput,
                        },
                        position: self.logos_lexer.extras.position,
                    }));
                }
            };

            let converted = match logos_token.try_into() {
                Ok(t) => t,

                Err(LogosTokenToTokenConversionError::Skipped) => continue,

                Err(LogosTokenToTokenConversionError::IllegalCharacter(position)) => {
                    let c = self.logos_lexer.source()[position.byte_index..]
                        .chars()
                        .next()
                        .unwrap();
                    return Some(Err(LexError {
                        character: UnexpectedCharOrEndOfInput::Char(c),
                        position,
                    }));
                }
            };

            return Some(Ok(converted));
        }
    }

    fn validate_lookarounds(&self, token: Token<'a>) -> Result<(), LexError> {
        lookarounds::validate_lookarounds(token, self.logos_lexer.source())
    }
}

struct LogosExtras {
    pub position: TextPosition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Logos)]
#[logos(extras = LogosExtras)]
enum LogosToken<'a> {
    #[token("start", tagged_token_callback)]
    StartKw(token::StartKw<'a>),

    #[token("struct", tagged_token_callback)]
    StructKw(token::StructKw<'a>),

    #[token("enum", tagged_token_callback)]
    EnumKw(token::EnumKw<'a>),

    #[token("manual", tagged_token_callback)]
    ManualKw(token::ManualKw<'a>),

    #[token("terminal", tagged_token_callback)]
    TerminalKw(token::TerminalKw<'a>),

    #[token("bridge", tagged_token_callback)]
    BridgeKw(token::BridgeKw<'a>),

    #[token("_", tagged_token_callback)]
    Underscore(token::Underscore<'a>),

    #[token("(", tagged_token_callback)]
    LParen(token::LParen<'a>),

    #[token(")", tagged_token_callback)]
    RParen(token::RParen<'a>),

    #[token("[", tagged_token_callback)]
    LSquare(token::LSquare<'a>),

    #[token("]", tagged_token_callback)]
    RSquare(token::RSquare<'a>),

    #[token("{", tagged_token_callback)]
    LCurly(token::LCurly<'a>),

    #[token("}", tagged_token_callback)]
    RCurly(token::RCurly<'a>),

    #[token(",", tagged_token_callback)]
    Comma(token::Comma<'a>),

    #[token("::", tagged_token_callback)]
    DoubleColon(token::DoubleColon<'a>),

    #[token(":", tagged_token_callback)]
    Colon(token::Colon<'a>),

    #[token("=", tagged_token_callback)]
    Equal(token::Equal<'a>),

    #[token("#", tagged_token_callback)]
    Octothorpe(token::Octothorpe<'a>),

    #[regex(r#"[A-Z][a-zA-Z0-9_]*"#, tagged_token_callback)]
    UpperName(token::UpperName<'a>),

    #[regex(r#"[a-z][a-zA-Z0-9_]*"#, tagged_token_callback)]
    LowerName(token::LowerName<'a>),

    #[regex(r#"\$[A-Z][a-zA-Z0-9_]*"#, tagged_token_callback)]
    DollarUpperName(token::DollarUpperName<'a>),

    #[regex(r#"@[A-Z][a-zA-Z0-9_]*"#, tagged_token_callback)]
    AtUpperName(token::AtUpperName<'a>),

    #[regex(r#"@[a-z][a-zA-Z0-9_]*"#, tagged_token_callback)]
    AtLowerName(token::AtLowerName<'a>),

    #[regex(r#"rust#*""#, rust_string_tagged_token_callback)]
    RustStringLiteral(token::RustStringLiteral<'a>),

    #[regex(r"\/\/[^\n]*", position_callback, allow_greedy = true)]
    SingleLineComment(TextPosition),

    #[regex(r"[ \t\r\n\f]+", position_callback)]
    Whitespace(TextPosition),

    #[regex(r".", position_callback, priority = 0)]
    IllegalCharacter(TextPosition),
}

fn tagged_token_callback<'a, T>(
    logos_lexer: &mut LogosLexer<'a, LogosToken<'a>>,
) -> token::tagged_token::generic_tagged_token::TaggedToken<'a, T> {
    position_callback(logos_lexer);
    token::tagged_token::generic_tagged_token::TaggedToken::new(SourceFragment::new(
        logos_lexer.slice(),
    ))
}

fn position_callback<'a>(logos_lexer: &mut LogosLexer<'a, LogosToken<'a>>) -> TextPosition {
    let start = logos_lexer.extras.position;
    logos_lexer.extras.position.byte_index += logos_lexer.slice().len();
    start
}

fn rust_string_tagged_token_callback<'a>(
    logos_lexer: &mut LogosLexer<'a, LogosToken<'a>>,
) -> Option<token::RustStringLiteral<'a>> {
    let opener = logos_lexer.slice();
    let hashes = &opener.as_bytes()["rust".len()..opener.len() - 1];
    let remainder = logos_lexer.remainder();
    let terminator_end = remainder.match_indices('"').find_map(|(index, _)| {
        remainder.as_bytes()[index + 1..]
            .starts_with(hashes)
            .then_some(index + 1 + hashes.len())
    });

    let Some(terminator_end) = terminator_end else {
        // Advance to EOF so that the error handling logic will correctly return `UnexpectedCharOrEndOfInput::EndOfInput`.
        logos_lexer.bump(remainder.len());
        position_callback(logos_lexer);

        return None;
    };

    logos_lexer.bump(terminator_end);
    Some(tagged_token_callback(logos_lexer))
}

#[derive(Debug, Clone, Copy)]
enum LogosTokenToTokenConversionError {
    Skipped,
    IllegalCharacter(TextPosition),
}

impl<'a> TryFrom<LogosToken<'a>> for Token<'a> {
    type Error = LogosTokenToTokenConversionError;

    fn try_from(logos_token: LogosToken<'a>) -> Result<Self, Self::Error> {
        Ok(match logos_token {
            LogosToken::StartKw(token) => Token::StartKw(token),
            LogosToken::StructKw(token) => Token::StructKw(token),
            LogosToken::EnumKw(token) => Token::EnumKw(token),
            LogosToken::ManualKw(token) => Token::ManualKw(token),
            LogosToken::TerminalKw(token) => Token::TerminalKw(token),
            LogosToken::BridgeKw(token) => Token::BridgeKw(token),
            LogosToken::Underscore(token) => Token::Underscore(token),
            LogosToken::LParen(token) => Token::LParen(token),
            LogosToken::RParen(token) => Token::RParen(token),
            LogosToken::LSquare(token) => Token::LSquare(token),
            LogosToken::RSquare(token) => Token::RSquare(token),
            LogosToken::LCurly(token) => Token::LCurly(token),
            LogosToken::RCurly(token) => Token::RCurly(token),
            LogosToken::Comma(token) => Token::Comma(token),
            LogosToken::DoubleColon(token) => Token::DoubleColon(token),
            LogosToken::Colon(token) => Token::Colon(token),
            LogosToken::Equal(token) => Token::Equal(token),
            LogosToken::Octothorpe(token) => Token::Octothorpe(token),
            LogosToken::UpperName(token) => Token::UpperName(token),
            LogosToken::LowerName(token) => Token::LowerName(token),
            LogosToken::DollarUpperName(token) => Token::DollarUpperName(token),
            LogosToken::AtUpperName(token) => Token::AtUpperName(token),
            LogosToken::AtLowerName(token) => Token::AtLowerName(token),
            LogosToken::RustStringLiteral(token) => Token::RustStringLiteral(token),

            LogosToken::SingleLineComment(_) | LogosToken::Whitespace(_) => {
                return Err(LogosTokenToTokenConversionError::Skipped);
            }

            LogosToken::IllegalCharacter(position) => {
                return Err(LogosTokenToTokenConversionError::IllegalCharacter(position));
            }
        })
    }
}

mod lookarounds {
    use super::*;

    pub fn validate_lookarounds(token: Token<'_>, source: &str) -> Result<(), LexError> {
        let fragment = match token {
            Token::StartKw(token) => token.raw,
            Token::StructKw(token) => token.raw,
            Token::EnumKw(token) => token.raw,
            Token::ManualKw(token) => token.raw,
            Token::TerminalKw(token) => token.raw,
            Token::BridgeKw(token) => token.raw,
            Token::Underscore(token) => token.raw,
            Token::UpperName(token) => token.raw,
            Token::LowerName(token) => token.raw,
            Token::DollarUpperName(token) => token.raw,
            Token::AtUpperName(token) => token.raw,
            Token::AtLowerName(token) => token.raw,

            Token::LParen(_)
            | Token::RParen(_)
            | Token::LSquare(_)
            | Token::RSquare(_)
            | Token::LCurly(_)
            | Token::RCurly(_)
            | Token::Comma(_)
            | Token::DoubleColon(_)
            | Token::Colon(_)
            | Token::Equal(_)
            | Token::Octothorpe(_)
            | Token::RustStringLiteral(_) => return Ok(()),
        };

        let range = SourceFile::new(source)
            .substr_range(fragment)
            .expect("Lexed token must be a fragment of its source file");
        let prefix = &source[..range.start];
        let prev_char = prefix.chars().last();
        if is_namelike_char(prev_char) {
            return Err(LexError {
                character: UnexpectedCharOrEndOfInput::Char(fragment.raw().chars().next().unwrap()),
                position: TextPosition {
                    byte_index: range.start,
                },
            });
        }

        let suffix = &source[range.end..];
        let next_char = suffix.chars().next();
        if is_namelike_char(next_char) {
            return Err(LexError {
                character: UnexpectedCharOrEndOfInput::Char(next_char.unwrap()),
                position: TextPosition {
                    byte_index: range.end,
                },
            });
        }

        Ok(())
    }

    fn is_namelike_char(o: Option<char>) -> bool {
        o.is_some_and(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$' || c == '@')
    }
}

use crate::internal_prelude::*;

use logos::{Lexer as LogosLexer, Logos};

pub fn lex(source: &str) -> impl Iterator<Item = Result<Token, LexError>> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnexpectedCharOrEndOfInput {
    EndOfInput,
    Char(char),
}

struct Lexer<'a> {
    logos_lexer: LogosLexer<'a, LogosToken>,
    is_done: bool,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, LexError>;

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
    fn get_next_without_doneness_check(&mut self) -> Option<Result<Token, LexError>> {
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
    ) -> Option<Result<Token, LexError>> {
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

                Err(LogosTokenToTokenConversionError::IllegalCharacter(span)) => {
                    let c = self.logos_lexer.source()[span.start.byte_index..]
                        .chars()
                        .next()
                        .unwrap();
                    return Some(Err(LexError {
                        character: UnexpectedCharOrEndOfInput::Char(c),
                        position: span.start,
                    }));
                }
            };

            return Some(Ok(converted));
        }
    }

    fn validate_lookarounds(&self, token: Token) -> Result<(), LexError> {
        lookarounds::validate_lookarounds(token, self.logos_lexer.source())
    }
}

struct LogosExtras {
    pub position: TextPosition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Logos)]
#[logos(extras = LogosExtras)]
enum LogosToken {
    #[token("start", span_callback)]
    StartKw(Span),

    #[token("struct", span_callback)]
    StructKw(Span),

    #[token("enum", span_callback)]
    EnumKw(Span),

    #[token("custom", span_callback)]
    CustomKw(Span),

    #[token("terminal", span_callback)]
    TerminalKw(Span),

    #[token("_", span_callback)]
    Underscore(Span),

    #[token("(", span_callback)]
    LParen(Span),

    #[token(")", span_callback)]
    RParen(Span),

    #[token("[", span_callback)]
    LSquare(Span),

    #[token("]", span_callback)]
    RSquare(Span),

    #[token("{", span_callback)]
    LCurly(Span),

    #[token("}", span_callback)]
    RCurly(Span),

    #[token(",", span_callback)]
    Comma(Span),

    #[token("::", span_callback)]
    DoubleColon(Span),

    #[token(":", span_callback)]
    Colon(Span),

    #[token("=", span_callback)]
    Equal(Span),

    #[token("#", span_callback)]
    Octothorpe(Span),

    #[regex(r#"[A-Z][a-zA-Z0-9_]*"#, span_callback)]
    UpperName(Span),

    #[regex(r#"[a-z][a-zA-Z0-9_]*"#, span_callback)]
    LowerName(Span),

    #[regex(r#"\$[A-Z][a-zA-Z0-9_]*"#, span_callback)]
    DollarUpperName(Span),

    #[regex(r#"rust#*""#, rust_string_span_callback)]
    RustStringLiteral(Span),

    #[regex(r"\/\/[^\n]*", span_callback, allow_greedy = true)]
    SingleLineComment(Span),

    #[regex(r"[ \t\r\n\f]+", span_callback)]
    Whitespace(Span),

    #[regex(r".", span_callback, priority = 0)]
    IllegalCharacter(Span),
}

fn span_callback(logos_lexer: &mut LogosLexer<LogosToken>) -> Span {
    let start = logos_lexer.extras.position;
    logos_lexer.extras.position.byte_index += logos_lexer.slice().len();
    let end = logos_lexer.extras.position;
    Span { start, end }
}

fn rust_string_span_callback(logos_lexer: &mut LogosLexer<LogosToken>) -> Option<Span> {
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
        span_callback(logos_lexer);

        return None;
    };

    logos_lexer.bump(terminator_end);
    let span = span_callback(logos_lexer);
    Some(span)
}

#[derive(Debug, Clone, Copy)]
enum LogosTokenToTokenConversionError {
    Skipped,
    IllegalCharacter(Span),
}

impl TryFrom<LogosToken> for Token {
    type Error = LogosTokenToTokenConversionError;

    fn try_from(logos_token: LogosToken) -> Result<Self, Self::Error> {
        Ok(match logos_token {
            LogosToken::StartKw(span) => Token {
                kind: TokenKind::StartKw,
                span,
            },
            LogosToken::StructKw(span) => Token {
                kind: TokenKind::StructKw,
                span,
            },
            LogosToken::EnumKw(span) => Token {
                kind: TokenKind::EnumKw,
                span,
            },
            LogosToken::CustomKw(span) => Token {
                kind: TokenKind::CustomKw,
                span,
            },
            LogosToken::TerminalKw(span) => Token {
                kind: TokenKind::TerminalKw,
                span,
            },
            LogosToken::Underscore(span) => Token {
                kind: TokenKind::Underscore,
                span,
            },
            LogosToken::LParen(span) => Token {
                kind: TokenKind::LParen,
                span,
            },
            LogosToken::RParen(span) => Token {
                kind: TokenKind::RParen,
                span,
            },
            LogosToken::LSquare(span) => Token {
                kind: TokenKind::LSquare,
                span,
            },
            LogosToken::RSquare(span) => Token {
                kind: TokenKind::RSquare,
                span,
            },
            LogosToken::LCurly(span) => Token {
                kind: TokenKind::LCurly,
                span,
            },
            LogosToken::RCurly(span) => Token {
                kind: TokenKind::RCurly,
                span,
            },
            LogosToken::Comma(span) => Token {
                kind: TokenKind::Comma,
                span,
            },
            LogosToken::DoubleColon(span) => Token {
                kind: TokenKind::DoubleColon,
                span,
            },
            LogosToken::Colon(span) => Token {
                kind: TokenKind::Colon,
                span,
            },
            LogosToken::Equal(span) => Token {
                kind: TokenKind::Equal,
                span,
            },
            LogosToken::Octothorpe(span) => Token {
                kind: TokenKind::Octothorpe,
                span,
            },
            LogosToken::UpperName(span) => Token {
                kind: TokenKind::UpperName,
                span,
            },
            LogosToken::LowerName(span) => Token {
                kind: TokenKind::LowerName,
                span,
            },
            LogosToken::DollarUpperName(span) => Token {
                kind: TokenKind::DollarUpperName,
                span,
            },
            LogosToken::RustStringLiteral(span) => Token {
                kind: TokenKind::RustStringLiteral,
                span,
            },

            LogosToken::SingleLineComment(_) | LogosToken::Whitespace(_) => {
                return Err(LogosTokenToTokenConversionError::Skipped);
            }

            LogosToken::IllegalCharacter(span) => {
                return Err(LogosTokenToTokenConversionError::IllegalCharacter(span));
            }
        })
    }
}

mod lookarounds {
    use super::*;

    pub fn validate_lookarounds(token: Token, source: &str) -> Result<(), LexError> {
        let Token { kind, span } = token;

        match kind {
            TokenKind::StartKw
            | TokenKind::StructKw
            | TokenKind::EnumKw
            | TokenKind::CustomKw
            | TokenKind::TerminalKw
            | TokenKind::Underscore
            | TokenKind::UpperName
            | TokenKind::LowerName
            | TokenKind::DollarUpperName => {
                let prefix = &source[..span.start.byte_index];
                let prev_char = prefix.chars().last();
                if is_namelike_char(prev_char) {
                    return Err(LexError {
                        character: UnexpectedCharOrEndOfInput::Char(
                            source[span.start.byte_index..].chars().next().unwrap(),
                        ),
                        position: span.start,
                    });
                }

                let suffix = &source[span.end.byte_index..];
                let next_char = suffix.chars().next();
                if is_namelike_char(next_char) {
                    return Err(LexError {
                        character: UnexpectedCharOrEndOfInput::Char(next_char.unwrap()),
                        position: span.end,
                    });
                }

                Ok(())
            }

            TokenKind::LParen
            | TokenKind::RParen
            | TokenKind::LSquare
            | TokenKind::RSquare
            | TokenKind::LCurly
            | TokenKind::RCurly
            | TokenKind::Comma
            | TokenKind::DoubleColon
            | TokenKind::Colon
            | TokenKind::Equal
            | TokenKind::Octothorpe
            | TokenKind::RustStringLiteral => Ok(()),
        }
    }

    fn is_namelike_char(o: Option<char>) -> bool {
        o.is_some_and(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$')
    }
}

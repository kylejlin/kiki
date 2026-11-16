use crate::internal_prelude::*;

mod azuki_bridge;
include!(concat!(env!("OUT_DIR"), "/cst/parser.rs"));

pub use parser::non_terminals::{
    Enum, EnumVariant, Fieldset, File, FileItem, GeneratedTerminal, GrammarSymbolOrSelf,
    ManualNonTerminal, ManualNonTerminalVariant, ManualTerminal, Name, NamedField, NamedFieldset,
    Path, Struct, TerminalEnumVariant, TupleField, TupleFieldset, UnderscoreOrLowerName,
};

#[derive(Debug, Clone)]
pub struct ParseError<'a> {
    /// This is `None` if and only if there was an unexpected end of input.
    pub unexpected_token: Option<Token<'a>>,
}

/// If there is a lexer error, this returns `Err(Err(LexError))`.
/// If there are no lexer errors, but there is a parser error, this returns `Err(Ok(ParseError))`.
/// Otherwise, this returns `Ok`.
pub fn parse<'a>(
    file_source: FileSource<'a>,
) -> Result<File<'a>, Result<ParseError<'a>, LexError>> {
    let mut lexer = lex(file_source.raw());
    let mut lex_error = None;

    let tokens = lexer
        .by_ref()
        .map_while(|result| match result {
            Ok(token) => Some(token),
            Err(error) => {
                lex_error = Some(error);
                None
            }
        })
        .fuse();

    let parsed = parser::parse(tokens);

    if let Some(error) = lex_error.or_else(|| lexer.find_map(Result::err)) {
        return Err(Err(error));
    }

    parsed.map_err(|unexpected_token| Ok(ParseError { unexpected_token }))
}

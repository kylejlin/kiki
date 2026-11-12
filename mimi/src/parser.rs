use crate::internal_prelude::*;

/// If the parser encounters an unexpected token `t`, it will return `Err(Some(t))`.
/// If the parser encounters an unexpected end of input, it will return `Err(None)`.
pub fn parse<I>(tokens: I) -> Result<m::File, Option<Token>>
where
    I: IntoIterator<Item = Token>,
{
    let tokens = tokens.into_iter().map(TransposedToken::from);
    crate::raw_parser::parse(tokens)
        .map(m::File::from)
        .map_err(|o| o.map(Token::from))
}

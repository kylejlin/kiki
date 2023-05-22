#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser);

mod ast;
mod cst;

mod ast_to_rust;
mod cst_to_ast;

#[derive(Debug)]
pub enum KikiErr {
    Parse(ByteIndex, String, ByteIndex),
}

impl KikiErr {
    fn parse_err(
        src: &str,
        err: lalrpop_util::ParseError<usize, lalrpop_util::lexer::Token<'_>, &'static str>,
    ) -> Self {
        use lalrpop_util::ParseError::*;
        match err {
            InvalidToken { location } => KikiErr::Parse(
                ByteIndex(location),
                src[location..].to_owned(),
                ByteIndex(src.len()),
            ),
            UnrecognizedEOF { location, expected } => {
                KikiErr::Parse(ByteIndex(location), "".to_owned(), ByteIndex(src.len()))
            }
            UnrecognizedToken {
                token: (start, token, end),
                expected,
            } => KikiErr::Parse(ByteIndex(start), src[start..end].to_owned(), ByteIndex(end)),
            ExtraToken {
                token: (start, token, end),
            } => KikiErr::Parse(ByteIndex(start), src[start..end].to_owned(), ByteIndex(end)),
            User { error } => KikiErr::Parse(ByteIndex(0), "".to_owned(), ByteIndex(0)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RustSrc(pub String);

pub fn generate(src: &str) -> Result<RustSrc, KikiErr> {
    let cst = parser::FileParser::new()
        .parse(src)
        .map_err(|e| KikiErr::parse_err(src, e))?;
    let ast: ast::File = cst.into();
    (&ast).try_into()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ByteIndex(pub usize);

#[cfg(test)]
mod tests;

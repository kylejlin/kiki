use crate::data::{ByteIndex, KikiErr};

impl KikiErr {
    pub(crate) fn parse_err(
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
            UnrecognizedEOF { location, .. } => {
                KikiErr::Parse(ByteIndex(location), "".to_owned(), ByteIndex(src.len()))
            }
            UnrecognizedToken {
                token: (start, _, end),
                ..
            } => KikiErr::Parse(ByteIndex(start), src[start..end].to_owned(), ByteIndex(end)),
            ExtraToken {
                token: (start, _, end),
            } => KikiErr::Parse(ByteIndex(start), src[start..end].to_owned(), ByteIndex(end)),
            User { .. } => KikiErr::Parse(ByteIndex(0), "".to_owned(), ByteIndex(0)),
        }
    }
}

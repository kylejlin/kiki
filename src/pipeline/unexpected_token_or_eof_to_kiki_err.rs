use crate::data::{token::Token, *};

pub fn unexpected_token_or_eof_to_kiki_err(unexpected: Option<&Token>, src: &str) -> KikiErr {
    let Some(token) = unexpected else {
        return get_unexpected_eof_err(src);
    };

    let start = token.byte_index();
    let end = ByteIndex(start.0 + token.content_len());
    let content = src[start.0..end.0].to_string();
    KikiErr::Parse(start, content, end)
}

fn get_unexpected_eof_err(src: &str) -> KikiErr {
    KikiErr::Parse(ByteIndex(src.len()), "".to_string(), ByteIndex(src.len()))
}

impl Token {
    fn byte_index(&self) -> ByteIndex {
        todo!()
    }

    fn content_len(&self) -> usize {
        todo!()
    }
}

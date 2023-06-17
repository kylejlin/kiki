use crate::data::{token::Token, *};

pub fn unexpected_token_or_eof_to_kiki_err(unexpected: Option<&Token>, src: &str) -> KikiErr {
    let Some(token) = unexpected else {
        return get_unexpected_eof_err(src);
    };

    let start = token.start();
    let end = ByteIndex(start.0 + token.content_len());
    let content = src[start.0..end.0].to_string();
    KikiErr::Parse(start, content, end)
}

fn get_unexpected_eof_err(src: &str) -> KikiErr {
    KikiErr::Parse(ByteIndex(src.len()), "".to_string(), ByteIndex(src.len()))
}

impl Token {
    fn start(&self) -> ByteIndex {
        match self {
            Token::Underscore(start) => *start,
            Token::Ident(ident) => ident.position,
            Token::TerminalIdent(ident) => ByteIndex(ident.dollarless_position.0 - "$".len()),
            Token::OuterAttribute(attr) => attr.position,
            Token::StartKw(start) => *start,
            Token::StructKw(start) => *start,
            Token::EnumKw(start) => *start,
            Token::TerminalKw(start) => *start,
            Token::Colon(start) => *start,
            Token::DoubleColon(start) => *start,
            Token::Comma(start) => *start,
            Token::LParen(start) => *start,
            Token::RParen(start) => *start,
            Token::LCurly(start) => *start,
            Token::RCurly(start) => *start,
            Token::LAngle(start) => *start,
            Token::RAngle(start) => *start,
        }
    }

    fn content_len(&self) -> usize {
        match self {
            Token::Underscore(_) => "_".len(),
            Token::Ident(ident) => ident.name.len(),
            Token::TerminalIdent(ident) => "$".len() + ident.name.raw().len(),
            Token::OuterAttribute(attr) => attr.src.len(),
            Token::StartKw(_) => "start".len(),
            Token::StructKw(_) => "struct".len(),
            Token::EnumKw(_) => "enum".len(),
            Token::TerminalKw(_) => "terminal".len(),
            Token::Colon(_) => ":".len(),
            Token::DoubleColon(_) => "::".len(),
            Token::Comma(_) => ",".len(),
            Token::LParen(_) => "(".len(),
            Token::RParen(_) => ")".len(),
            Token::LCurly(_) => "{".len(),
            Token::RCurly(_) => "}".len(),
            Token::LAngle(_) => "<".len(),
            Token::RAngle(_) => ">".len(),
        }
    }
}

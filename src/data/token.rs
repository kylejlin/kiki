pub use crate::data::*;

pub use crate::pipeline::parser::Token;

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Underscore(a), Token::Underscore(b)) => a == b,
            (Token::Ident(a), Token::Ident(b)) => a == b,
            (Token::TerminalIdent(a), Token::TerminalIdent(b)) => a == b,
            (Token::StartKw(a), Token::StartKw(b)) => a == b,
            (Token::StructKw(a), Token::StructKw(b)) => a == b,
            (Token::EnumKw(a), Token::EnumKw(b)) => a == b,
            (Token::TerminalKw(a), Token::TerminalKw(b)) => a == b,
            (Token::Colon(a), Token::Colon(b)) => a == b,
            (Token::DoubleColon(a), Token::DoubleColon(b)) => a == b,
            (Token::Comma(a), Token::Comma(b)) => a == b,
            (Token::LParen(a), Token::LParen(b)) => a == b,
            (Token::RParen(a), Token::RParen(b)) => a == b,
            (Token::LCurly(a), Token::LCurly(b)) => a == b,
            (Token::RCurly(a), Token::RCurly(b)) => a == b,
            (Token::LAngle(a), Token::LAngle(b)) => a == b,
            (Token::RAngle(a), Token::RAngle(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for Token {}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Ident {
    pub name: String,
    pub position: ByteIndex,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TerminalIdent {
    pub name: DollarlessTerminalName,
    pub dollarless_position: ByteIndex,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Attribute {
    pub src: String,
    pub position: ByteIndex,
}

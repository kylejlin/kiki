use crate::data::token::*;

pub fn positionless_ident(s: &str) -> Ident {
    Ident {
        name: s.to_owned(),
        position: ByteIndex(0),
    }
}

pub fn positionless_terminal_ident(s: &DollarlessTerminalName) -> TerminalIdent {
    TerminalIdent {
        name: s.clone(),
        dollarless_position: ByteIndex(1),
    }
}

pub fn positionless_attribute(s: &str) -> Attribute {
    Attribute {
        src: s.to_owned(),
        position: ByteIndex(0),
    }
}

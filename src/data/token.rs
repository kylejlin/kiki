pub use crate::data::*;

pub use crate::pipeline::parser::Token;

#[derive(Clone, Debug)]
pub struct Ident {
    pub name: String,
    pub position: ByteIndex,
}

#[derive(Clone, Debug)]
pub struct TerminalIdent {
    pub name: DollarlessTerminalName,
    pub dollarless_position: ByteIndex,
}

pub use crate::data::*;

pub use crate::pipeline::parser::Token;

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

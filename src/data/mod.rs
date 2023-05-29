pub mod ast;
pub mod cst;
pub mod machine;
pub mod table;
pub mod validated_file;

#[derive(Debug)]
pub enum KikiErr {
    Parse(ByteIndex, String, ByteIndex),
    NoStartSymbol,
    MultipleStartSymbols(Vec<ByteIndex>),
    NoTerminalEnum,
    MultipleTerminalEnums(Vec<ByteIndex>),
    SymbolsFirstLetteNotCapitalized(ByteIndex),
    DuplicateTerminalVariants(String, ByteIndex, ByteIndex),
    DuplicateNonterminals(String, ByteIndex, ByteIndex),
    UndefinedNonterminal(String, ByteIndex),
    UndefinedTerminal(DollarlessTerminalName, ByteIndex),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RustSrc(pub String);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ByteIndex(pub usize);

pub use crate::logic::oset::Oset;

use validated_file::DollarlessTerminalName;

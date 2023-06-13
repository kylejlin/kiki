pub mod ast;
pub mod cst;
pub mod index_updater;
pub mod machine;
pub mod oset;
pub mod table;
pub mod token;
pub mod unnormalized_machine;
pub mod validated_file;

pub use index_updater::*;
pub use oset::*;

#[derive(Debug)]
pub enum KikiErr {
    Lex(
        ByteIndex,
        /// If the lexer encounters an unexpected character `c`,
        /// this is `Some(c)`.
        /// If the lexer encounters an unexpected end of input,
        /// this is `None`.
        Option<char>,
    ),
    Parse(ByteIndex, String, ByteIndex),
    NoStartSymbol,
    MultipleStartSymbols(Vec<ByteIndex>),
    NoTerminalEnum,
    MultipleTerminalEnums(Vec<ByteIndex>),
    SymbolOrTerminalEnumNameFirstLetterNotUppercase(ByteIndex),
    FieldFirstLetterNotLowercase(ByteIndex),
    NameClash(String, ByteIndex, ByteIndex),
    NonterminalEnumVariantNameClash(String, ByteIndex, ByteIndex),
    NonterminalEnumVariantSymbolSequenceClash(Vec<Symbol>, ByteIndex, ByteIndex),
    UndefinedNonterminal(String, ByteIndex),
    UndefinedTerminal(DollarlessTerminalName, ByteIndex),
    TableConflict(Box<TableConflictErr>),
}

#[derive(Debug)]
pub struct TableConflictErr {
    pub state_index: machine::StateIndex,
    pub items: (machine::StateItem, machine::StateItem),
    pub file: validated_file::File,
    pub machine: machine::Machine,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RustSrc(pub String);

impl RustSrc {
    pub fn as_ref(&self) -> RustSrcRef {
        RustSrcRef(&self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RustSrcRef<'a>(pub &'a str);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ByteIndex(pub usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DollarlessTerminalName(String);

impl DollarlessTerminalName {
    pub fn remove_dollars(name: &str) -> Self {
        Self(name.chars().filter(|c| *c != '$').collect())
    }

    pub fn raw(&self) -> &str {
        &self.0
    }
}

impl ToString for DollarlessTerminalName {
    fn to_string(&self) -> String {
        self.raw().to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Symbol {
    Terminal(DollarlessTerminalName),
    Nonterminal(String),
}

impl From<cst::IdentOrTerminalIdent> for Symbol {
    fn from(ident: cst::IdentOrTerminalIdent) -> Self {
        match ident {
            cst::IdentOrTerminalIdent::Ident(ident) => Symbol::Nonterminal(ident.name),
            cst::IdentOrTerminalIdent::Terminal(ident) => Symbol::Terminal(ident.name),
        }
    }
}

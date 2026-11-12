use crate::internal_prelude::*;

pub struct File {
    pub items: Vec<FileItem>,
}

pub enum FileItem {
    // TODO
}

pub struct Struct {
    pub prefix: Option<span_of::RustStringLiteral>,
    pub name: span_of::UpperName,
    pub fieldset: Fieldset,
}

// TODO: Enum, Manual, TerminalEnum

pub enum Fieldset {
    Empty,
    Named(NamedFieldset),
    // TODO: Tuple(TupleFieldset),
}

pub struct NamedFieldset {
    pub fields: Vec<NamedField>,
}

pub struct NamedField {
    pub name: UnderscoreOrLowerName,
    pub symbol: GrammarSymbol,
}

// TODO: TupleFieldset, TupleField, EnumVariant, ManualVariant, TerminalEnumVariant

// TODO: Path

pub use crate::raw_parser::{GrammarSymbol, Name, UnderscoreOrLowerName};

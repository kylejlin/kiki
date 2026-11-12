use crate::internal_prelude::*;

pub struct File {
    pub items: Vec<FileItem>,
}

pub enum FileItem {
    Start(span_of::UpperName),
    Struct(Struct),
    Enum(Enum),
    Manual(Manual),
    Terminal(TerminalEnum),
    Bridge(Path),
}

pub struct Struct {
    pub prefix: Option<span_of::RustStringLiteral>,
    pub name: span_of::UpperName,
    pub fieldset: Fieldset,
}

pub struct Enum {
    pub prefix: Option<span_of::RustStringLiteral>,
    pub name: span_of::UpperName,
    pub variants: Vec<EnumVariant>,
}

pub struct Manual {
    pub name: span_of::UpperName,
    pub override_: Option<span_of::AtUpperName>,
    pub variants: Vec<ManualVariant>,
}

pub struct TerminalEnum {
    pub prefix: Option<span_of::RustStringLiteral>,
    pub name: span_of::UpperName,
    pub variants: Vec<TerminalEnumVariant>,
}

pub enum Fieldset {
    Empty,
    Named(NamedFieldset),
    Tuple(TupleFieldset),
}

pub struct NamedFieldset {
    pub fields: Vec<NamedField>,
}

pub struct NamedField {
    pub name: UnderscoreOrLowerName,
    pub symbol: GrammarSymbol,
}

pub struct TupleFieldset {
    pub fields: Vec<TupleField>,
}

pub enum TupleField {
    Used(GrammarSymbol),
    Skipped(GrammarSymbol),
}

pub struct EnumVariant {
    pub name: span_of::UpperName,
    pub fieldset: Fieldset,
}

pub struct ManualVariant {
    pub function: span_of::AtLowerName,
    pub inputs: Vec<TupleField>,
}

pub struct TerminalEnumVariant {
    pub name: span_of::DollarUpperName,
    pub override_: Option<span_of::AtUpperName>,
}

pub type Path = Vec<Name>;

pub use crate::raw_parser::{GrammarSymbol, Name, UnderscoreOrLowerName};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SigilFreeStr<'a> {
    raw: &'a str,
}
impl Debug for SigilFreeStr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.raw.fmt(f)
    }
}
impl<'a> SigilFreeStr<'a> {
    pub fn new(raw: &'a str) -> Option<Self> {
        if raw.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            Some(Self { raw })
        } else {
            None
        }
    }

    pub const fn raw(self) -> &'a str {
        self.raw
    }
}

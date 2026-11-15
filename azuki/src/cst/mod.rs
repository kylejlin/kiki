use crate::internal_prelude::*;

mod kiki_bridge;
mod massage;
mod raw_parser;

#[derive(Debug)]
pub struct File<'a> {
    pub items: Vec<FileItem<'a>>,
}

#[derive(Debug)]
pub enum FileItem<'a> {
    Start(token::UpperName<'a>),
    Struct(Struct<'a>),
    Enum(Enum<'a>),
    Manual(Manual<'a>),
    Terminal(TerminalEnum<'a>),
    Bridge(Path<'a>),
}

#[derive(Debug)]
pub struct Struct<'a> {
    pub prefix: Option<token::RustStringLiteral<'a>>,
    pub name: token::UpperName<'a>,
    pub fieldset: Fieldset<'a>,
}

#[derive(Debug)]
pub struct Enum<'a> {
    pub prefix: Option<token::RustStringLiteral<'a>>,
    pub name: token::UpperName<'a>,
    pub variants: Vec<EnumVariant<'a>>,
}

#[derive(Debug)]
pub struct Manual<'a> {
    pub name: token::UpperName<'a>,
    pub override_: Option<token::AtUpperName<'a>>,
    pub lifetimes: Vec<token::SingleQuoteLowerName<'a>>,
    pub variants: Vec<ManualVariant<'a>>,
}

#[derive(Debug)]
pub struct TerminalEnum<'a> {
    pub prefix: Option<token::RustStringLiteral<'a>>,
    pub name: token::UpperName<'a>,
    pub global_lifetimes: Vec<token::SingleQuoteLowerName<'a>>,
    pub variants: Vec<TerminalEnumVariant<'a>>,
}

#[derive(Debug)]
pub enum Fieldset<'a> {
    Empty,
    Named(NamedFieldset<'a>),
    Tuple(TupleFieldset<'a>),
}

#[derive(Debug)]
pub struct NamedFieldset<'a> {
    pub fields: Vec<NamedField<'a>>,
}

#[derive(Debug)]
pub struct NamedField<'a> {
    pub name: UnderscoreOrLowerName<'a>,
    pub symbol: GrammarSymbolOrSelf<'a>,
}

#[derive(Debug)]
pub struct TupleFieldset<'a> {
    pub fields: Vec<TupleField<'a>>,
}

#[derive(Debug)]
pub enum TupleField<'a> {
    Used(GrammarSymbolOrSelf<'a>),
    Skipped(GrammarSymbolOrSelf<'a>),
}

#[derive(Debug)]
pub struct EnumVariant<'a> {
    pub name: token::UpperName<'a>,
    pub fieldset: Fieldset<'a>,
}

#[derive(Debug)]
pub struct ManualVariant<'a> {
    pub function: token::AtLowerName<'a>,
    pub inputs: Vec<TupleField<'a>>,
}

#[derive(Debug)]
pub struct TerminalEnumVariant<'a> {
    pub name: token::DollarUpperName<'a>,
    pub override_: Option<token::AtUpperName<'a>>,
    pub lifetimes: Vec<token::SingleQuoteLowerName<'a>>,
}

pub type Path<'a> = Vec<Name<'a>>;

#[derive(Debug)]
pub enum Name<'a> {
    Upper(token::UpperName<'a>),
    Lower(token::LowerName<'a>),
}

#[derive(Debug)]
pub enum UnderscoreOrLowerName<'a> {
    Underscore(token::Underscore<'a>),
    Lower(token::LowerName<'a>),
}

#[derive(Debug)]
pub enum GrammarSymbolOrSelf<'a> {
    Upper(token::UpperName<'a>),
    DollarUpper(token::DollarUpperName<'a>),
    Self_(token::UpperSelfKw<'a>),
}

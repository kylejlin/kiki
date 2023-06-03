#[derive(Clone, Debug)]
pub struct File {
    pub items: Vec<FileItem>,
}

#[derive(Clone, Debug)]
pub enum FileItem {
    Start(Ident),
    Struct(Struct),
    Enum(Enum),
    Terminal(TerminalEnum),
}

#[derive(Clone, Debug)]
pub struct Struct {
    pub name: Ident,
    pub fieldset: Fieldset,
}

#[derive(Clone, Debug)]
pub struct Enum {
    pub name: Ident,
    pub variants: Vec<EnumVariant>,
}

#[derive(Clone, Debug)]
pub struct TerminalEnum {
    pub name: Ident,
    pub variants: Vec<TerminalEnumVariant>,
}

#[derive(Clone, Debug)]
pub enum Fieldset {
    Empty,
    Named(NamedFieldset),
    Tuple(TupleFieldset),
}

impl Fieldset {
    pub fn len(&self) -> usize {
        match self {
            Fieldset::Empty => 0,
            Fieldset::Named(named) => named.fields.len(),
            Fieldset::Tuple(tuple) => tuple.fields.len(),
        }
    }

    pub fn get_symbol_ident(&self, i: usize) -> &IdentOrTerminalIdent {
        match self {
            Fieldset::Empty => panic!("Called Fieldset::get_symbol_ident on Fieldset::Empty"),
            Fieldset::Named(named) => &named.fields[i].symbol,
            Fieldset::Tuple(tuple) => &tuple.fields[i].symbol(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct NamedFieldset {
    pub fields: Vec<NamedField>,
}

#[derive(Clone, Debug)]
pub struct NamedField {
    pub name: IdentOrUnderscore,
    pub symbol: IdentOrTerminalIdent,
}

#[derive(Clone, Debug)]
pub struct TupleFieldset {
    pub fields: Vec<TupleField>,
}

#[derive(Clone, Debug)]
pub enum TupleField {
    Used(IdentOrTerminalIdent),
    Skipped(IdentOrTerminalIdent),
}

impl TupleField {
    pub fn symbol(&self) -> &IdentOrTerminalIdent {
        match self {
            TupleField::Used(symbol) => symbol,
            TupleField::Skipped(symbol) => symbol,
        }
    }
}

#[derive(Clone, Debug)]
pub struct EnumVariant {
    pub name: Ident,
    pub fieldset: Fieldset,
}

#[derive(Clone, Debug)]
pub struct TerminalEnumVariant {
    pub name: TerminalIdent,
    pub type_: Type,
}

#[derive(Clone, Debug)]
pub enum Type {
    Unit,
    Path(Vec<Ident>),
    Complex(Box<ComplexType>),
}

#[derive(Clone, Debug)]
pub struct ComplexType {
    pub callee: Vec<Ident>,
    pub args: Vec<Type>,
}

pub use crate::data::cst::{Ident, IdentOrTerminalIdent, IdentOrUnderscore, TerminalIdent, Token};

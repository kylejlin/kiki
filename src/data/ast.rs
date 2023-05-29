#[derive(Clone, Debug)]
pub struct File {
    pub items: Vec<Item>,
}

#[derive(Clone, Debug)]
pub enum Item {
    Start(Ident),
    Struct(StructDef),
    Enum(EnumDef),
    Terminal(TerminalDef),
}

#[derive(Clone, Debug)]
pub struct StructDef {
    pub name: Ident,
    pub fieldset: Fieldset,
}

#[derive(Clone, Debug)]
pub struct EnumDef {
    pub name: Ident,
    pub variants: Vec<EnumVariant>,
}

#[derive(Clone, Debug)]
pub struct TerminalDef {
    pub name: Ident,
    pub variants: Vec<TerminalVariant>,
}

#[derive(Clone, Debug)]
pub enum Fieldset {
    Empty,
    Named(NamedFieldset),
    Tuple(TupleFieldset),
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

#[derive(Clone, Debug)]
pub struct EnumVariant {
    pub name: Ident,
    pub fieldset: Fieldset,
}

#[derive(Clone, Debug)]
pub struct TerminalVariant {
    pub name: TerminalIdent,
    pub type_: Type,
}

#[derive(Clone, Debug)]
pub enum Type {
    Unit,
    Path(Path),
    Complex(Box<ComplexType>),
}

#[derive(Clone, Debug)]
pub enum Path {
    One(Ident),
    Cons(Box<Path>, Ident),
}

#[derive(Clone, Debug)]
pub struct ComplexType {
    pub callee: Type,
    pub types: Vec<Type>,
}

pub use crate::data::cst::{Ident, IdentOrTerminalIdent, IdentOrUnderscore, TerminalIdent, Token};

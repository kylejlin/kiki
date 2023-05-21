#[derive(Clone, Debug)]
pub struct File {
    pub items: OptItems,
}

#[derive(Clone, Debug)]
pub enum OptItems {
    Nil,
    Cons(Box<OptItems>, Item),
}

#[derive(Clone, Debug)]
pub enum Item {
    Start(Ident),
    Struct(Struct),
    Enum(Enum),
    Terminal(Terminal),
}

#[derive(Clone, Debug)]
pub struct Struct {
    pub name: Ident,
    pub fieldset: Fieldset,
}

#[derive(Clone, Debug)]
pub struct Enum {
    pub name: Ident,
    pub variants: OptEnumVariants,
}

#[derive(Clone, Debug)]
pub struct Terminal {
    pub name: Ident,
    pub variants: OptTerminalVariants,
}

#[derive(Clone, Debug)]
pub enum Fieldset {
    Empty,
    Named(NamedFieldset),
    Tuple(TupleFieldset),
}

#[derive(Clone, Debug)]
pub struct NamedFieldset {
    pub fields: NamedFields,
}

#[derive(Clone, Debug)]
pub enum NamedFields {
    One(NamedField),
    Mul(Box<NamedFields>, NamedField),
}

#[derive(Clone, Debug)]
pub struct NamedField {
    pub name: IdentOrUnderscore,
    pub symbol: IdentOrTerminalIdent,
}

#[derive(Clone, Debug)]
pub struct TupleFieldset {
    pub fields: TupleFields,
}

#[derive(Clone, Debug)]
pub enum TupleFields {
    One(TupleField),
    Cons(Box<TupleFields>, TupleField),
}

#[derive(Clone, Debug)]
pub enum TupleField {
    Used(IdentOrTerminalIdent),
    Skipped(IdentOrTerminalIdent),
}

#[derive(Clone, Debug)]
pub enum OptEnumVariants {
    Nil,
    Cons(Box<OptEnumVariants>, EnumVariant),
}

#[derive(Clone, Debug)]
pub struct EnumVariant {
    pub name: Ident,
    pub fieldset: Fieldset,
}

#[derive(Clone, Debug)]
pub enum OptTerminalVariants {
    Nil,
    Cons(Box<OptTerminalVariants>, TerminalVariant),
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
    pub types: CommaSeparatedTypes,
}

#[derive(Clone, Debug)]
pub enum CommaSeparatedTypes {
    One(Type),
    Cons(Box<CommaSeparatedTypes>, Type),
}

#[derive(Clone, Debug)]
pub enum IdentOrUnderscore {
    Ident(Ident),
    Underscore,
}

#[derive(Clone, Debug)]
pub enum IdentOrTerminalIdent {
    Ident(Ident),
    Terminal(TerminalIdent),
}

#[derive(Clone, Debug)]
pub enum Token {
    Underscore,
    Ident(Ident),
    TerminalIdent(TerminalIdent),

    StartKw,
    StructKw,
    EnumKw,
    TerminalKw,

    Colon,
    DoubleColon,
    Comma,

    LParen,
    RParen,
    LCurly,
    RCurly,
    LAngle,
    RAngle,
}

#[derive(Clone, Debug)]
pub struct Ident {
    pub name: String,
    pub position: ByteIndex,
}

#[derive(Clone, Debug)]
pub struct TerminalIdent {
    pub name: String,
    pub position: ByteIndex,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ByteIndex(pub usize);

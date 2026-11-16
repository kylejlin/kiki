use super::parser::non_terminals as c;

pub use crate::token::*;

pub type Items<'a> = Vec<c::FileItem<'a>>;
pub type OptRustStringLiteral<'a> = Option<RustStringLiteral<'a>>;
pub type NamedFields<'a> = Vec<c::NamedField<'a>>;
pub type TupleFields<'a> = Vec<c::TupleField<'a>>;
pub type EnumVariants<'a> = Vec<c::EnumVariant<'a>>;
pub type ManualNonTerminalVariants<'a> = Vec<c::ManualNonTerminalVariant<'a>>;
pub type TerminalEnumVariants<'a> = Vec<c::TerminalEnumVariant<'a>>;
pub type OptEqualAtUpperName<'a> = Option<AtUpperName<'a>>;
pub type Path<'a> = Vec<c::Name<'a>>;
pub type Lifetimes<'a> = Vec<SingleQuoteLowerName<'a>>;

pub fn vec_empty<T>() -> Vec<T> {
    Vec::new()
}

pub fn vec_one<T>(value: T) -> Vec<T> {
    vec![value]
}

pub fn vec_push<T>(mut values: Vec<T>, value: T) -> Vec<T> {
    values.push(value);
    values
}

pub fn option_none<T>() -> Option<T> {
    None
}

pub fn option_some<T>(value: T) -> Option<T> {
    Some(value)
}

pub fn tuple_fields(fieldset: c::TupleFieldset<'_>) -> TupleFields<'_> {
    let c::TupleFieldset { fields } = fieldset;
    fields
}

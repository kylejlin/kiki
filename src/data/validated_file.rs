use crate::data::ast::*;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct ValidatedFile {
    pub raw: File,
    pub start: String,
    pub terminal_enum: TerminalEnum,
    pub defined_identifiers: HashSet<String>,
}

#[derive(Debug, Clone)]
pub struct TerminalEnum {
    pub name: String,
    pub variants: Vec<TerminalVariant>,
}

#[derive(Debug, Clone)]
pub struct TerminalVariant {
    pub dollarless_name: String,
    pub type_: String,
}

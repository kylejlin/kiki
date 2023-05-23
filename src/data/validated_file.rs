use crate::data::ast::*;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct ValidatedFile {
    pub start: String,
    pub terminal_enum: TerminalEnum,
    pub nonterminals: Vec<Nonterminal>,
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

#[derive(Debug, Clone)]
pub enum Nonterminal {
    Struct(Struct),
    Enum(Enum),
}

impl Nonterminal {
    pub fn name(&self) -> &str {
        match self {
            Nonterminal::Struct(s) => &s.name.name,
            Nonterminal::Enum(e) => &e.name.name,
        }
    }
}

use crate::data::ast::*;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct File {
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

impl TerminalEnum {
    pub fn get_type(&self, variant_name: &DollarlessTerminalName) -> Option<&str> {
        self.variants
            .iter()
            .find(|variant| variant.dollarless_name == *variant_name)
            .map(|variant| -> &str { &variant.type_ })
    }
}

#[derive(Debug, Clone)]
pub struct TerminalVariant {
    pub dollarless_name: DollarlessTerminalName,
    pub type_: String,
}

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

#[derive(Debug, Clone)]
pub enum Nonterminal {
    Struct(StructDef),
    Enum(EnumDef),
}

impl Nonterminal {
    pub fn name(&self) -> &str {
        match self {
            Nonterminal::Struct(s) => &s.name.name,
            Nonterminal::Enum(e) => &e.name.name,
        }
    }
}

use crate::data::ast::*;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct File {
    pub start: String,
    pub terminal_enum: TerminalEnum,
    pub nonterminals: Vec<Nonterminal>,
    pub defined_identifiers: HashSet<String>,
}

impl File {
    pub fn get_rules(&self) -> impl Iterator<Item = Rule> {
        self.nonterminals
            .iter()
            .flat_map(|nonterminal| match nonterminal {
                Nonterminal::Struct(s) => {
                    vec![Rule {
                        constructor_name: ConstructorName::Struct(&s.name.name),
                        fieldset: &s.fieldset,
                    }]
                }
                Nonterminal::Enum(e) => e
                    .variants
                    .iter()
                    .map(|v| {
                        let enum_name = &e.name.name;
                        let variant_name = &v.name.name;
                        Rule {
                            constructor_name: ConstructorName::EnumVariant {
                                enum_name,
                                variant_name,
                            },
                            fieldset: &v.fieldset,
                        }
                    })
                    .collect(),
            })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rule<'a> {
    pub constructor_name: ConstructorName<'a>,
    pub fieldset: &'a Fieldset,
}

#[derive(Debug, Clone, Copy)]
pub enum ConstructorName<'a> {
    Struct(&'a str),
    EnumVariant {
        enum_name: &'a str,
        variant_name: &'a str,
    },
}

impl ConstructorName<'_> {
    pub fn to_string(&self) -> String {
        match self {
            ConstructorName::Struct(name) => name.to_string(),
            ConstructorName::EnumVariant {
                enum_name,
                variant_name,
            } => format!("{enum_name}::{variant_name}"),
        }
    }

    pub fn type_name(&self) -> &str {
        match self {
            ConstructorName::Struct(name) => name,
            ConstructorName::EnumVariant { enum_name, .. } => enum_name,
        }
    }
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

impl ToString for DollarlessTerminalName {
    fn to_string(&self) -> String {
        self.raw().to_string()
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

use crate::data::*;

use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct File {
    pub start: String,
    pub terminal_enum: TerminalEnum,
    pub nonterminals: Vec<Nonterminal>,
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

impl File {
    pub fn get_defined_identifiers(&self) -> HashSet<String> {
        self.get_nonterminal_names()
            .chain(self.get_terminal_enum_variant_names())
            .chain(std::iter::once(self.terminal_enum.name.clone()))
            .collect()
    }

    fn get_nonterminal_names(&self) -> impl Iterator<Item = String> + '_ {
        self.nonterminals
            .iter()
            .map(|nonterminal| match nonterminal {
                Nonterminal::Struct(s) => &s.name.name,
                Nonterminal::Enum(e) => &e.name.name,
            })
            .cloned()
    }

    fn get_terminal_enum_variant_names(&self) -> impl Iterator<Item = String> + '_ {
        self.terminal_enum
            .variants
            .iter()
            .map(|variant| variant.dollarless_name.to_string())
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

pub use crate::data::ast::{
    ComplexType, Enum, EnumVariant, Fieldset, NamedField, NamedFieldset, Struct, TupleField,
    TupleFieldset, Type,
};
pub use crate::data::ast::{Ident, IdentOrTerminalIdent, IdentOrUnderscore, TerminalIdent, Token};

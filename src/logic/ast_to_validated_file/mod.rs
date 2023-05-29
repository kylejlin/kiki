use crate::data::{
    ast::*,
    validated_file::{self as validated, DollarlessTerminalName},
    ByteIndex, KikiErr,
};
use std::collections::{HashMap, HashSet};

pub fn ast_to_validated_file(file: File) -> Result<validated::File, KikiErr> {
    let terminal_enum = get_terminal_enum(&file)?;
    let nonterminals = get_nonterminals(&file)?;
    let start = get_start_symbol_name(&file, &nonterminals)?;
    let DefinedIdentifiers(defined_identifiers) = get_defined_identifiers(&file)?;

    Ok(validated::File {
        start,
        terminal_enum,
        nonterminals,
        defined_identifiers,
    })
}

mod terminal_enum;
use terminal_enum::*;

mod nonterminals;
use nonterminals::*;

mod start_symbol;
use start_symbol::*;

mod defined_identifiers;
use defined_identifiers::*;

mod type_to_string;

fn validate_symbol_ident_name_capitalization(ident: &Ident) -> Result<&str, KikiErr> {
    validate_symbol_name_capitalization(&ident.name, ident.position)
}

fn validate_symbol_terminal_ident_name_capitalization(
    ident: &TerminalIdent,
) -> Result<&str, KikiErr> {
    validate_symbol_name_capitalization(&ident.dollared_name, ident.position)
}

fn validate_symbol_name_capitalization(name: &str, position: ByteIndex) -> Result<&str, KikiErr> {
    let first_letter = name.chars().find(|c| c.is_ascii_alphabetic());
    match first_letter {
        None => Ok(name),
        Some(first_letter) => {
            if first_letter.is_ascii_uppercase() {
                Ok(name)
            } else {
                Err(KikiErr::SymbolFirstLetterNotUppercase(position))
            }
        }
    }
}

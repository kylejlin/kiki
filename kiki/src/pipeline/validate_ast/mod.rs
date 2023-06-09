use crate::data::{
    ast::*,
    validated_file::{self as validated},
    ByteIndex, DollarlessTerminalName, KikiErr,
};
use std::collections::{HashMap, HashSet};

pub fn validate_ast(file: File) -> Result<validated::File, KikiErr> {
    let terminal_enum = get_terminal_enum(&file)?;
    let nonterminals = get_nonterminals(&file)?;
    let start = get_start_symbol_name(&file, &nonterminals)?;
    assert_there_are_no_top_level_name_clashes(&file)?;

    Ok(validated::File {
        start,
        terminal_enum,
        nonterminals,
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

fn validate_ident_uppercase_start(ident: &Ident) -> Result<&str, KikiErr> {
    validate_uppercase_start(&ident.name, ident.position)
}

fn validate_terminal_ident_uppercase_start(ident: &TerminalIdent) -> Result<&str, KikiErr> {
    validate_uppercase_start(ident.name.raw(), ident.dollarless_position)
}

fn validate_uppercase_start(name: &str, position: ByteIndex) -> Result<&str, KikiErr> {
    let first_letter = name.chars().find(|c| c.is_ascii_alphabetic());
    match first_letter {
        None => Ok(name),
        Some(first_letter) => {
            if first_letter.is_ascii_uppercase() {
                Ok(name)
            } else {
                Err(KikiErr::SymbolOrTerminalEnumNameFirstLetterNotUppercase(
                    position,
                ))
            }
        }
    }
}

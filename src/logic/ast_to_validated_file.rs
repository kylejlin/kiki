use crate::data::{
    ast::*,
    validated_file::{self as validated, DollarlessTerminalName},
    ByteIndex, KikiErr,
};

pub fn ast_to_validated_file(file: File) -> Result<validated::File, KikiErr> {
    let start = get_start_symbol_name(&file)?;
    let terminal_enum = get_terminal_enum(&file)?;
    todo!()
}

fn get_start_symbol_name(file: &File) -> Result<String, KikiErr> {
    let starts: Vec<&Ident> = file
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Start(start) => Some(start),
            _ => None,
        })
        .collect();
    if starts.len() == 0 {
        Err(KikiErr::NoStartSymbol)
    } else if starts.len() > 1 {
        let positions = starts.iter().map(|start| start.position).collect();
        Err(KikiErr::MultipleStartSymbols(positions))
    } else {
        Ok(starts[0].name.clone())
    }
}

fn get_terminal_enum(file: &File) -> Result<validated::TerminalEnum, KikiErr> {
    let terminals: Vec<&TerminalDef> = file
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Terminal(t) => Some(t),
            _ => None,
        })
        .collect();
    if terminals.len() == 0 {
        Err(KikiErr::NoTerminalEnum)
    } else if terminals.len() > 1 {
        let positions = terminals.iter().map(|t| t.name.position).collect();
        Err(KikiErr::MultipleTerminalEnums(positions))
    } else {
        validate_terminal_def(terminals[0])
    }
}

fn validate_terminal_def(def: &TerminalDef) -> Result<validated::TerminalEnum, KikiErr> {
    let name = validate_ident_symbol_capitalization(&def.name)?.to_string();
    let variants = validate_terminal_variants(def)?;
    Ok(validated::TerminalEnum { name, variants })
}

fn validate_ident_symbol_capitalization(ident: &Ident) -> Result<&str, KikiErr> {
    validate_symbol_capitalization(&ident.name, ident.position)
}

fn validate_terminal_ident_symbol_capitalization(ident: &TerminalIdent) -> Result<&str, KikiErr> {
    validate_symbol_capitalization(&ident.dollared_name, ident.position)
}

fn validate_symbol_capitalization(name: &str, position: ByteIndex) -> Result<&str, KikiErr> {
    let first_letter = name.chars().find(|c| c.is_ascii_alphabetic());
    match first_letter {
        None => Ok(name),
        Some(first_letter) => {
            if first_letter.is_ascii_uppercase() {
                Ok(name)
            } else {
                Err(KikiErr::SymbolsFirstLetteNotCapitalized(position))
            }
        }
    }
}

fn validate_terminal_variants(
    def: &TerminalDef,
) -> Result<Vec<validated::TerminalVariant>, KikiErr> {
    let mut variants = Vec::new();
    for variant in &def.variants {
        let validated_name = validate_terminal_ident_symbol_capitalization(&variant.name)?;
        let dollarless_name = DollarlessTerminalName::remove_dollars(validated_name);
        let type_ = type_to_string(variant.type_);
        variants.push(validated::TerminalVariant {
            dollarless_name,
            type_,
        });
    }
    Ok(variants)
}

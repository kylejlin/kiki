use crate::data::{ast::*, validated_file as validated, KikiErr};

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
    let name = validate_symbol_capitalization(&def.name);
    let variants = validate_terminal_variants(def)?;
    Ok(validated::TerminalEnum { name, variants })
}

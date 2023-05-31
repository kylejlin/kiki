use super::*;

/// This function validates that:
/// 1. There is exactly one `start` statement.
/// 2. The start symbol refers to a valid nonterminal.
pub fn get_start_symbol_name(
    file: &File,
    nonterminals: &[validated::Nonterminal],
) -> Result<String, KikiErr> {
    let starts: Vec<&Ident> = file
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Start(start) => Some(start),
            _ => None,
        })
        .collect();

    if starts.len() == 0 {
        return Err(KikiErr::NoStartSymbol);
    }

    if starts.len() > 1 {
        let positions = starts.iter().map(|start| start.position).collect();
        return Err(KikiErr::MultipleStartSymbols(positions));
    }

    validate_start_symbol_name_is_defined(&starts[0], nonterminals)
}

fn validate_start_symbol_name_is_defined(
    start_symbol: &Ident,
    nonterminals: &[validated::Nonterminal],
) -> Result<String, KikiErr> {
    let is_defined = nonterminals
        .iter()
        .any(|nonterminal| nonterminal.name() == start_symbol.name);

    if !is_defined {
        return Err(KikiErr::UndefinedNonterminal(
            start_symbol.name.to_owned(),
            start_symbol.position,
        ));
    }

    Ok(start_symbol.name.to_owned())
}

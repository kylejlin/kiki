use super::*;

/// This function validates that:
/// 1. There is exactly one `terminal` statement.
/// 2. Each variant name has proper capitalization.
///
/// This function does **not** check for name clashes.
///
/// ## Capitalization rule:
/// If a terminal variant name contains one or more letters,
/// the first letter must be uppercase.
pub fn get_terminal_enum(file: &File) -> Result<validated::TerminalEnum, KikiErr> {
    let unvalidated = get_unvalidated_terminal_enum(file)?;
    validate_terminal_def(unvalidated)
}

/// This function validates that:
/// 1. There is exactly one `terminal` statement.
///
/// If it finds exactly one `terminal` statement, it returns
/// **without** any further validation.
pub fn get_unvalidated_terminal_enum(file: &File) -> Result<&TerminalDef, KikiErr> {
    let terminals: Vec<&TerminalDef> = file
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Terminal(t) => Some(t),
            _ => None,
        })
        .collect();

    if terminals.len() == 0 {
        return Err(KikiErr::NoTerminalEnum);
    }

    if terminals.len() > 1 {
        let positions = terminals.iter().map(|t| t.name.position).collect();
        return Err(KikiErr::MultipleTerminalEnums(positions));
    }

    Ok(&terminals[0])
}

fn validate_terminal_def(def: &TerminalDef) -> Result<validated::TerminalEnum, KikiErr> {
    let name = validate_symbol_ident_name_capitalization(&def.name)?.to_string();
    let variants = validate_terminal_variants(def)?;
    Ok(validated::TerminalEnum { name, variants })
}

fn validate_terminal_variants(
    def: &TerminalDef,
) -> Result<Vec<validated::TerminalVariant>, KikiErr> {
    let variants = def
        .variants
        .iter()
        .map(validate_variant_capitalization)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(variants)
}

fn validate_variant_capitalization(
    variant: &TerminalVariant,
) -> Result<validated::TerminalVariant, KikiErr> {
    let validated_name = validate_symbol_terminal_ident_name_capitalization(&variant.name)?;
    let dollarless_name = DollarlessTerminalName::remove_dollars(validated_name);
    let type_ = type_to_string::type_to_string(&variant.type_);
    Ok(validated::TerminalVariant {
        dollarless_name,
        type_,
    })
}

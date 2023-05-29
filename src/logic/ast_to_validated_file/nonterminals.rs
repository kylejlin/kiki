use super::*;

/// This function validates that:
/// 1. Every nonterminal name is properly capitalized.
/// 2. Every nonterminal enum variant name is properly capitalized.
/// 3. Every field name is properly capitalized.
///
/// This function does **not** check for name clashes.
///
/// ## Capitalization rules:
/// 1. If a nonterminal name contains one or more letters,
///    the first letter must be uppercase.
/// 2. If a nonterminal enum variant name contains one or more letters,
///    the first letter must be uppercase.
/// 3. If a field name contains one or more letters,
///    the first letter must be lowercase.
pub fn get_nonterminals(file: &File) -> Result<Vec<validated::Nonterminal>, KikiErr> {
    let unvalidated: Vec<UnvalidatedNonterminal> = file
        .items
        .iter()
        .filter_map(get_unvalidated_nonterminal)
        .collect();

    let defined_symbols = get_defined_symbols(&file)?;
    let nonterminals = unvalidated
        .iter()
        .map(|nonterminal| validate_nonterminal(*nonterminal, &defined_symbols))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(nonterminals)
}

#[derive(Debug, Clone, Copy)]
enum UnvalidatedNonterminal<'a> {
    Struct(&'a StructDef),
    Enum(&'a EnumDef),
}

fn get_unvalidated_nonterminal(item: &Item) -> Option<UnvalidatedNonterminal<'_>> {
    match item {
        Item::Struct(struct_def) => Some(UnvalidatedNonterminal::Struct(struct_def)),
        Item::Enum(enum_def) => Some(UnvalidatedNonterminal::Enum(enum_def)),
        _ => None,
    }
}

fn validate_nonterminal(
    nonterminal: UnvalidatedNonterminal,
    defined_symbols: &DefinedSymbols,
) -> Result<validated::Nonterminal, KikiErr> {
    match nonterminal {
        UnvalidatedNonterminal::Enum(e) => validate_enum(e, defined_symbols),
        UnvalidatedNonterminal::Struct(s) => validate_struct(s, defined_symbols),
    }
}

fn validate_enum(
    enum_def: &EnumDef,
    defined_symbols: &DefinedSymbols,
) -> Result<validated::Nonterminal, KikiErr> {
    validate_symbol_ident_name_capitalization(&enum_def.name)?;
    assert_variants_are_valid(&enum_def.variants, defined_symbols)?;
    Ok(validated::Nonterminal::Enum(enum_def.clone()))
}

fn assert_variants_are_valid(
    variants: &[EnumVariant],
    defined_symbols: &DefinedSymbols,
) -> Result<(), KikiErr> {
    for variant in variants {
        validate_symbol_ident_name_capitalization(&variant.name)?;
        assert_fieldset_is_valid(&variant.fieldset, defined_symbols)?;
    }
    Ok(())
}

fn validate_struct(
    struct_def: &StructDef,
    defined_symbols: &DefinedSymbols,
) -> Result<validated::Nonterminal, KikiErr> {
    validate_symbol_ident_name_capitalization(&struct_def.name)?;
    assert_fieldset_is_valid(&struct_def.fieldset, defined_symbols)?;
    Ok(validated::Nonterminal::Struct(struct_def.clone()))
}

fn assert_fieldset_is_valid(
    fieldset: &Fieldset,
    defined_symbols: &DefinedSymbols,
) -> Result<(), KikiErr> {
    match fieldset {
        Fieldset::Empty => Ok(()),
        Fieldset::Named(named) => assert_named_fieldset_is_valid(named, defined_symbols),
        Fieldset::Tuple(tuple) => assert_tuple_fieldset_is_valid(tuple, defined_symbols),
    }
}

fn assert_named_fieldset_is_valid(
    fieldset: &NamedFieldset,
    defined_symbols: &DefinedSymbols,
) -> Result<(), KikiErr> {
    for field in &fieldset.fields {
        assert_field_ident_or_underscore_name_is_valid(&field.name)?;
        assert_symbol_is_defined(&field.symbol, defined_symbols)?;
    }
    Ok(())
}

fn assert_tuple_fieldset_is_valid(
    fieldset: &TupleFieldset,
    defined_symbols: &DefinedSymbols,
) -> Result<(), KikiErr> {
    for field in &fieldset.fields {
        assert_symbol_is_defined(field.symbol(), defined_symbols)?;
    }
    Ok(())
}

fn assert_field_ident_or_underscore_name_is_valid(
    field: &IdentOrUnderscore,
) -> Result<(), KikiErr> {
    match field {
        IdentOrUnderscore::Underscore => Ok(()),
        IdentOrUnderscore::Ident(ident) => assert_field_ident_name_is_valid(ident),
    }
}

fn assert_field_ident_name_is_valid(ident: &Ident) -> Result<(), KikiErr> {
    assert_field_name_is_valid(&ident.name, ident.position)
}

fn assert_field_name_is_valid(name: &str, position: ByteIndex) -> Result<(), KikiErr> {
    let first_letter = name.chars().find(|c| c.is_ascii_alphabetic());
    match first_letter {
        None => Ok(()),
        Some(first_letter) => {
            if first_letter.is_ascii_lowercase() {
                Ok(())
            } else {
                Err(KikiErr::FieldFirstLetterNotLowercase(position))
            }
        }
    }
}

fn assert_symbol_is_defined(
    symbol: &IdentOrTerminalIdent,
    defined_symbols: &DefinedSymbols,
) -> Result<(), KikiErr> {
    match symbol {
        IdentOrTerminalIdent::Ident(ident) => assert_nonterminal_is_defined(ident, defined_symbols),
        IdentOrTerminalIdent::Terminal(terminal_ident) => {
            assert_terminal_is_defined(terminal_ident, defined_symbols)
        }
    }
}

fn assert_nonterminal_is_defined(
    ident: &Ident,
    defined_symbols: &DefinedSymbols,
) -> Result<(), KikiErr> {
    if defined_symbols.0.contains(&ident.name) {
        Ok(())
    } else {
        Err(KikiErr::UndefinedNonterminal(
            ident.name.clone(),
            ident.position,
        ))
    }
}

fn assert_terminal_is_defined(
    terminal_ident: &TerminalIdent,
    defined_symbols: &DefinedSymbols,
) -> Result<(), KikiErr> {
    if defined_symbols
        .0
        .contains(terminal_ident.dollarless_name().raw())
    {
        Ok(())
    } else {
        Err(KikiErr::UndefinedTerminal(
            terminal_ident.dollarless_name(),
            ByteIndex(terminal_ident.position.0 + "$".len()),
        ))
    }
}

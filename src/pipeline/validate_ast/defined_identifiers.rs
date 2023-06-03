use super::*;

/// This function validates that:
/// 1. There are no duplicate nonterminal names.
/// 2. There are no duplicate terminal variant names.
/// 3. The nonterminal names, terminal variant names,
///    _and the terminal enum name_ are pairwise disjoint.
///
/// Things this function does **not** validate:
/// 1. This function does **not** check for name clashes with
///    builtins, such as `Option`.
/// 2. This function does **not** check that every nonterminal enum
///    has uniquely named variants.
/// 3. This function does **not** validate capitalization.
pub fn assert_there_are_no_top_level_name_clashes(file: &File) -> Result<(), KikiErr> {
    let mut seen = get_defined_symbol_positions(file)?;

    define_terminal_enum_name(&mut seen, file)?;

    Ok(())
}

/// This set contains:
/// 1. Nonterminal names
/// 2. Terminal variant names
///
/// It does **not** contain the terminal enum name.
pub struct DefinedSymbols(pub HashSet<String>);

/// This function validates that:
/// 1. There are no duplicate nonterminal names.
/// 2. There are no duplicate terminal variant names.
/// 3. The nonterminal names and terminal variant names
///    are pairwise disjoint.
///
/// This function does **not** check for name clashes with
/// builtins, such as `Option`.
///
/// This function does **not** validate capitalization.
pub fn get_defined_symbols(file: &File) -> Result<DefinedSymbols, KikiErr> {
    let seen = get_defined_symbol_positions(file)?;
    Ok(DefinedSymbols(
        seen.into_iter().map(|(name, _)| name.to_owned()).collect(),
    ))
}

fn get_defined_symbol_positions(file: &File) -> Result<HashMap<String, ByteIndex>, KikiErr> {
    let mut seen: HashMap<String, ByteIndex> = HashMap::new();

    define_nonterminals(&mut seen, file)?;

    let unvalidated_terminal_enum = get_unvalidated_terminal_enum(file)?;
    define_terminal_variants(&mut seen, unvalidated_terminal_enum)?;

    Ok(seen)
}

fn define_nonterminals(seen: &mut HashMap<String, ByteIndex>, file: &File) -> Result<(), KikiErr> {
    for item in &file.items {
        define_nonterminal_if_possible(seen, item)?;
    }
    Ok(())
}

fn define_nonterminal_if_possible(
    seen: &mut HashMap<String, ByteIndex>,
    item: &FileItem,
) -> Result<(), KikiErr> {
    match item {
        FileItem::Start(_) => Ok(()),
        FileItem::Struct(struct_def) => define_nonterminal(seen, &struct_def.name),
        FileItem::Enum(enum_def) => define_nonterminal(seen, &enum_def.name),
        FileItem::Terminal(_) => Ok(()),
    }
}

fn define_nonterminal(seen: &mut HashMap<String, ByteIndex>, ident: &Ident) -> Result<(), KikiErr> {
    if let Some(conflicting_symbol_position) = seen.get(&ident.name) {
        return Err(KikiErr::NameClash(
            ident.name.to_owned(),
            *conflicting_symbol_position,
            ident.position,
        ));
    }

    seen.insert(ident.name.clone(), ident.position);

    Ok(())
}

fn define_terminal_variants(
    seen: &mut HashMap<String, ByteIndex>,
    terminal_enum: &TerminalEnum,
) -> Result<(), KikiErr> {
    for variant in &terminal_enum.variants {
        define_terminal_variant(seen, variant)?;
    }
    Ok(())
}

fn define_terminal_variant(
    seen: &mut HashMap<String, ByteIndex>,
    variant: &TerminalEnumVariant,
) -> Result<(), KikiErr> {
    let dollarless_name = variant.name.name.clone();
    let dollarless_position = variant.name.dollarless_position;

    if let Some(conflicting_symbol_pos) = seen.get(dollarless_name.raw()) {
        return Err(KikiErr::NameClash(
            dollarless_name.to_string(),
            *conflicting_symbol_pos,
            dollarless_position,
        ));
    }

    seen.insert(dollarless_name.to_string(), dollarless_position);

    Ok(())
}

fn define_terminal_enum_name(
    seen: &mut HashMap<String, ByteIndex>,
    file: &File,
) -> Result<(), KikiErr> {
    let terminal_enum = get_unvalidated_terminal_enum(file)?;

    if let Some(conflicting_symbol_position) = seen.get(&terminal_enum.name.name) {
        return Err(KikiErr::NameClash(
            terminal_enum.name.name.to_owned(),
            *conflicting_symbol_position,
            terminal_enum.name.position,
        ));
    }

    seen.insert(
        terminal_enum.name.name.to_owned(),
        terminal_enum.name.position,
    );

    Ok(())
}

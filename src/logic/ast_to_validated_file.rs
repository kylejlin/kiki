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

fn get_terminal_enum(file: &File) -> Result<validated::TerminalEnum, KikiErr> {
    let unvalidated = get_unvalidated_terminal_enum(file)?;
    validate_terminal_def(unvalidated)
}

fn get_unvalidated_terminal_enum(file: &File) -> Result<&TerminalDef, KikiErr> {
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
                Err(KikiErr::SymbolsFirstLetteNotCapitalized(position))
            }
        }
    }
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

fn get_nonterminals(file: &File) -> Result<Vec<validated::Nonterminal>, KikiErr> {
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
                Err(KikiErr::SymbolsFirstLetteNotCapitalized(position))
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

fn get_start_symbol_name(
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

struct DefinedIdentifiers(HashSet<String>);

fn get_defined_identifiers(file: &File) -> Result<DefinedIdentifiers, KikiErr> {
    let mut seen = get_defined_symbol_positions(file)?;

    define_terminal_enum_name(&mut seen, file)?;

    Ok(DefinedIdentifiers(
        seen.into_iter().map(|(name, _)| name.to_owned()).collect(),
    ))
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

struct DefinedSymbols(HashSet<String>);

fn get_defined_symbols(file: &File) -> Result<DefinedSymbols, KikiErr> {
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
    item: &Item,
) -> Result<(), KikiErr> {
    match item {
        Item::Start(_) => Ok(()),
        Item::Struct(struct_def) => define_nonterminal(seen, &struct_def.name),
        Item::Enum(enum_def) => define_nonterminal(seen, &enum_def.name),
        Item::Terminal(_) => Ok(()),
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
    terminal_enum: &TerminalDef,
) -> Result<(), KikiErr> {
    for variant in &terminal_enum.variants {
        define_terminal_variant(seen, variant)?;
    }
    Ok(())
}

fn define_terminal_variant(
    seen: &mut HashMap<String, ByteIndex>,
    variant: &TerminalVariant,
) -> Result<(), KikiErr> {
    let dollarless_name = DollarlessTerminalName::remove_dollars(&variant.name.dollared_name);
    let dollarless_position = ByteIndex(variant.name.position.0 + "$".len());

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

mod type_to_string {
    use super::*;

    pub fn type_to_string(type_: &Type) -> String {
        match type_ {
            Type::Unit => format!("()"),
            Type::Path(path) => path_to_string(path),
            Type::Complex(complex) => complex_to_string(complex),
        }
    }

    pub fn path_to_string(path: &[Ident]) -> String {
        path.iter()
            .map(|part| -> &str { &part.name })
            .collect::<Vec<&str>>()
            .join("::")
    }

    pub fn complex_to_string(complex: &ComplexType) -> String {
        let callee = path_to_string(&complex.callee);
        let comma_separated_args = complex
            .args
            .iter()
            .map(type_to_string)
            .collect::<Vec<String>>()
            .join(", ");
        format!("{callee}<{comma_separated_args}>")
    }
}

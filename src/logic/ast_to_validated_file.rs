use crate::data::{
    ast::*,
    validated_file::{self as validated, DollarlessTerminalName},
    ByteIndex, KikiErr,
};
use std::collections::{HashMap, HashSet};

pub fn ast_to_validated_file(file: File) -> Result<validated::File, KikiErr> {
    let terminal_enum = get_terminal_enum(&file)?;
    let nonterminals = get_nonterminals(&file, &terminal_enum)?;
    let start = get_start_symbol_name(&file, &nonterminals)?;
    let DefinedIdentifiers(defined_identifiers) =
        get_unvalidated_defined_identifiers(&file, &terminal_enum);

    Ok(validated::File {
        start,
        terminal_enum,
        nonterminals,
        defined_identifiers,
    })
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
    assert_no_duplicate_variants(def)?;

    let variants = def
        .variants
        .iter()
        .map(validate_variant_capitalization)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(variants)
}

fn assert_no_duplicate_variants(def: &TerminalDef) -> Result<(), KikiErr> {
    let mut seen: HashMap<&str, &TerminalVariant> = HashMap::new();
    for variant in &def.variants {
        let name: &str = &variant.name.dollared_name;

        if let Some(conflicting_variant) = seen.get(name) {
            return Err(KikiErr::DuplicateTerminalVariants(
                name.to_owned(),
                conflicting_variant.name.position,
                variant.name.position,
            ));
        }

        seen.insert(name, variant);
    }
    Ok(())
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

fn get_nonterminals(
    file: &File,
    terminal_enum: &validated::TerminalEnum,
) -> Result<Vec<validated::Nonterminal>, KikiErr> {
    let unvalidated: Vec<UnvalidatedNonterminal> = file
        .items
        .iter()
        .filter_map(get_unvalidated_nonterminal)
        .collect();

    assert_no_duplicate_nonterminals(&unvalidated)?;

    let names = get_unvalidated_defined_symbol_names(&unvalidated, terminal_enum);
    let nonterminals = unvalidated
        .iter()
        .map(|nonterminal| validate_nonterminal(*nonterminal, &names))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(nonterminals)
}

#[derive(Debug, Clone, Copy)]
enum UnvalidatedNonterminal<'a> {
    Struct(&'a StructDef),
    Enum(&'a EnumDef),
}

impl<'a> UnvalidatedNonterminal<'a> {
    fn name(self) -> &'a Ident {
        match self {
            UnvalidatedNonterminal::Struct(struct_def) => &struct_def.name,
            UnvalidatedNonterminal::Enum(enum_def) => &enum_def.name,
        }
    }
}

fn get_unvalidated_nonterminal(item: &Item) -> Option<UnvalidatedNonterminal<'_>> {
    match item {
        Item::Struct(struct_def) => Some(UnvalidatedNonterminal::Struct(struct_def)),
        Item::Enum(enum_def) => Some(UnvalidatedNonterminal::Enum(enum_def)),
        _ => None,
    }
}

fn assert_no_duplicate_nonterminals(
    nonterminals: &[UnvalidatedNonterminal],
) -> Result<(), KikiErr> {
    let mut seen: HashMap<&str, &UnvalidatedNonterminal> = HashMap::new();
    for nonterminal in nonterminals {
        let name: &str = &nonterminal.name().name;

        if let Some(conflicting_nonterminal) = seen.get(name) {
            return Err(KikiErr::DuplicateNonterminals(
                name.to_owned(),
                conflicting_nonterminal.name().position,
                nonterminal.name().position,
            ));
        }

        seen.insert(name, nonterminal);
    }
    Ok(())
}

struct DefinedSymbolNames(HashSet<String>);

fn get_unvalidated_defined_symbol_names(
    nonterminals: &[UnvalidatedNonterminal],
    terminal_enum: &validated::TerminalEnum,
) -> DefinedSymbolNames {
    let nonterminal_symbols = nonterminals
        .iter()
        .map(|nonterminal| UnvalidatedNonterminal::name(*nonterminal).name.to_owned());

    let terminal_symbols = terminal_enum
        .variants
        .iter()
        .map(|variant| variant.dollarless_name.to_string());

    DefinedSymbolNames(nonterminal_symbols.chain(terminal_symbols).collect())
}

fn validate_nonterminal(
    nonterminal: UnvalidatedNonterminal,
    defined_symbol_names: &DefinedSymbolNames,
) -> Result<validated::Nonterminal, KikiErr> {
    match nonterminal {
        UnvalidatedNonterminal::Enum(e) => validate_enum(e, defined_symbol_names),
        UnvalidatedNonterminal::Struct(s) => validate_struct(s, defined_symbol_names),
    }
}

fn validate_enum(
    enum_def: &EnumDef,
    defined_symbol_names: &DefinedSymbolNames,
) -> Result<validated::Nonterminal, KikiErr> {
    validate_symbol_ident_name_capitalization(&enum_def.name)?;
    assert_variants_are_valid(&enum_def.variants, defined_symbol_names)?;
    Ok(validated::Nonterminal::Enum(enum_def.clone()))
}

fn assert_variants_are_valid(
    variants: &[EnumVariant],
    defined_symbol_names: &DefinedSymbolNames,
) -> Result<(), KikiErr> {
    for variant in variants {
        validate_symbol_ident_name_capitalization(&variant.name)?;
        assert_fieldset_is_valid(&variant.fieldset, defined_symbol_names)?;
    }
    Ok(())
}

fn validate_struct(
    struct_def: &StructDef,
    defined_symbol_names: &DefinedSymbolNames,
) -> Result<validated::Nonterminal, KikiErr> {
    validate_symbol_ident_name_capitalization(&struct_def.name)?;
    assert_fieldset_is_valid(&struct_def.fieldset, defined_symbol_names)?;
    Ok(validated::Nonterminal::Struct(struct_def.clone()))
}

fn assert_fieldset_is_valid(
    fieldset: &Fieldset,
    defined_symbol_names: &DefinedSymbolNames,
) -> Result<(), KikiErr> {
    match fieldset {
        Fieldset::Empty => Ok(()),
        Fieldset::Named(named) => assert_named_fieldset_is_valid(named, defined_symbol_names),
        Fieldset::Tuple(tuple) => assert_tuple_fieldset_is_valid(tuple, defined_symbol_names),
    }
}

fn assert_named_fieldset_is_valid(
    fieldset: &NamedFieldset,
    defined_symbol_names: &DefinedSymbolNames,
) -> Result<(), KikiErr> {
    for field in &fieldset.fields {
        assert_field_ident_or_underscore_name_is_valid(&field.name)?;
        assert_symbol_is_defined(&field.symbol, defined_symbol_names)?;
    }
    Ok(())
}

fn assert_tuple_fieldset_is_valid(
    fieldset: &TupleFieldset,
    defined_symbol_names: &DefinedSymbolNames,
) -> Result<(), KikiErr> {
    for field in &fieldset.fields {
        assert_symbol_is_defined(field.symbol(), defined_symbol_names)?;
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
    defined_symbol_names: &DefinedSymbolNames,
) -> Result<(), KikiErr> {
    match symbol {
        IdentOrTerminalIdent::Ident(ident) => {
            assert_nonterminal_is_defined(ident, defined_symbol_names)
        }
        IdentOrTerminalIdent::Terminal(terminal_ident) => {
            assert_terminal_is_defined(terminal_ident, defined_symbol_names)
        }
    }
}

fn assert_nonterminal_is_defined(
    ident: &Ident,
    defined_symbol_names: &DefinedSymbolNames,
) -> Result<(), KikiErr> {
    if defined_symbol_names.0.contains(&ident.name) {
        Ok(())
    } else {
        Err(KikiErr::UndefinedSymbol(ident.name.clone(), ident.position))
    }
}

fn assert_terminal_is_defined(
    terminal_ident: &TerminalIdent,
    defined_symbol_names: &DefinedSymbolNames,
) -> Result<(), KikiErr> {
    if defined_symbol_names
        .0
        .contains(terminal_ident.dollarless_name().raw())
    {
        Ok(())
    } else {
        Err(KikiErr::UndefinedSymbol(
            terminal_ident.dollared_name.clone(),
            terminal_ident.position,
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
        return Err(KikiErr::UndefinedSymbol(
            start_symbol.name.to_owned(),
            start_symbol.position,
        ));
    }

    Ok(start_symbol.name.to_owned())
}

struct DefinedIdentifiers(HashSet<String>);

fn get_unvalidated_defined_identifiers(
    file: &File,
    terminal_enum: &validated::TerminalEnum,
) -> DefinedIdentifiers {
    let item_identifiers = get_item_identifiers(file);
    let terminal_variant_identifiers = get_terminal_variant_identifiers(terminal_enum);
    DefinedIdentifiers(
        item_identifiers
            .chain(terminal_variant_identifiers)
            .collect(),
    )
}

fn get_item_identifiers(file: &File) -> impl Iterator<Item = String> + '_ {
    file.items.iter().filter_map(|item| match item {
        Item::Start(_) => None,
        Item::Struct(struct_def) => Some(struct_def.name.name.clone()),
        Item::Enum(enum_def) => Some(enum_def.name.name.clone()),
        Item::Terminal(terminal_def) => Some(terminal_def.name.name.clone()),
    })
}

fn get_terminal_variant_identifiers(
    terminal_enum: &validated::TerminalEnum,
) -> impl Iterator<Item = String> + '_ {
    terminal_enum
        .variants
        .iter()
        .map(|variant| variant.dollarless_name.to_string())
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

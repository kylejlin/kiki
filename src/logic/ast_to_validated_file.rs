use crate::data::{
    ast::*,
    validated_file::{self as validated, DollarlessTerminalName},
    ByteIndex, KikiErr,
};
use std::collections::HashMap;

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
    let validated_name = validate_terminal_ident_symbol_capitalization(&variant.name)?;
    let dollarless_name = DollarlessTerminalName::remove_dollars(validated_name);
    let type_ = type_to_string::type_to_string(&variant.type_);
    Ok(validated::TerminalVariant {
        dollarless_name,
        type_,
    })
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

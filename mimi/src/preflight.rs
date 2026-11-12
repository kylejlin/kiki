use crate::internal_prelude::*;

/// Paired spans identify the first occurrence followed by its duplicate.
#[derive(Debug, Clone)]
pub enum PreflightCheckError {
    NoStartItem,
    MultipleStartItems(span_of::UpperName, span_of::UpperName),
    NoTerminalItem,
    MultipleTerminalItems(span_of::UpperName, span_of::UpperName),
    NoBridgeItem,
    MultipleBridgeItems(Span, Span),
    ItemNameClash(span_of::UpperName, span_of::UpperName),
    EnumVariantNameClash(span_of::UpperName, span_of::UpperName),
    TerminalVariantNameClash(span_of::DollarUpperName, span_of::DollarUpperName),
    VariantSymbolSequenceClash(Span, Span),
    FieldNameClash(span_of::LowerName, span_of::LowerName),
    UndefinedTerminal(span_of::DollarUpperName),
    UndefinedNonTerminal(span_of::UpperName),
}

pub struct PreflightCertificate<'a> {
    pub(crate) start_symbol: span_of::UpperName,
    pub(crate) terminal: &'a m::TerminalEnum,
    pub(crate) bridge_path: &'a m::Path,
    pub(crate) non_terminal_defs: HashMap<SigilFreeStr<'a>, &'a m::FileItem>,
    pub(crate) terminal_resolutions: HashMap<SigilFreeStr<'a>, SigilFreeStr<'a>>,
}

struct PreflightDefinitions<'a> {
    non_terminal_defs: HashMap<SigilFreeStr<'a>, &'a m::FileItem>,
    terminal_resolutions: HashMap<SigilFreeStr<'a>, SigilFreeStr<'a>>,
}

/// Precondition: `file` must be the parse result of `source`.
///
/// Postcondition: Returns `Ok` if and only if all of the following hold:
/// 1. There is exactly one `start` item.
/// 2. There is exactly one `terminal` item.
/// 3. There is exactly one `bridge` item.
/// 4. No two items have the same name.
/// 5. For any given `enum` or `terminal` item, no two variants have the same name.
/// 6. For any given `enum` or `manual` item, no two variants have the same symbol sequence.
///    For example, `enum Foo { Bar { a: $Lorem, b: Ipsum } Baz { x: $Lorem, _: Ipsum } }` is illegal
///    (since both `Bar` and `Baz` have the sequence `[$Lorem, Ipsum]`).
/// 7. For any given named field set, no two fields have the same name (obviously, underscores are exempt).
/// 8. Every terminal grammar symbol occurrence is defined in the `terminal` item.
/// 9. Every non-terminal grammar symbol occurrence (including the `start` symbol) is defined by a non-terminal item (i.e., `struct`, `enum`, or `manual`).
pub fn preflight_check<'a>(
    source: &'a str,
    file: &'a m::File,
) -> Result<PreflightCertificate<'a>, PreflightCheckError> {
    let (start_symbol, terminal, bridge_path) = get_required_items(file)?;
    let non_terminal_defs = get_non_terminal_defs(source, file)?;
    let terminal_resolutions = get_terminal_resolutions(source, terminal)?;
    let definitions = PreflightDefinitions {
        non_terminal_defs,
        terminal_resolutions,
    };

    check_non_terminal(source, start_symbol, &definitions)?;

    for item in &file.items {
        match item {
            m::FileItem::Start(_) => {}
            m::FileItem::Struct(m::Struct {
                prefix: _,
                name: _,
                fieldset,
            }) => {
                check_fieldset(source, fieldset, &definitions)?;
            }
            m::FileItem::Enum(def) => check_enum(source, def, &definitions)?,
            m::FileItem::Manual(def) => check_manual(source, def, &definitions)?,
            m::FileItem::Terminal(_) => {}
            m::FileItem::Bridge(_) => {}
        }
    }

    Ok(PreflightCertificate {
        start_symbol,
        terminal,
        bridge_path,
        non_terminal_defs: definitions.non_terminal_defs,
        terminal_resolutions: definitions.terminal_resolutions,
    })
}

fn get_required_items(
    file: &m::File,
) -> Result<(span_of::UpperName, &m::TerminalEnum, &m::Path), PreflightCheckError> {
    let mut start_symbol = None;
    let mut terminal: Option<&m::TerminalEnum> = None;
    let mut bridge_path: Option<&m::Path> = None;

    for item in &file.items {
        match item {
            m::FileItem::Start(name) => {
                if let Some(first) = start_symbol {
                    return Err(PreflightCheckError::MultipleStartItems(first, *name));
                }
                start_symbol = Some(*name);
            }
            m::FileItem::Struct(_) => {}
            m::FileItem::Enum(_) => {}
            m::FileItem::Manual(_) => {}
            m::FileItem::Terminal(def) => {
                if let Some(first) = terminal {
                    return Err(PreflightCheckError::MultipleTerminalItems(
                        first.name, def.name,
                    ));
                }
                terminal = Some(def);
            }
            m::FileItem::Bridge(path) => {
                if let Some(first) = bridge_path {
                    return Err(PreflightCheckError::MultipleBridgeItems(
                        path_span(first),
                        path_span(path),
                    ));
                }
                bridge_path = Some(path);
            }
        }
    }

    Ok((
        start_symbol.ok_or(PreflightCheckError::NoStartItem)?,
        terminal.ok_or(PreflightCheckError::NoTerminalItem)?,
        bridge_path.ok_or(PreflightCheckError::NoBridgeItem)?,
    ))
}

fn get_non_terminal_defs<'a>(
    source: &'a str,
    file: &'a m::File,
) -> Result<HashMap<SigilFreeStr<'a>, &'a m::FileItem>, PreflightCheckError> {
    let mut names = HashMap::new();
    let mut out = HashMap::new();

    for item in &file.items {
        let name = match item {
            m::FileItem::Start(_) => continue,
            m::FileItem::Struct(m::Struct {
                prefix: _,
                name,
                fieldset: _,
            }) => *name,
            m::FileItem::Enum(m::Enum {
                prefix: _,
                name,
                variants: _,
            }) => *name,
            m::FileItem::Manual(m::Manual {
                name,
                override_: _,
                variants: _,
            }) => *name,
            m::FileItem::Terminal(m::TerminalEnum {
                prefix: _,
                name,
                variants: _,
            }) => *name,
            m::FileItem::Bridge(_) => continue,
        };
        let key = sigil_free_name(source, name.raw);
        if let Some(first) = names.insert(key, name) {
            return Err(PreflightCheckError::ItemNameClash(first, name));
        }
        if !matches!(item, m::FileItem::Terminal(_)) {
            out.insert(key, item);
        }
    }

    Ok(out)
}

fn check_enum(
    source: &str,
    def: &m::Enum,
    definitions: &PreflightDefinitions<'_>,
) -> Result<(), PreflightCheckError> {
    let m::Enum {
        prefix: _,
        name: _,
        variants,
    } = def;
    let mut names = HashMap::new();
    let mut sequences = HashMap::new();

    for m::EnumVariant { name, fieldset } in variants {
        let key = sigil_free_name(source, name.raw);
        if let Some(first) = names.insert(key, *name) {
            return Err(PreflightCheckError::EnumVariantNameClash(first, *name));
        }
        let sequence = check_fieldset(source, fieldset, definitions)?;
        if let Some(first) = sequences.insert(sequence, name.raw) {
            return Err(PreflightCheckError::VariantSymbolSequenceClash(
                first, name.raw,
            ));
        }
    }

    Ok(())
}

fn check_manual(
    source: &str,
    def: &m::Manual,
    definitions: &PreflightDefinitions<'_>,
) -> Result<(), PreflightCheckError> {
    let m::Manual {
        name: _,
        override_: _,
        variants,
    } = def;
    let mut sequences = HashMap::new();

    for m::ManualVariant { function, inputs } in variants {
        let sequence = check_tuple_fields(source, inputs, definitions)?;
        if let Some(first) = sequences.insert(sequence, function.raw) {
            return Err(PreflightCheckError::VariantSymbolSequenceClash(
                first,
                function.raw,
            ));
        }
    }

    Ok(())
}

fn get_terminal_resolutions<'a>(
    source: &'a str,
    terminal: &m::TerminalEnum,
) -> Result<HashMap<SigilFreeStr<'a>, SigilFreeStr<'a>>, PreflightCheckError> {
    let m::TerminalEnum {
        prefix: _,
        name: _,
        variants,
    } = terminal;
    let mut names = HashMap::new();
    let mut out = HashMap::new();

    for m::TerminalEnumVariant { name, override_ } in variants {
        let key = sigil_free_name(source, name.raw);
        if let Some(first) = names.insert(key, *name) {
            return Err(PreflightCheckError::TerminalVariantNameClash(first, *name));
        }
        let resolution = match override_ {
            None => key,
            Some(name) => sigil_free_name(source, name.raw),
        };
        out.insert(key, resolution);
    }

    Ok(out)
}

/// Returns the symbol sequence, preserving sigils and including skipped fields.
fn check_fieldset<'a>(
    source: &'a str,
    fieldset: &m::Fieldset,
    definitions: &PreflightDefinitions<'_>,
) -> Result<Vec<&'a str>, PreflightCheckError> {
    match fieldset {
        m::Fieldset::Empty => Ok(Vec::new()),
        m::Fieldset::Named(m::NamedFieldset { fields }) => {
            let mut names = HashMap::new();
            let mut sequence = Vec::with_capacity(fields.len());

            for m::NamedField { name, symbol } in fields {
                match name {
                    m::UnderscoreOrLowerName::Underscore(_) => {}
                    m::UnderscoreOrLowerName::Lower(name) => {
                        let key = sigil_free_name(source, name.raw);
                        if let Some(first) = names.insert(key, *name) {
                            return Err(PreflightCheckError::FieldNameClash(first, *name));
                        }
                    }
                }
                sequence.push(check_grammar_symbol(source, symbol, definitions)?);
            }

            Ok(sequence)
        }
        m::Fieldset::Tuple(m::TupleFieldset { fields }) => {
            check_tuple_fields(source, fields, definitions)
        }
    }
}

fn check_tuple_fields<'a>(
    source: &'a str,
    fields: &[m::TupleField],
    definitions: &PreflightDefinitions<'_>,
) -> Result<Vec<&'a str>, PreflightCheckError> {
    fields
        .iter()
        .map(|field| {
            let symbol = match field {
                m::TupleField::Used(symbol) => symbol,
                m::TupleField::Skipped(symbol) => symbol,
            };
            check_grammar_symbol(source, symbol, definitions)
        })
        .collect()
}

fn check_grammar_symbol<'a>(
    source: &'a str,
    symbol: &m::GrammarSymbol,
    definitions: &PreflightDefinitions<'_>,
) -> Result<&'a str, PreflightCheckError> {
    match symbol {
        m::GrammarSymbol::Upper(name) => {
            check_non_terminal(source, *name, definitions)?;
            Ok(source_text(source, name.raw))
        }
        m::GrammarSymbol::DollarUpper(name) => {
            let key = sigil_free_name(source, name.raw);
            if !definitions.terminal_resolutions.contains_key(&key) {
                return Err(PreflightCheckError::UndefinedTerminal(*name));
            }
            Ok(source_text(source, name.raw))
        }
    }
}

fn check_non_terminal(
    source: &str,
    name: span_of::UpperName,
    definitions: &PreflightDefinitions<'_>,
) -> Result<(), PreflightCheckError> {
    let key = sigil_free_name(source, name.raw);
    if !definitions.non_terminal_defs.contains_key(&key) {
        return Err(PreflightCheckError::UndefinedNonTerminal(name));
    }
    Ok(())
}

fn sigil_free_name(source: &str, span: Span) -> SigilFreeStr<'_> {
    let text = source_text(source, span);
    let raw = text.strip_prefix(['$', '@']).unwrap_or(text);
    SigilFreeStr::new(raw)
        .expect("Parsed names contain only ASCII letters, digits, and underscores")
}

fn source_text(source: &str, span: Span) -> &str {
    &source[span.start.byte_index..span.end.byte_index]
}

fn path_span(path: &m::Path) -> Span {
    let name_span = |name: &m::Name| match name {
        m::Name::Upper(name) => name.raw,
        m::Name::Lower(name) => name.raw,
    };
    Span {
        start: name_span(path.first().expect("Parsed paths are nonempty")).start,
        end: name_span(path.last().expect("Parsed paths are nonempty")).end,
    }
}

pub use refined_str::*;
mod refined_str {
    use super::*;

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct SigilFreeStr<'a> {
        raw: &'a str,
    }
    impl Debug for SigilFreeStr<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            self.raw.fmt(f)
        }
    }
    impl<'a> SigilFreeStr<'a> {
        pub fn new(raw: &'a str) -> Option<Self> {
            if raw.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                Some(Self { raw })
            } else {
                None
            }
        }

        pub const fn raw(self) -> &'a str {
            self.raw
        }
    }
}

use crate::internal_prelude::*;

pub struct PreflightCertificate<'a> {
    pub(crate) source: &'a str,
    pub(crate) file: &'a cst::File<'a>,
    pub(crate) start_symbol: token::UpperName<'a>,
    pub(crate) terminal_enum: &'a cst::TerminalEnum<'a>,
    pub(crate) bridge_path: &'a cst::Path<'a>,

    /// IDs follow declaration order, starting at 1.
    pub(crate) terminal_defs: TerminalIdMap<&'a cst::TerminalEnumVariant<'a>>,

    /// IDs follow declaration order, with index 0 reserved for the synthetic start symbol.
    pub(crate) non_terminal_defs: OriginalNonTerminalIdMap<&'a cst::FileItem<'a>>,

    /// The first rule is always the synthetic rule (i.e., `{synthetic_start} <- {original_start}`).
    /// Original rules follow declaration order, including variant order within each item.
    pub(crate) rules: Vec<Rule>,
}

/// Paired fragments identify the first occurrence followed by its duplicate.
#[derive(Debug, Clone)]
pub enum PreflightCheckError<'a> {
    NoStartItem,
    MultipleStartItems(token::UpperName<'a>, token::UpperName<'a>),
    NoTerminalItem,
    MultipleTerminalItems(token::UpperName<'a>, token::UpperName<'a>),
    NoBridgeItem,
    MultipleBridgeItems(SourceFragment<'a>, SourceFragment<'a>),
    ItemNameClash(token::UpperName<'a>, token::UpperName<'a>),
    EnumVariantNameClash(token::UpperName<'a>, token::UpperName<'a>),
    TerminalVariantNameClash(token::DollarUpperName<'a>, token::DollarUpperName<'a>),
    VariantSymbolSequenceClash(SourceFragment<'a>, SourceFragment<'a>),
    FieldNameClash(token::LowerName<'a>, token::LowerName<'a>),
    UndefinedTerminal(token::DollarUpperName<'a>),
    UndefinedNonTerminal(token::UpperName<'a>),
    LifetimeNameClash(
        token::SingleQuoteLowerName<'a>,
        token::SingleQuoteLowerName<'a>,
    ),
    UndefinedLifetime(token::SingleQuoteLowerName<'a>),
}

struct PreflightDefinitions<'a> {
    terminal_defs: TerminalIdMap<&'a cst::TerminalEnumVariant<'a>>,
    terminal_ids: HashMap<&'a str, TerminalId>,
    non_terminal_defs: OriginalNonTerminalIdMap<&'a cst::FileItem<'a>>,
    non_terminal_ids: HashMap<&'a str, NonTerminalId>,
}

/// Precondition: `file` must be the parse result of `source`.
///
/// Postcondition: Returns `Ok` if and only if all of the following hold:
/// 1. There is exactly one `start` item.
/// 2. There is exactly one `terminal` item.
/// 3. There is exactly one `bridge` item.
/// 4. No two `struct`, `enum`, `manual`, or `terminal` items have the same name.
/// 5. For any given `enum` or `terminal` item, no two variants have the same name.
/// 6. For any given `enum` or `manual` item, no two variants have the same symbol sequence.
///    For example, `enum Foo { Bar { a: $Lorem b: Ipsum } Baz { x: $Lorem _: Ipsum } }` is illegal
///    (since both `Bar` and `Baz` have the sequence `[$Lorem, Ipsum]`).
/// 7. For any given named field set, no two fields have the same name (obviously, underscores are exempt).
/// 8. Every terminal grammar symbol occurrence is defined in the `terminal` item.
/// 9. Every non-terminal grammar symbol occurrence (including the `start` symbol) is defined by a non-terminal item (i.e., `struct`, `enum`, or `manual`).
/// 10. For any given lifetime list, there are no duplicates.
/// 11. Any given lifetime list is a non-strict subset of the `terminal` item's global lifetime set.
///
/// Names are compared by spelling. `Self` denotes the enclosing non-terminal item,
/// including when comparing symbol sequences. Symbol sequences include skipped fields.
pub fn preflight_check<'a>(
    source: &'a str,
    file: &'a cst::File<'a>,
) -> Result<PreflightCertificate<'a>, PreflightCheckError<'a>> {
    let (start_symbol, terminal_enum, bridge_path) = get_required_items(source, file)?;
    let definitions = get_definitions(file, terminal_enum)?;
    let global_lifetimes = get_lifetime_names(&terminal_enum.global_lifetimes)?;

    for variant in &terminal_enum.variants {
        check_lifetimes(&variant.lifetimes, &global_lifetimes)?;
    }

    let rules = get_rules(file, start_symbol, &definitions, &global_lifetimes)?;
    let PreflightDefinitions {
        terminal_defs,
        terminal_ids: _,
        non_terminal_defs,
        non_terminal_ids: _,
    } = definitions;

    Ok(PreflightCertificate {
        source,
        file,
        start_symbol,
        terminal_enum,
        bridge_path,
        terminal_defs,
        non_terminal_defs,
        rules,
    })
}

fn get_required_items<'a>(
    source: &'a str,
    file: &'a cst::File<'a>,
) -> Result<
    (
        token::UpperName<'a>,
        &'a cst::TerminalEnum<'a>,
        &'a cst::Path<'a>,
    ),
    PreflightCheckError<'a>,
> {
    let mut start_symbol = None;
    let mut terminal_enum: Option<&cst::TerminalEnum<'a>> = None;
    let mut bridge_path = None;

    for item in &file.items {
        match item {
            cst::FileItem::Start(name) => {
                if let Some(first) = start_symbol {
                    return Err(PreflightCheckError::MultipleStartItems(first, *name));
                }
                start_symbol = Some(*name);
            }
            cst::FileItem::Struct(_) => {}
            cst::FileItem::Enum(_) => {}
            cst::FileItem::Manual(_) => {}
            cst::FileItem::Terminal(def) => {
                if let Some(first) = terminal_enum {
                    return Err(PreflightCheckError::MultipleTerminalItems(
                        first.name, def.name,
                    ));
                }
                terminal_enum = Some(def);
            }
            cst::FileItem::Bridge(path) => {
                if let Some(first) = bridge_path {
                    return Err(PreflightCheckError::MultipleBridgeItems(
                        path_fragment(source, first),
                        path_fragment(source, path),
                    ));
                }
                bridge_path = Some(path);
            }
        }
    }

    Ok((
        start_symbol.ok_or(PreflightCheckError::NoStartItem)?,
        terminal_enum.ok_or(PreflightCheckError::NoTerminalItem)?,
        bridge_path.ok_or(PreflightCheckError::NoBridgeItem)?,
    ))
}

fn get_definitions<'a>(
    file: &'a cst::File<'a>,
    terminal_enum: &'a cst::TerminalEnum<'a>,
) -> Result<PreflightDefinitions<'a>, PreflightCheckError<'a>> {
    let mut item_names: HashMap<&str, token::UpperName<'a>> = HashMap::new();
    let mut non_terminal_defs = Vec::new();
    let mut non_terminal_ids = HashMap::new();

    for item in &file.items {
        let name = match item {
            cst::FileItem::Start(_) => continue,
            cst::FileItem::Struct(def) => def.name,
            cst::FileItem::Enum(def) => def.name,
            cst::FileItem::Manual(def) => def.name,
            cst::FileItem::Terminal(def) => def.name,
            cst::FileItem::Bridge(_) => continue,
        };
        let key = name.raw.raw();
        if let Some(first) = item_names.insert(key, name) {
            return Err(PreflightCheckError::ItemNameClash(first, name));
        }
        if !matches!(item, cst::FileItem::Terminal(_)) {
            let id = NonTerminalId {
                raw: NonZeroUsize::new(SymbolId::HIGH_BIT_MASK | (non_terminal_defs.len() + 1))
                    .unwrap(),
            };
            non_terminal_defs.push(item);
            non_terminal_ids.insert(key, id);
        }
    }

    let mut terminal_defs: Vec<&cst::TerminalEnumVariant<'a>> = Vec::new();
    let mut terminal_ids: HashMap<&str, TerminalId> = HashMap::new();
    for variant in &terminal_enum.variants {
        let name = variant.name;
        let key = name.raw.raw();
        if let Some(&id) = terminal_ids.get(key) {
            let first = terminal_defs[id.raw.get() - 1];
            return Err(PreflightCheckError::TerminalVariantNameClash(
                first.name, name,
            ));
        }
        let id = TerminalId {
            raw: NonZeroUsize::new(terminal_defs.len() + 1).unwrap(),
        };
        terminal_defs.push(variant);
        terminal_ids.insert(key, id);
    }

    Ok(PreflightDefinitions {
        terminal_defs: TerminalIdMap { raw: terminal_defs },
        terminal_ids,
        non_terminal_defs: OriginalNonTerminalIdMap {
            raw: non_terminal_defs,
        },
        non_terminal_ids,
    })
}

fn get_rules<'a>(
    file: &cst::File<'a>,
    start_symbol: token::UpperName<'a>,
    definitions: &PreflightDefinitions<'a>,
    global_lifetimes: &HashMap<&str, token::SingleQuoteLowerName<'a>>,
) -> Result<Vec<Rule>, PreflightCheckError<'a>> {
    let start_id = check_non_terminal(start_symbol, definitions)?;
    let mut rules = vec![Rule {
        lhs: NonTerminalId {
            raw: NonZeroUsize::new(SymbolId::HIGH_BIT_MASK).unwrap(),
        },
        rhs: vec![start_id.into()],
    }];

    for item in &file.items {
        match item {
            cst::FileItem::Start(_) => {}
            cst::FileItem::Struct(cst::Struct {
                prefix: _,
                name,
                fieldset,
            }) => {
                let lhs = definitions.non_terminal_ids[name.raw.raw()];
                let rhs = check_fieldset(fieldset, lhs, definitions)?;
                rules.push(Rule { lhs, rhs });
            }
            cst::FileItem::Enum(def) => check_enum(def, definitions, &mut rules)?,
            cst::FileItem::Manual(def) => {
                check_lifetimes(&def.lifetimes, global_lifetimes)?;
                check_manual(def, definitions, &mut rules)?;
            }
            cst::FileItem::Terminal(_) => {}
            cst::FileItem::Bridge(_) => {}
        }
    }

    Ok(rules)
}

fn check_enum<'a>(
    def: &cst::Enum<'a>,
    definitions: &PreflightDefinitions<'a>,
    rules: &mut Vec<Rule>,
) -> Result<(), PreflightCheckError<'a>> {
    let cst::Enum {
        prefix: _,
        name,
        variants,
    } = def;
    let lhs = definitions.non_terminal_ids[name.raw.raw()];
    let mut names = HashMap::new();
    let mut sequences = HashMap::new();

    for cst::EnumVariant { name, fieldset } in variants {
        if let Some(first) = names.insert(name.raw.raw(), *name) {
            return Err(PreflightCheckError::EnumVariantNameClash(first, *name));
        }
        let rhs = check_fieldset(fieldset, lhs, definitions)?;
        if let Some(first) = sequences.insert(rhs.clone(), name.raw) {
            return Err(PreflightCheckError::VariantSymbolSequenceClash(
                first, name.raw,
            ));
        }
        rules.push(Rule { lhs, rhs });
    }

    Ok(())
}

fn check_manual<'a>(
    def: &cst::Manual<'a>,
    definitions: &PreflightDefinitions<'a>,
    rules: &mut Vec<Rule>,
) -> Result<(), PreflightCheckError<'a>> {
    let cst::Manual {
        name,
        override_: _,
        lifetimes: _,
        variants,
    } = def;
    let lhs = definitions.non_terminal_ids[name.raw.raw()];
    let mut sequences = HashMap::new();

    for cst::ManualVariant { function, inputs } in variants {
        let rhs = check_tuple_fields(inputs, lhs, definitions)?;
        if let Some(first) = sequences.insert(rhs.clone(), function.raw) {
            return Err(PreflightCheckError::VariantSymbolSequenceClash(
                first,
                function.raw,
            ));
        }
        rules.push(Rule { lhs, rhs });
    }

    Ok(())
}

fn check_fieldset<'a>(
    fieldset: &cst::Fieldset<'a>,
    lhs: NonTerminalId,
    definitions: &PreflightDefinitions<'a>,
) -> Result<Vec<SymbolId>, PreflightCheckError<'a>> {
    match fieldset {
        cst::Fieldset::Empty => Ok(Vec::new()),
        cst::Fieldset::Named(cst::NamedFieldset { fields }) => {
            let mut names = HashMap::new();
            let mut sequence = Vec::with_capacity(fields.len());

            for cst::NamedField { name, symbol } in fields {
                match name {
                    cst::UnderscoreOrLowerName::Underscore(_) => {}
                    cst::UnderscoreOrLowerName::Lower(name) => {
                        if let Some(first) = names.insert(name.raw.raw(), *name) {
                            return Err(PreflightCheckError::FieldNameClash(first, *name));
                        }
                    }
                }
                sequence.push(check_grammar_symbol(symbol, lhs, definitions)?);
            }

            Ok(sequence)
        }
        cst::Fieldset::Tuple(cst::TupleFieldset { fields }) => {
            check_tuple_fields(fields, lhs, definitions)
        }
    }
}

fn check_tuple_fields<'a>(
    fields: &[cst::TupleField<'a>],
    lhs: NonTerminalId,
    definitions: &PreflightDefinitions<'a>,
) -> Result<Vec<SymbolId>, PreflightCheckError<'a>> {
    fields
        .iter()
        .map(|field| {
            let symbol = match field {
                cst::TupleField::Used(symbol) | cst::TupleField::Skipped(symbol) => symbol,
            };
            check_grammar_symbol(symbol, lhs, definitions)
        })
        .collect()
}

fn check_grammar_symbol<'a>(
    symbol: &cst::GrammarSymbolOrSelf<'a>,
    lhs: NonTerminalId,
    definitions: &PreflightDefinitions<'a>,
) -> Result<SymbolId, PreflightCheckError<'a>> {
    match symbol {
        cst::GrammarSymbolOrSelf::Upper(name) => {
            check_non_terminal(*name, definitions).map(SymbolId::from_non_terminal)
        }
        cst::GrammarSymbolOrSelf::DollarUpper(name) => definitions
            .terminal_ids
            .get(name.raw.raw())
            .copied()
            .map(SymbolId::from_terminal)
            .ok_or(PreflightCheckError::UndefinedTerminal(*name)),
        cst::GrammarSymbolOrSelf::Self_(_) => Ok(lhs.into()),
    }
}

fn check_non_terminal<'a>(
    name: token::UpperName<'a>,
    definitions: &PreflightDefinitions<'a>,
) -> Result<NonTerminalId, PreflightCheckError<'a>> {
    definitions
        .non_terminal_ids
        .get(name.raw.raw())
        .copied()
        .ok_or(PreflightCheckError::UndefinedNonTerminal(name))
}

fn get_lifetime_names<'a>(
    lifetimes: &[token::SingleQuoteLowerName<'a>],
) -> Result<HashMap<&'a str, token::SingleQuoteLowerName<'a>>, PreflightCheckError<'a>> {
    let mut names = HashMap::new();
    for &name in lifetimes {
        if let Some(first) = names.insert(name.raw.raw(), name) {
            return Err(PreflightCheckError::LifetimeNameClash(first, name));
        }
    }
    Ok(names)
}

fn check_lifetimes<'a>(
    lifetimes: &[token::SingleQuoteLowerName<'a>],
    global_lifetimes: &HashMap<&str, token::SingleQuoteLowerName<'a>>,
) -> Result<(), PreflightCheckError<'a>> {
    get_lifetime_names(lifetimes)?;
    for &name in lifetimes {
        if !global_lifetimes.contains_key(name.raw.raw()) {
            return Err(PreflightCheckError::UndefinedLifetime(name));
        }
    }
    Ok(())
}

fn path_fragment<'a>(source: &'a str, path: &cst::Path<'a>) -> SourceFragment<'a> {
    let name_fragment = |name: &cst::Name<'a>| match name {
        cst::Name::Upper(name) => name.raw,
        cst::Name::Lower(name) => name.raw,
    };
    let first = name_fragment(path.first().expect("Parsed paths are nonempty"));
    let last = name_fragment(path.last().expect("Parsed paths are nonempty"));
    let file_source = FileSource::new(source);
    let start = file_source
        .substr_range(first)
        .expect("Parsed names must be fragments of their source file")
        .start;
    let end = file_source
        .substr_range(last)
        .expect("Parsed names must be fragments of their source file")
        .end;
    SourceFragment::new(&source[start..end])
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SymbolId {
    raw: NonZeroUsize,
}

impl SymbolId {
    const HIGH_BIT_MASK: usize = 1 << (usize::BITS - 1);

    pub const fn as_terminal(self) -> Result<TerminalId, NonTerminalId> {
        let Self { raw } = self;

        if (raw.get() & Self::HIGH_BIT_MASK) == 0 {
            Ok(TerminalId { raw })
        } else {
            Err(NonTerminalId { raw })
        }
    }

    pub const fn as_non_terminal(self) -> Result<NonTerminalId, TerminalId> {
        let Self { raw } = self;

        if (raw.get() & Self::HIGH_BIT_MASK) == 0 {
            Err(TerminalId { raw })
        } else {
            Ok(NonTerminalId { raw })
        }
    }

    pub const fn from_terminal(id: TerminalId) -> Self {
        Self { raw: id.raw }
    }

    pub const fn from_non_terminal(id: NonTerminalId) -> Self {
        Self { raw: id.raw }
    }
}
impl From<TerminalId> for SymbolId {
    fn from(id: TerminalId) -> Self {
        Self::from_terminal(id)
    }
}
impl From<NonTerminalId> for SymbolId {
    fn from(id: NonTerminalId) -> Self {
        Self::from_non_terminal(id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TerminalId {
    /// INVARIANT: The most significant bit is zero.
    raw: NonZeroUsize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonTerminalId {
    /// INVARIANT: The most significant bit is one.
    raw: NonZeroUsize,
}

pub struct TerminalIdMap<T> {
    raw: Vec<T>,
}

impl<T> Index<TerminalId> for TerminalIdMap<T> {
    type Output = T;

    fn index(&self, index: TerminalId) -> &Self::Output {
        let raw_index = index.raw.get();
        let index = raw_index - 1;
        &self.raw[index]
    }
}

pub struct OriginalNonTerminalIdMap<T> {
    raw: Vec<T>,
}

impl<T> Index<NonTerminalId> for OriginalNonTerminalIdMap<T> {
    type Output = T;

    fn index(&self, index: NonTerminalId) -> &Self::Output {
        let raw_index = index.raw.get() & !SymbolId::HIGH_BIT_MASK;

        // _Original_ non-terminals have a non-zero index.
        // The zeroth index is reserved for the synthetic start symbol.
        let index = raw_index.checked_sub(1).unwrap();
        &self.raw[index]
    }
}

pub struct Rule {
    pub lhs: NonTerminalId,
    pub rhs: Vec<SymbolId>,
}

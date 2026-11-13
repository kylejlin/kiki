use crate::internal_prelude::*;

pub(crate) struct TakeoffCertificate<'a> {
    pub preflight: PreflightCertificate<'a>,

    /// IDs follow declaration order, starting at 1.
    pub terminal_ids: HashMap<SigilFreeStr<'a>, TerminalId>,

    /// Rule IDs index this vector: augmented at 0, then original rules in declaration order.
    pub rules: Vec<Rule<'a>>,

    /// For `enum` and `manual` items, the vectors correspond to the variant order.
    /// For `struct` items, the vector is a singleton.
    pub lhs_to_rule_ids: HashMap<SigilFreeStr<'a>, Vec<RuleId>>,

    pub first_sets: HashMap<GrammarSymbol<'a>, EpsilonOrTerminalSet>,
}

#[derive(Debug, Clone, Copy)]
pub enum EpsilonOrTerminal {
    Epsilon,
    Terminal(TerminalId),
}
impl PartialEq for EpsilonOrTerminal {
    fn eq(&self, other: &Self) -> bool {
        self.as_usize().eq(&other.as_usize())
    }
}
impl Eq for EpsilonOrTerminal {}
impl PartialOrd for EpsilonOrTerminal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for EpsilonOrTerminal {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_usize().cmp(&other.as_usize())
    }
}
impl Hash for EpsilonOrTerminal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_usize().hash(state);
    }
}

impl EpsilonOrTerminal {
    pub const fn as_usize(self) -> usize {
        match self {
            Self::Epsilon => 0,
            Self::Terminal(id) => id.raw.get(),
        }
    }

    pub const fn from_usize(n: usize) -> Self {
        match NonZeroUsize::new(n) {
            None => Self::Epsilon,
            Some(n) => Self::Terminal(TerminalId { raw: n }),
        }
    }

    pub const fn terminal(self) -> Option<TerminalId> {
        match self {
            Self::Epsilon => None,
            Self::Terminal(id) => Some(id),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TerminalId {
    pub raw: NonZeroUsize,
}
impl Debug for TerminalId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.raw.fmt(f)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RuleId {
    Augmented,
    Original(OriginalRuleId),
}

impl PartialEq for RuleId {
    fn eq(&self, other: &Self) -> bool {
        self.as_usize().eq(&other.as_usize())
    }
}
impl Eq for RuleId {}
impl PartialOrd for RuleId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for RuleId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_usize().cmp(&other.as_usize())
    }
}
impl Hash for RuleId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_usize().hash(state);
    }
}

impl RuleId {
    pub const fn as_usize(self) -> usize {
        match self {
            Self::Augmented => 0,
            Self::Original(id) => id.raw.get(),
        }
    }

    pub const fn from_usize(n: usize) -> Self {
        match NonZeroUsize::new(n) {
            None => Self::Augmented,
            Some(n) => Self::Original(OriginalRuleId { raw: n }),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OriginalRuleId {
    pub raw: NonZeroUsize,
}
impl Debug for OriginalRuleId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.raw.fmt(f)
    }
}

#[derive(Clone, Copy)]
pub enum Rule<'a> {
    Augmented,
    Struct(&'a m::Struct),
    EnumVariant(&'a m::Enum, &'a m::EnumVariant),
    ManualVariant(&'a m::Manual, &'a m::ManualVariant),
}

impl<'a> Rule<'a> {
    /// Returns `None` if and only if `self == Augmented`.
    pub(crate) fn lhs(self, takeoff: &'a TakeoffCertificate<'a>) -> Option<SigilFreeStr<'a>> {
        let source = takeoff.preflight.source;

        Some(match self {
            Self::Augmented => return None,

            Self::Struct(s) => sigil_free_name(source, s.name.raw),

            Self::EnumVariant(e, _) => sigil_free_name(source, e.name.raw),

            Self::ManualVariant(m, _) => sigil_free_name(source, m.name.raw),
        })
    }

    pub(crate) fn rhs(self, takeoff: &'a TakeoffCertificate<'a>) -> RuleRhsSuffix<'a> {
        RuleRhsSuffix {
            rule: self,
            start_index: 0,
            takeoff,
        }
    }
}

#[derive(Clone)]
pub(crate) struct RuleRhsSuffix<'a> {
    pub rule: Rule<'a>,
    pub start_index: usize,
    pub takeoff: &'a TakeoffCertificate<'a>,
}

impl<'a> RuleRhsSuffix<'a> {
    pub(crate) fn get(&self, i: usize) -> Option<GrammarSymbol<'a>> {
        let i = self.start_index + i;
        let takeoff = self.takeoff;

        match self.rule {
            Rule::Augmented => {
                if i == 0 {
                    Some(GrammarSymbol {
                        raw: source_text(
                            takeoff.preflight.source,
                            takeoff.preflight.start_symbol.raw,
                        ),
                    })
                } else {
                    None
                }
            }

            Rule::Struct(s) => s.fieldset.symbol(i, takeoff),

            Rule::EnumVariant(_, v) => v.fieldset.symbol(i, takeoff),

            Rule::ManualVariant(_, v) => Some(v.inputs.get(i)?.symbol(takeoff)),
        }
    }

    pub(crate) fn suffix(&self, i: usize) -> Self {
        let &Self {
            rule,
            start_index,
            takeoff,
        } = self;
        Self {
            rule,
            start_index: start_index + i,
            takeoff,
        }
    }
}

impl<'a> Iterator for RuleRhsSuffix<'a> {
    type Item = GrammarSymbol<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.get(0);
        if out.is_some() {
            self.start_index += 1;
        }
        out
    }
}

impl m::Fieldset {
    fn symbol<'a>(
        &self,
        i: usize,
        takeoff: &'a TakeoffCertificate<'a>,
    ) -> Option<GrammarSymbol<'a>> {
        Some(match self {
            m::Fieldset::Empty => return None,
            m::Fieldset::Named(f) => f.fields.get(i)?.symbol.source_text(takeoff),
            m::Fieldset::Tuple(f) => f.fields.get(i)?.symbol(takeoff),
        })
    }
}

impl m::GrammarSymbol {
    fn source_text<'a>(&self, takeoff: &'a TakeoffCertificate<'a>) -> GrammarSymbol<'a> {
        let source = takeoff.preflight.source;
        let raw = match self {
            m::GrammarSymbol::Upper(span) => source_text(source, span.raw),
            m::GrammarSymbol::DollarUpper(span) => source_text(source, span.raw),
        };
        GrammarSymbol { raw }
    }
}

impl m::TupleField {
    fn symbol<'a>(&self, takeoff: &'a TakeoffCertificate<'a>) -> GrammarSymbol<'a> {
        match self {
            m::TupleField::Used(span) | m::TupleField::Skipped(span) => span.source_text(takeoff),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GrammarSymbol<'a> {
    pub raw: &'a str,
}
impl Debug for GrammarSymbol<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.raw, f)
    }
}
impl<'a> PartialEq<GrammarSymbol<'a>> for SigilFreeStr<'a> {
    fn eq(&self, other: &GrammarSymbol<'a>) -> bool {
        self.raw() == other.raw
    }
}
impl<'a> PartialEq<SigilFreeStr<'a>> for GrammarSymbol<'a> {
    fn eq(&self, other: &SigilFreeStr<'a>) -> bool {
        self.raw == other.raw()
    }
}
impl<'a> GrammarSymbol<'a> {
    pub fn is_terminal(self) -> bool {
        self.raw.starts_with('$')
    }

    pub fn as_non_terminal(self) -> Option<SigilFreeStr<'a>> {
        SigilFreeStr::new(self.raw)
    }
}

impl<'a> TakeoffCertificate<'a> {
    pub fn new(preflight: PreflightCertificate<'a>) -> Self {
        let source = preflight.source;
        let terminal = preflight.terminal;
        let file = preflight.file;
        Self {
            preflight,
            terminal_ids: get_terminal_ids(source, terminal),
            rules: get_rules(file),
            lhs_to_rule_ids: get_lhs_to_rule_ids_map(source, file),
            first_sets: todo!(),
        }
    }
}

fn get_terminal_ids<'a>(
    source: &'a str,
    terminal: &m::TerminalEnum,
) -> HashMap<SigilFreeStr<'a>, TerminalId> {
    let m::TerminalEnum {
        prefix: _,
        name: _,
        variants,
    } = terminal;

    variants
        .iter()
        .enumerate()
        .map(|(index, m::TerminalEnumVariant { name, override_: _ })| {
            let key = sigil_free_name(source, name.raw);
            let id = TerminalId {
                raw: NonZeroUsize::new(index + 1).unwrap(),
            };
            (key, id)
        })
        .collect()
}

fn get_rules(file: &m::File) -> Vec<Rule<'_>> {
    let mut out = vec![Rule::Augmented];

    for item in &file.items {
        match item {
            m::FileItem::Start(_) => {}
            m::FileItem::Struct(def) => out.push(Rule::Struct(def)),
            m::FileItem::Enum(e) => out.extend(e.variants.iter().map(|v| Rule::EnumVariant(e, v))),
            m::FileItem::Manual(m) => {
                out.extend(m.variants.iter().map(|v| Rule::ManualVariant(m, v)))
            }
            m::FileItem::Terminal(_) => {}
            m::FileItem::Bridge(_) => {}
        }
    }

    out
}

fn get_lhs_to_rule_ids_map<'a>(
    source: &'a str,
    file: &m::File,
) -> HashMap<SigilFreeStr<'a>, Vec<RuleId>> {
    let mut out = HashMap::new();
    let mut next_id = 1;

    for item in &file.items {
        let (name, count) = match item {
            m::FileItem::Start(_) => continue,
            m::FileItem::Struct(def) => (def.name, 1),
            m::FileItem::Enum(e) => (e.name, e.variants.len()),
            m::FileItem::Manual(m) => (m.name, m.variants.len()),
            m::FileItem::Terminal(_) => continue,
            m::FileItem::Bridge(_) => continue,
        };

        let key = sigil_free_name(source, name.raw);
        let ids = (next_id..next_id + count).map(RuleId::from_usize).collect();
        out.insert(key, ids);
        next_id += count;
    }

    out
}

impl<'a> Index<RuleId> for TakeoffCertificate<'a> {
    type Output = Rule<'a>;

    fn index(&self, index: RuleId) -> &Self::Output {
        &self.rules[index.as_usize()]
    }
}

impl<'a> TakeoffCertificate<'a> {
    pub(crate) fn enumerate_rules(&'a self) -> impl Iterator<Item = (RuleId, Rule<'a>)> {
        self.rules
            .iter()
            .copied()
            .enumerate()
            .map(|(i, r)| (RuleId::from_usize(i), r))
    }
}

use crate::internal_prelude::*;

pub(crate) struct TakeoffCertificate<'a> {
    pub preflight: PreflightCertificate<'a>,

    /// IDs follow declaration order, starting at 1.
    pub terminal_ids: HashMap<SigilFreeStr<'a>, TerminalId>,

    /// Rule IDs index this vector: augmented at 0, then original rules in declaration order.
    pub rules: Vec<Rule<'a>>,
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
    EnumVariant(&'a m::EnumVariant),
    ManualVariant(&'a m::ManualVariant),
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
            m::FileItem::Enum(m::Enum {
                prefix: _,
                name: _,
                variants,
            }) => out.extend(variants.iter().map(Rule::EnumVariant)),
            m::FileItem::Manual(m::Manual {
                name: _,
                override_: _,
                variants,
            }) => out.extend(variants.iter().map(Rule::ManualVariant)),
            m::FileItem::Terminal(_) => {}
            m::FileItem::Bridge(_) => {}
        }
    }

    out
}

impl<'a> Index<RuleId> for TakeoffCertificate<'a> {
    type Output = Rule<'a>;

    fn index(&self, index: RuleId) -> &Self::Output {
        &self.rules[index.as_usize()]
    }
}

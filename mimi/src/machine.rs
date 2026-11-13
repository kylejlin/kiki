use crate::internal_prelude::*;

pub struct Machine {
    pub states: Vec<State>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State {
    pub items: Oset<Item>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Item {
    pub rule_id: RuleId,

    /// The number of seen RHS symbols.
    pub dot_index: usize,

    pub lookahead: EoiOrTerminal,
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

#[derive(Debug, Clone, Copy)]
pub enum EoiOrTerminal {
    Eoi,
    Terminal(TerminalId),
}
impl PartialEq for EoiOrTerminal {
    fn eq(&self, other: &Self) -> bool {
        self.as_usize().eq(&other.as_usize())
    }
}
impl Eq for EoiOrTerminal {}
impl PartialOrd for EoiOrTerminal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for EoiOrTerminal {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_usize().cmp(&other.as_usize())
    }
}
impl Hash for EoiOrTerminal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_usize().hash(state);
    }
}

impl EoiOrTerminal {
    pub const fn as_usize(self) -> usize {
        match self {
            Self::Eoi => 0,
            Self::Terminal(id) => id.raw.get(),
        }
    }
}

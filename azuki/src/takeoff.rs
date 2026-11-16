use crate::internal_prelude::*;

pub struct TakeoffCertificate<'a> {
    pub preflight: PreflightCertificate<'a>,

    /// For `enum` and `manual` items, the vectors correspond to the variant order.
    /// For `struct` items, the vector is a singleton.
    pub lhs_to_rule_ids: OriginalNonTerminalIdMap<Vec<RuleId>>,

    /// Rule IDs follow rule order, with each rule listed once per RHS nonterminal.
    pub rhs_nonterminal_to_rule_ids: OriginalNonTerminalIdMap<Vec<RuleId>>,
    // pub non_terminal_first_sets: TerminalIdMap<EpsilonOrTerminalSet>,
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
        Debug::fmt(&self.raw, f)
    }
}

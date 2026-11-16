use crate::internal_prelude::*;

pub struct TakeoffCertificate<'a> {
    pub preflight: PreflightCertificate<'a>,

    /// For `enum` and `manual` items, the vectors correspond to the variant order.
    /// (For `struct` items, the vector is a singleton.)
    pub lhs_to_rule_ids: OriginalNonTerminalIdMap<Vec<RuleId>>,

    pub rhs_nonterminal_to_rule_ids: OriginalNonTerminalIdMap<Vec<RuleId>>,

    pub non_terminal_first_sets: TerminalIdMap<EpsilonOrTerminalSet>,
}

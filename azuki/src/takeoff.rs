use crate::internal_prelude::*;

pub struct TakeoffCertificate<'a> {
    pub preflight: PreflightCertificate<'a>,

    pub lhs_to_rule_ids: OriginalNonTerminalIdMap<Vec<RuleId>>,

    pub rhs_nonterminal_to_rule_ids: OriginalNonTerminalIdMap<Vec<RuleId>>,

    pub non_terminal_first_sets: OriginalNonTerminalIdMap<EpsilonOrTerminalSet>,
}

impl<'a> TakeoffCertificate<'a> {
    pub fn new(preflight: PreflightCertificate<'a>) -> Self {
        todo!()
    }
}

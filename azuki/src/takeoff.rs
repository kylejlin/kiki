use crate::internal_prelude::*;

pub struct TakeoffCertificate<'a> {
    pub preflight: PreflightCertificate<'a>,

    pub lhs_to_rule_ids: OriginalNonTerminalIdMap<Vec<RuleId>>,

    pub rhs_nonterminal_to_rule_ids: OriginalNonTerminalIdMap<Vec<RuleId>>,

    pub non_terminal_first_sets: OriginalNonTerminalIdMap<EpsilonOrTerminalSet>,
}

impl<'a> TakeoffCertificate<'a> {
    pub fn new(preflight: PreflightCertificate<'a>) -> Self {
        let lhs_to_rule_ids = get_lhs_to_rule_ids_map(&preflight);
        let rhs_nonterminal_to_rule_ids = get_rhs_nonterminal_to_rule_ids_map(&preflight);
        let non_terminal_first_sets =
            get_non_terminal_first_sets(&preflight, &rhs_nonterminal_to_rule_ids);

        Self {
            preflight,
            lhs_to_rule_ids,
            rhs_nonterminal_to_rule_ids,
            non_terminal_first_sets,
        }
    }
}

fn get_lhs_to_rule_ids_map(
    preflight: &PreflightCertificate<'_>,
) -> OriginalNonTerminalIdMap<Vec<RuleId>> {
    let mut out = OriginalNonTerminalIdMap::from_raw(
        iter::repeat_with(Vec::new)
            .take(preflight.non_terminal_defs.len())
            .collect(),
    );

    for (index, Rule { lhs, rhs: _ }) in preflight.rules.iter().enumerate() {
        if let Some(lhs) = lhs.into_original() {
            out[lhs].push(RuleId::from_usize(index));
        }
    }

    out
}

fn get_rhs_nonterminal_to_rule_ids_map(
    preflight: &PreflightCertificate<'_>,
) -> OriginalNonTerminalIdMap<Vec<RuleId>> {
    let mut out = OriginalNonTerminalIdMap::from_raw(
        iter::repeat_with(Vec::new)
            .take(preflight.non_terminal_defs.len())
            .collect(),
    );

    for (index, Rule { lhs: _, rhs }) in preflight.rules.iter().enumerate() {
        let rule_id = RuleId::from_usize(index);
        for symbol in rhs {
            if let Ok(non_terminal) = symbol.as_non_terminal() {
                let ids = &mut out[non_terminal];
                if ids.last() != Some(&rule_id) {
                    ids.push(rule_id);
                }
            }
        }
    }

    out
}

fn get_non_terminal_first_sets(
    preflight: &PreflightCertificate<'_>,
    rhs_nonterminal_to_rule_ids: &OriginalNonTerminalIdMap<Vec<RuleId>>,
) -> OriginalNonTerminalIdMap<EpsilonOrTerminalSet> {
    let mut out = OriginalNonTerminalIdMap::from_raw(
        iter::repeat_with(EpsilonOrTerminalSet::empty)
            .take(preflight.non_terminal_defs.len())
            .collect(),
    );

    // Exclude the augmented rule.
    let mut queue: VecDeque<RuleId> = (1..preflight.rules.len()).map(RuleId::from_usize).collect();
    let mut currently_enqueued: HashSet<RuleId> = queue.iter().copied().collect();

    let epsilon = EpsilonOrTerminalSingletonSet {
        value: EpsilonOrTerminal::Epsilon,
    };
    let mut accumulator = EpsilonOrTerminalSet::empty();

    while let Some(rule_id) = queue.pop_front() {
        currently_enqueued.remove(&rule_id);

        let Rule { lhs, rhs } = &preflight[rule_id];
        let lhs = lhs
            .into_original()
            .expect("`queue` should never contain the augmented rule");

        accumulator.clear();
        accumulator |= epsilon;

        for symbol in rhs {
            accumulator -= epsilon;

            match symbol.as_terminal() {
                Ok(terminal) => {
                    accumulator |= EpsilonOrTerminalSingletonSet {
                        value: EpsilonOrTerminal::Terminal(terminal),
                    };
                    break;
                }
                Err(non_terminal) => {
                    let first_set = &out[non_terminal];
                    accumulator |= first_set;
                    if !first_set.contains(EpsilonOrTerminal::Epsilon) {
                        break;
                    }
                }
            }
        }

        let lhs_first_set = &mut out[lhs];
        let original_cardinality = lhs_first_set.cardinality();
        *lhs_first_set |= &accumulator;
        if lhs_first_set.cardinality() != original_cardinality {
            for &dependent_rule_id in &rhs_nonterminal_to_rule_ids[lhs] {
                if dependent_rule_id == RuleId::Augmented {
                    continue;
                }
                if currently_enqueued.insert(dependent_rule_id) {
                    queue.push_back(dependent_rule_id);
                }
            }
        }
    }

    out
}

use crate::internal_prelude::*;

pub struct TakeoffCertificate<'a> {
    pub preflight: PreflightCertificate<'a>,

    pub lhs_to_rule_ids: OriginalNonTerminalIdMap<Vec<RuleId>>,

    pub rhs_nonterminal_to_rule_ids: OriginalNonTerminalIdMap<Vec<RuleId>>,

    pub non_terminal_first_sets: OriginalNonTerminalIdMap<EpsilonOrTerminalSet>,

    pub terminal_lifetime_sets: TerminalIdMap<LifetimeSet>,

    /// `struct` and `enum` items have their lifetime sets inferred from their stored (i.e., non-`_`) fields.
    pub non_terminal_lifetime_sets: OriginalNonTerminalIdMap<LifetimeSet>,
}

impl<'a> TakeoffCertificate<'a> {
    pub fn new(preflight: PreflightCertificate<'a>) -> Self {
        let lhs_to_rule_ids = get_lhs_to_rule_ids_map(&preflight);
        let rhs_nonterminal_to_rule_ids = get_rhs_nonterminal_to_rule_ids_map(&preflight);
        let non_terminal_first_sets =
            get_non_terminal_first_sets(&preflight, &rhs_nonterminal_to_rule_ids);
        let lifetime_ids: HashMap<_, _> = preflight
            .terminal_enum
            .global_lifetimes()
            .iter()
            .enumerate()
            .map(|(i, name)| (name.raw.raw(), LifetimeId { raw: i }))
            .collect();
        let terminal_lifetime_sets = get_terminal_lifetime_sets(&preflight, &lifetime_ids);
        let non_terminal_lifetime_sets = get_non_terminal_lifetime_sets(
            &preflight,
            &lhs_to_rule_ids,
            &rhs_nonterminal_to_rule_ids,
            &terminal_lifetime_sets,
            &lifetime_ids,
        );

        Self {
            preflight,
            lhs_to_rule_ids,
            rhs_nonterminal_to_rule_ids,
            non_terminal_first_sets,
            terminal_lifetime_sets,
            non_terminal_lifetime_sets,
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

fn get_terminal_lifetime_sets(
    preflight: &PreflightCertificate<'_>,
    lifetime_ids: &HashMap<&str, LifetimeId>,
) -> TerminalIdMap<LifetimeSet> {
    TerminalIdMap::from_raw(
        preflight
            .terminal_enum
            .variants()
            .iter()
            .map(
                |cst::TerminalEnumVariant {
                     name: _,
                     override_: _,
                     lifetimes,
                 }| get_declared_lifetime_set(lifetimes, lifetime_ids),
            )
            .collect(),
    )
}

fn get_non_terminal_lifetime_sets(
    preflight: &PreflightCertificate<'_>,
    lhs_to_rule_ids: &OriginalNonTerminalIdMap<Vec<RuleId>>,
    rhs_nonterminal_to_rule_ids: &OriginalNonTerminalIdMap<Vec<RuleId>>,
    terminal_lifetime_sets: &TerminalIdMap<LifetimeSet>,
    lifetime_ids: &HashMap<&str, LifetimeId>,
) -> OriginalNonTerminalIdMap<LifetimeSet> {
    let mut out = OriginalNonTerminalIdMap::from_raw(
        iter::repeat_with(LifetimeSet::empty)
            .take(preflight.non_terminal_defs.len())
            .collect(),
    );
    let mut stored_symbols = OriginalNonTerminalIdMap::from_raw(
        iter::repeat_with(Vec::new)
            .take(preflight.non_terminal_defs.len())
            .collect(),
    );

    for (lhs, item) in preflight.non_terminal_defs.enumerate() {
        let rule_ids = &lhs_to_rule_ids[lhs];
        match item {
            cst::FileItem::Start(_) => {
                unreachable!("Non-terminal definitions cannot be start items")
            }
            cst::FileItem::Struct(cst::Struct {
                prefix: _,
                name: _,
                fieldset,
            }) => {
                stored_symbols[lhs] = get_stored_symbols(fieldset, &preflight[rule_ids[0]].rhs);
            }
            cst::FileItem::Enum(cst::Enum {
                prefix: _,
                name: _,
                variants,
            }) => {
                for (cst::EnumVariant { name: _, fieldset }, &rule_id) in
                    variants.iter().zip(rule_ids)
                {
                    stored_symbols[lhs]
                        .extend(get_stored_symbols(fieldset, &preflight[rule_id].rhs));
                }
            }
            cst::FileItem::ManualNonTerminal(cst::ManualNonTerminal {
                name: _,
                override_: _,
                lifetimes,
                variants: _,
            }) => {
                // A manual non-terminal type's lifetimes are independent of its conversion inputs.
                out[lhs] = get_declared_lifetime_set(lifetimes, lifetime_ids);
            }
            cst::FileItem::GeneratedTerminal(_) => {
                unreachable!("Non-terminal definitions cannot be generated terminal items")
            }
            cst::FileItem::ManualTerminal(_) => {
                unreachable!("Non-terminal definitions cannot be manual terminal items")
            }
            cst::FileItem::Bridge(_) => {
                unreachable!("Non-terminal definitions cannot be bridge items")
            }
        }
    }

    // Manual non-terminal types have no stored symbols and are never enqueued.
    let mut queue: VecDeque<OriginalNonTerminalId> = stored_symbols
        .enumerate()
        .filter_map(|(id, symbols)| (!symbols.is_empty()).then_some(id))
        .collect();
    let mut currently_enqueued: HashSet<OriginalNonTerminalId> = queue.iter().copied().collect();
    let mut accumulator = LifetimeSet::empty();

    while let Some(lhs) = queue.pop_front() {
        currently_enqueued.remove(&lhs);
        accumulator.clear();

        for symbol in &stored_symbols[lhs] {
            match symbol.as_terminal() {
                Ok(terminal) => accumulator |= &terminal_lifetime_sets[terminal],
                Err(non_terminal) => accumulator |= &out[non_terminal],
            }
        }

        let lhs_lifetime_set = &mut out[lhs];
        let original_cardinality = lhs_lifetime_set.cardinality();
        *lhs_lifetime_set |= &accumulator;
        if lhs_lifetime_set.cardinality() != original_cardinality {
            for &dependent_rule_id in &rhs_nonterminal_to_rule_ids[lhs] {
                let Some(dependent) = preflight[dependent_rule_id].lhs.into_original() else {
                    continue;
                };
                if !stored_symbols[dependent].is_empty() && currently_enqueued.insert(dependent) {
                    queue.push_back(dependent);
                }
            }
        }
    }

    out
}

fn get_declared_lifetime_set(
    lifetimes: &[token::SingleQuoteLowerName<'_>],
    lifetime_ids: &HashMap<&str, LifetimeId>,
) -> LifetimeSet {
    let mut out = LifetimeSet::empty();
    for name in lifetimes {
        out |= LifetimeSingletonSet {
            value: lifetime_ids[name.raw.raw()],
        };
    }
    out
}

fn get_stored_symbols(
    fieldset: &cst::Fieldset<'_>,
    rhs: &[OriginalSymbolId],
) -> Vec<OriginalSymbolId> {
    match fieldset {
        cst::Fieldset::Empty => Vec::new(),
        cst::Fieldset::Named(cst::NamedFieldset { fields }) => fields
            .iter()
            .zip(rhs)
            .filter_map(|(cst::NamedField { name, symbol: _ }, &id)| match name {
                cst::UnderscoreOrLowerName::Underscore(_) => None,
                cst::UnderscoreOrLowerName::Lower(_) => Some(id),
            })
            .collect(),
        cst::Fieldset::Tuple(cst::TupleFieldset { fields }) => fields
            .iter()
            .zip(rhs)
            .filter_map(|(field, &id)| match field {
                cst::TupleField::Used(_) => Some(id),
                cst::TupleField::Skipped(_) => None,
            })
            .collect(),
    }
}

impl Index<RuleId> for TakeoffCertificate<'_> {
    type Output = Rule;

    fn index(&self, index: RuleId) -> &Self::Output {
        &self.preflight[index]
    }
}

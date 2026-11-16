use crate::internal_prelude::*;

pub struct Machine {
    pub states: Oset<State>,
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

impl Machine {
    pub fn new(takeoff: &TakeoffCertificate) -> Self {
        use std::collections::hash_map::Entry;

        let initial_state = closure(
            iter::once(Item {
                rule_id: RuleId::Augmented,
                dot_index: 0,
                lookahead: EoiOrTerminal::Eoi,
            }),
            takeoff,
        );

        // Normally, we'd use `HashSet`.
        // However, at the time of writing, `HashSet::entry` is not available on stable.
        // So, we use `HashMap<State, ()>` instead.
        let mut states = HashMap::from([(initial_state.clone(), ())]);

        let mut queue = VecDeque::from([initial_state]);

        while let Some(start) = queue.pop_front() {
            for symbol_id in start.next_symbols(takeoff) {
                let dest = goto(&start, symbol_id, takeoff);
                assert_ne!(dest.items.len(), 0);

                match states.entry(dest) {
                    Entry::Vacant(entry) => {
                        queue.push_back(entry.key().clone());
                        entry.insert(());
                    }

                    // `dest` was already added, so no need to do anything.
                    Entry::Occupied(_) => {}
                }
            }
        }

        Self {
            states: states.into_keys().collect(),
        }
    }
}

fn closure<I>(items: I, takeoff: &TakeoffCertificate) -> State
where
    I: IntoIterator<Item = Item>,
{
    let mut current: HashSet<Item> = items.into_iter().collect();
    let mut queue: VecDeque<Item> = current.iter().copied().collect();

    while let Some(item) = queue.pop_front() {
        let Some(next) = item.next(takeoff).and_then(|s| s.as_non_terminal().ok()) else {
            continue;
        };

        let first = first_set(item.rhs_after_next(takeoff), takeoff);

        for rule_id in takeoff.lhs_to_rule_ids[next].iter().copied() {
            for lookahead in first.iter().filter_map(EpsilonOrTerminal::into_terminal) {
                let item = Item {
                    rule_id,
                    dot_index: 0,
                    lookahead: EoiOrTerminal::Terminal(lookahead),
                };
                let was_newly_inserted = current.insert(item);
                if was_newly_inserted {
                    queue.push_back(item);
                }
            }

            if first.contains(EpsilonOrTerminal::Epsilon) {
                let item = Item {
                    rule_id,
                    dot_index: 0,
                    lookahead: item.lookahead,
                };
                let was_newly_inserted = current.insert(item);
                if was_newly_inserted {
                    queue.push_back(item);
                }
            }
        }
    }

    State {
        items: current.into_iter().collect(),
    }
}

impl Item {
    fn next<'a>(self, takeoff: &'a TakeoffCertificate<'a>) -> Option<OriginalSymbolId> {
        let rule = &takeoff[self.rule_id];
        rule.rhs.get(self.dot_index).copied()
    }

    fn rhs_after_next<'a>(
        self,
        takeoff: &'a TakeoffCertificate<'a>,
    ) -> impl Iterator<Item = OriginalSymbolId> {
        let rule = &takeoff[self.rule_id];
        let i = self.dot_index + 1;
        rule.rhs.get(i..).unwrap_or(&[]).iter().copied()
    }
}

fn first_set<'a, I>(symbols: I, takeoff: &TakeoffCertificate) -> EpsilonOrTerminalSet
where
    I: IntoIterator<Item = OriginalSymbolId>,
{
    let epsilon = EpsilonOrTerminalSingletonSet {
        value: EpsilonOrTerminal::Epsilon,
    };
    let mut out = EpsilonOrTerminalSet::empty();

    out |= epsilon;

    for symbol in symbols {
        out -= epsilon;

        match symbol.as_terminal() {
            Ok(terminal) => {
                out |= EpsilonOrTerminalSingletonSet {
                    value: EpsilonOrTerminal::Terminal(terminal),
                };
                break;
            }
            Err(non_terminal) => {
                let first_set = &takeoff.non_terminal_first_sets[non_terminal];
                out |= first_set;
                if !first_set.contains(EpsilonOrTerminal::Epsilon) {
                    break;
                }
            }
        }
    }

    out
}

fn goto(start: &State, symbol_id: OriginalSymbolId, takeoff: &TakeoffCertificate) -> State {
    todo!()
}

impl State {
    fn next_symbols(&self, takeoff: &TakeoffCertificate) -> impl Iterator<Item = OriginalSymbolId> {
        let out: HashSet<OriginalSymbolId> = self
            .items
            .iter()
            .filter_map(|item| item.next(takeoff))
            .collect();
        out.into_iter()
    }
}

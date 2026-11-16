use super::*;

pub struct NonNormalizedMachine {
    pub states: Vec<State>,
    pub transitions: Vec<NonNormalizedTransition>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct StateCore {
    items: Oset<ItemCore>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ItemCore {
    rule_id: RuleId,
    dot_index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonNormalizedTransition {
    pub start_id: NonNormalizedStateId,
    pub dest_id: NonNormalizedStateId,
    pub symbol_id: OriginalSymbolId,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonNormalizedStateId {
    pub raw: usize,
}
impl NonNormalizedStateId {
    pub const START: Self = Self { raw: 0 };
}
impl Debug for NonNormalizedStateId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.raw, f)
    }
}

impl NonNormalizedMachine {
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

        let mut ids = HashMap::from([(initial_state.core(), NonNormalizedStateId::START)]);
        let mut states = vec![initial_state];
        let mut queue = VecDeque::from([NonNormalizedStateId::START]);
        let mut currently_enqueued = vec![true];
        let mut transitions = HashSet::new();

        while let Some(start_id) = queue.pop_front() {
            currently_enqueued[start_id.raw] = false;

            let start = &states[start_id.raw];
            for symbol_id in start.next_symbols(takeoff) {
                let start = &states[start_id.raw];
                let dest = goto(start, symbol_id, takeoff);
                assert_ne!(dest.items.len(), 0);

                let dest_id = {
                    let fresh_id = NonNormalizedStateId { raw: states.len() };

                    match ids.entry(dest.core()) {
                        Entry::Vacant(entry) => {
                            states.push(dest);
                            queue.push_back(fresh_id);
                            currently_enqueued.push(true);
                            entry.insert(fresh_id);
                            fresh_id
                        }

                        Entry::Occupied(entry) => {
                            let id = *entry.get();
                            let existing = &mut states[id.raw];
                            let original_item_count = existing.items.len();

                            // Both states are closed, so their union is already closed.
                            existing.items.extend(dest.items);

                            if existing.items.len() != original_item_count
                                && !currently_enqueued[id.raw]
                            {
                                currently_enqueued[id.raw] = true;
                                queue.push_back(id);
                            }

                            id
                        }
                    }
                };

                transitions.insert(NonNormalizedTransition {
                    start_id,
                    dest_id,
                    symbol_id,
                });
            }
        }

        Self {
            states,
            transitions: transitions.into_iter().collect(),
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
        let Some(next) = item
            .next_symbol(takeoff)
            .and_then(|s| s.as_non_terminal().ok())
        else {
            continue;
        };

        let first = first_set(item.rhs_after_next_symbol(takeoff), takeoff);

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
    fn next_symbol(self, takeoff: &TakeoffCertificate) -> Option<OriginalSymbolId> {
        let rule = &takeoff[self.rule_id];
        rule.rhs.get(self.dot_index).copied()
    }

    fn rhs_after_next_symbol(
        self,
        takeoff: &TakeoffCertificate,
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
    let items_after_consuming_symbol_id = start.items.iter().filter_map(|item| {
        if item.next_symbol(takeoff) == Some(symbol_id) {
            let Item {
                rule_id,
                dot_index,
                lookahead,
            } = *item;

            Some(Item {
                rule_id,
                dot_index: dot_index + 1,
                lookahead,
            })
        } else {
            None
        }
    });

    closure(items_after_consuming_symbol_id, takeoff)
}

impl State {
    fn core(&self) -> StateCore {
        StateCore {
            items: self
                .items
                .iter()
                .map(
                    |&Item {
                         rule_id,
                         dot_index,
                         lookahead: _,
                     }| ItemCore { rule_id, dot_index },
                )
                .collect(),
        }
    }

    fn next_symbols(&self, takeoff: &TakeoffCertificate) -> HashSet<OriginalSymbolId> {
        self.items
            .iter()
            .filter_map(|item| item.next_symbol(takeoff))
            .collect()
    }
}

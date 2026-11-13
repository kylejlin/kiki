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

impl Machine {
    pub fn new(takeoff: &TakeoffCertificate) -> Self {
        let initial_state = closure(
            iter::once(Item {
                rule_id: RuleId::Augmented,
                dot_index: 0,
                lookahead: EoiOrTerminal::Eoi,
            }),
            takeoff,
        );

        todo!()
    }
}

fn closure<I>(items: I, takeoff: &TakeoffCertificate) -> State
where
    I: IntoIterator<Item = Item>,
{
    let mut current: HashSet<Item> = items.into_iter().collect();
    let mut queue: VecDeque<Item> = current.iter().copied().collect();

    while let Some(item) = queue.pop_front() {
        let Some(next) = item.next(takeoff).and_then(GrammarSymbol::as_non_terminal) else {
            continue;
        };

        let first = first_set(item.rhs_after_next(takeoff), takeoff);

        for rule_id in takeoff.lhs_to_rule_ids[&next].iter().copied() {
            for lookahead in first.iter().filter_map(EpsilonOrTerminal::terminal) {
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
    fn next<'a>(self, takeoff: &'a TakeoffCertificate<'a>) -> Option<GrammarSymbol<'a>> {
        let rule = takeoff[self.rule_id];
        rule.rhs(takeoff).get(self.dot_index)
    }

    fn rhs_after_next<'a>(
        self,
        takeoff: &'a TakeoffCertificate<'a>,
    ) -> impl Iterator<Item = GrammarSymbol<'a>> {
        let rule = takeoff[self.rule_id];
        rule.rhs(takeoff).suffix(self.dot_index + 1)
    }
}

fn first_set<'a, I>(symbols: I, takeoff: &TakeoffCertificate) -> EpsilonOrTerminalSet
where
    I: Iterator<Item = GrammarSymbol<'a>>,
{
    todo!()
}

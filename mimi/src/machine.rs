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
        let item = vec![Item {
            rule_id: RuleId::Augmented,
            dot_index: 0,
            lookahead: EoiOrTerminal::Eoi,
        }];

        todo!()
    }
}

fn closure<'a, I>(items: &'a I, takeoff: &TakeoffCertificate) -> State
where
    &'a I: IntoIterator<Item = &'a Item>,
{
    let mut seen: HashSet<Item> = items.into_iter().copied().collect();
    let mut current: Vec<Item> = items.into_iter().copied().collect();

    loop {
        for item in items {
            // let rule = takeoff[item.rule_id];
        }
    }
    todo!()
}

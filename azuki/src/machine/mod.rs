use crate::internal_prelude::*;

pub(crate) use non_normalized::*;

mod non_normalized;
mod normalization;

pub struct Machine {
    pub start_id: StateId,
    pub states: Oset<State>,
    pub transitions: Oset<Transition>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Transition {
    pub start_id: StateId,
    pub dest_id: StateId,
    pub symbol_id: OriginalSymbolId,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateId {
    pub raw: usize,
}
impl Debug for StateId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.raw, f)
    }
}

impl Machine {
    pub fn new(takeoff: &TakeoffCertificate) -> Self {
        let non_normalized = NonNormalizedMachine::new(takeoff);
        non_normalized.normalize()
    }

    pub fn enumerate_states<'a>(&'a self) -> impl Iterator<Item = (StateId, &'a State)> {
        self.states
            .iter()
            .enumerate()
            .map(|(i, s)| (StateId { raw: i }, s))
    }
}

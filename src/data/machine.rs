use crate::data::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Machine {
    pub start: StateIndex,
    pub states: Oset<State>,
    pub transitions: Oset<Transition>,
}

impl Machine {
    pub fn get_shift_dest(
        &self,
        start: StateIndex,
        terminal: &DollarlessTerminalName,
    ) -> Option<StateIndex> {
        self.transitions.iter().find_map(|t| {
            if t.from == start && t.symbol == *terminal {
                Some(t.to)
            } else {
                None
            }
        })
    }
}

impl PartialEq<DollarlessTerminalName> for Symbol {
    fn eq(&self, other: &DollarlessTerminalName) -> bool {
        match self {
            Symbol::Terminal(t) => t == other,
            Symbol::Nonterminal(_) => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State {
    pub items: Oset<StateItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateItem {
    pub rule_index: RuleIndex,
    pub lookahead: Lookahead,
    /// The `dot` is the index of the symbol to the right of the dot.
    /// If the dot is at the end of the RHS, then `dot == right.len()`.
    pub dot: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RuleIndex {
    Original(usize),
    Augmented,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Lookahead {
    Terminal(DollarlessTerminalName),
    Eof,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Transition {
    pub from: StateIndex,
    pub to: StateIndex,
    pub symbol: Symbol,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateIndex(pub usize);

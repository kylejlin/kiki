use crate::data::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Machine {
    pub start: StateIndex,
    pub states: Oset<State>,
    pub transitions: Oset<Transition>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State {
    pub items: Oset<Item>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Item {
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

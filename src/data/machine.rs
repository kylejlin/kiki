use crate::data::Oset;

#[derive(Debug, Clone)]
pub struct Machine {
    /// The first state is the start state.
    pub states: Vec<State>,
    pub transitions: Vec<Transition>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    pub items: Oset<Item>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Item {
    pub left: String,
    pub right: Vec<String>,
    pub lookahead: String,
    /// The `dot` is the index of the symbol to the right of the dot.
    /// If the dot is at the end of the RHS, then `dot == right.len()`.
    pub dot: usize,
}

#[derive(Debug, Clone)]
pub struct Transition {
    pub from: usize,
    pub to: usize,
    pub symbol: String,
}

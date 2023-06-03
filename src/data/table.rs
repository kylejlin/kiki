use crate::data::*;

#[derive(Debug, Clone)]
pub struct Table {
    pub start: StateIndex,
    pub terminals: Vec<DollarlessTerminalName>,
    pub nonterminals: Vec<String>,
    pub actions: Vec<Action>,
    pub gotos: Vec<Goto>,
}

pub use machine::StateIndex;

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Shift(usize),
    Reduce(usize),
    Accept,
    Err,
}

#[derive(Debug, Clone, Copy)]
pub enum Goto {
    State(usize),
    Err,
}

#[derive(Debug, Clone, Copy)]
pub enum Quasiterminal<'a> {
    Terminal(&'a DollarlessTerminalName),
    Eof,
}

impl Table {
    pub fn state_count(&self) -> usize {
        self.actions.len() / (self.terminals.len() + 1)
    }

    /// ## Panics
    /// 1. Panics if the terminal is not in the table.
    /// 2. Panics if the state is too large.
    pub fn action(&self, state_index: StateIndex, terminal: Quasiterminal) -> Action {
        let i = self.action_index(state_index, terminal);
        self.actions[i]
    }

    /// ## Panics
    /// 1. Panics if the terminal is not in the table.
    /// 2. Panics if the state is too large.
    pub fn set_action(&mut self, state_index: StateIndex, terminal: Quasiterminal, val: Action) {
        let i = self.action_index(state_index, terminal);
        self.actions[i] = val;
    }

    /// ## Panics
    /// 1. Panics if the terminal is not in the table.
    /// 2. Panics if the state is too large.
    fn action_index(
        &self,
        StateIndex(state_index): StateIndex,
        quasiterminal: Quasiterminal,
    ) -> usize {
        let quasiterminal_index = match quasiterminal {
            Quasiterminal::Terminal(terminal) => self
                .terminals
                .iter()
                .position(|t| t == terminal)
                .expect("Terminal not found in table"),
            Quasiterminal::Eof => self.terminals.len(),
        };

        if state_index >= self.state_count() {
            let states = self.state_count();
            panic!("State index {state_index} is too large. There are only {states} states.");
        }

        state_index * (self.terminals.len() + 1) + quasiterminal_index
    }

    /// ## Panics
    /// 1. Panics if the nonterminal is not in the table.
    /// 2. Panics if the state is too large.
    pub fn goto(&self, state_index: StateIndex, nonterminal: &str) -> Goto {
        let i = self.goto_index(state_index, nonterminal);
        self.gotos[i]
    }

    /// ## Panics
    /// 1. Panics if the nonterminal is not in the table.
    /// 2. Panics if the state is too large.
    pub fn set_goto(&mut self, state_index: StateIndex, nonterminal: &str, val: Goto) {
        let i = self.goto_index(state_index, nonterminal);
        self.gotos[i] = val;
    }

    /// ## Panics
    /// 1. Panics if the nonterminal is not in the table.
    /// 2. Panics if the state is too large.
    fn goto_index(&self, StateIndex(state_index): StateIndex, nonterminal: &str) -> usize {
        let nonterminal_index = self
            .nonterminals
            .iter()
            .position(|t| t == nonterminal)
            .expect("Nonterminal not found in table");

        if state_index >= self.state_count() {
            let states = self.state_count();
            panic!("State index {state_index} is too large. There are only {states} states.");
        }

        state_index * self.nonterminals.len() + nonterminal_index
    }
}

use crate::data::validated_file::DollarlessTerminalName;

#[derive(Debug, Clone)]
pub struct Table {
    pub dollarless_terminals: Vec<DollarlessTerminalName>,
    pub nonterminals: Vec<String>,
    pub actions: Vec<Action>,
    pub gotos: Vec<Goto>,
}

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Shift(usize),
    Reduce(usize),
    Accept,
    Err,
}

#[derive(Debug, Clone, Copy)]
pub enum Goto {
    Goto(usize),
    Err,
}

impl Table {
    pub fn states(&self) -> usize {
        self.actions.len() / self.dollarless_terminals.len()
    }

    /// ## Panics
    /// 1. Panics if the terminal is not in the table.
    /// 2. Panics if the state is too large.
    pub fn action(&self, state: usize, terminal: &DollarlessTerminalName) -> Action {
        let i = self.action_index(state, terminal);
        self.actions[i]
    }

    /// ## Panics
    /// 1. Panics if the terminal is not in the table.
    /// 2. Panics if the state is too large.
    pub fn set_action(&mut self, state: usize, terminal: &DollarlessTerminalName, val: Action) {
        let i = self.action_index(state, terminal);
        self.actions[i] = val;
    }

    /// ## Panics
    /// 1. Panics if the terminal is not in the table.
    /// 2. Panics if the state is too large.
    fn action_index(&self, state: usize, terminal: &DollarlessTerminalName) -> usize {
        let terminal_index = self
            .dollarless_terminals
            .iter()
            .position(|t| t == terminal)
            .expect("Terminal not found in table");

        if state >= self.states() {
            let states = self.states();
            panic!("State {state} is too large. There are only {states} states.");
        }

        state * self.dollarless_terminals.len() + terminal_index
    }

    /// ## Panics
    /// 1. Panics if the nonterminal is not in the table.
    /// 2. Panics if the state is too large.
    pub fn goto(&self, state: usize, nonterminal: &str) -> Goto {
        let i = self.goto_index(state, nonterminal);
        self.gotos[i]
    }

    /// ## Panics
    /// 1. Panics if the nonterminal is not in the table.
    /// 2. Panics if the state is too large.
    pub fn set_goto(&mut self, state: usize, nonterminal: &str, val: Goto) {
        let i = self.goto_index(state, nonterminal);
        self.gotos[i] = val;
    }

    /// ## Panics
    /// 1. Panics if the nonterminal is not in the table.
    /// 2. Panics if the state is too large.
    fn goto_index(&self, state: usize, nonterminal: &str) -> usize {
        let nonterminal_index = self
            .nonterminals
            .iter()
            .position(|t| t == nonterminal)
            .expect("Nonterminal not found in table");

        if state >= self.states() {
            let states = self.states();
            panic!("State {state} is too large. There are only {states} states.");
        }

        state * self.nonterminals.len() + nonterminal_index
    }
}

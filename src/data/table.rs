#[derive(Debug, Clone)]
pub struct Table {
    pub terminals: Vec<String>,
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
        self.actions.len() / self.terminals.len()
    }

    /// ## Panics
    /// 1. Panics if the terminal is not in the table.
    /// 2. Panics if the state is too large.
    pub fn action(&self, state: usize, terminal: &str) -> Action {
        *self.action_mut(state, terminal)
    }

    /// ## Panics
    /// 1. Panics if the terminal is not in the table.
    /// 2. Panics if the state is too large.
    pub fn set_action(&self, state: usize, terminal: &str, val: Action) {
        *self.action_mut(state, terminal) = val;
    }

    /// ## Panics
    /// 1. Panics if the terminal is not in the table.
    /// 2. Panics if the state is too large.
    fn action_mut(&self, state: usize, terminal: &str) -> &mut Action {
        let terminal_index = self
            .terminals
            .iter()
            .position(|t| t == terminal)
            .expect("Terminal not found in table");

        if state >= self.states() {
            let states = self.states();
            panic!("State {state} is too large. There are only {states} states.");
        }

        &mut self.actions[state * self.terminals.len() + terminal_index]
    }

    /// ## Panics
    /// 1. Panics if the nonterminal is not in the table.
    /// 2. Panics if the state is too large.
    pub fn goto(&self, state: usize, nonterminal: &str) -> Goto {
        *self.goto_mut(state, nonterminal)
    }

    /// ## Panics
    /// 1. Panics if the nonterminal is not in the table.
    /// 2. Panics if the state is too large.
    pub fn set_goto(&self, state: usize, nonterminal: &str, val: Goto) {
        *self.goto_mut(state, nonterminal) = val;
    }

    /// ## Panics
    /// 1. Panics if the nonterminal is not in the table.
    /// 2. Panics if the state is too large.
    fn goto_mut(&self, state: usize, nonterminal: &str) -> &mut Goto {
        let nonterminal_index = self
            .nonterminals
            .iter()
            .position(|t| t == nonterminal)
            .expect("Nonterminal not found in table");

        if state >= self.states() {
            let states = self.states();
            panic!("State {state} is too large. There are only {states} states.");
        }

        &mut self.gotos[state * self.nonterminals.len() + nonterminal_index]
    }
}

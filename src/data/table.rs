#[derive(Debug, Clone)]
pub struct Table {
    pub terminals: Vec<String>,
    pub nonterminals: Vec<String>,
    pub actions: Vec<Action>,
    pub gotos: Vec<Goto>,
}

#[derive(Debug, Clone)]
pub enum Action {
    Shift(usize),
    Reduce(usize),
    Accept,
    Err,
}

#[derive(Debug, Clone)]
pub enum Goto {
    Goto(usize),
    Err,
}

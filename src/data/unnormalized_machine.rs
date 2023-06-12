use crate::data::machine::{State, Transition};

use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct UnnormalizedMachine {
    /// The first state must be the start state.
    pub states: Vec<State>,
    pub transitions: HashSet<Transition>,
}

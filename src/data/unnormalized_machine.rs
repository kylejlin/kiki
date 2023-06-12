use crate::data::machine::{State, Transition};

use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct UnnormalizedMachine {
    pub states: Vec<State>,
    pub transitions: HashSet<Transition>,
}

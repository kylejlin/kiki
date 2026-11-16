use crate::internal_prelude::*;

pub struct Table {
    start_id: StateId,
    terminal_count: usize,
    actions: Vec<Action>,
    original_non_terminal_count: usize,
    gotos: Vec<Goto>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Action {
    Shift(StateId),
    Reduce(RuleId),
    Accept,
    Err,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Goto {
    State(StateId),
    Err,
}

pub enum TableConflictError {
    // TODO
}

impl Table {
    pub fn new(
        machine: &Machine,
        takeoff: &TakeoffCertificate,
    ) -> Result<Self, TableConflictError> {
        let start_id = machine.start_id;
        let terminal_count = takeoff.preflight.terminal_defs.len();
        let original_non_terminal_count = takeoff.preflight.non_terminal_defs.len();
        let state_count = machine.states.len();
        let actions = vec![Action::Err; state_count * (terminal_count + 1)];
        let gotos = vec![Goto::Err; state_count * original_non_terminal_count];

        let mut out = Self {
            start_id,
            terminal_count,
            actions,
            original_non_terminal_count,
            gotos,
        };

        for transition in machine.transitions.iter().copied() {
            let Transition {
                start_id,
                dest_id,
                symbol_id,
            } = transition;

            match symbol_id.as_terminal() {
                Ok(terminal_id) => {
                    let entry = &mut out[(start_id, terminal_id)];

                    match *entry {
                        Action::Err => {
                            *entry = Action::Shift(dest_id);
                        }

                        Action::Shift(_) | Action::Reduce(_) | Action::Accept => {
                            return Err(todo!());
                        }
                    }
                }

                Err(non_terminal_id) => {
                    let entry = &mut out[(start_id, non_terminal_id)];

                    match *entry {
                        Goto::Err => {
                            *entry = Goto::State(dest_id);
                        }

                        Goto::State(existing) => return Err(todo!()),
                    }

                    *entry = Goto::State(dest_id);
                }
            }
        }

        for (state_id, state) in machine.enumerate_states() {
            for item in state.items.iter() {
                if item.next_symbol(takeoff).is_some() {
                    continue;
                }

                let rule = &takeoff[item.rule_id];
                let Some(lhs) = rule.lhs.into_original() else {
                    if item.lookahead == EoiOrTerminal::Eoi {
                        let entry = &mut out[(state_id, item.lookahead)];

                        match *entry {
                            Action::Err => {
                                *entry = Action::Accept;
                            }

                            Action::Shift(_) | Action::Reduce(_) | Action::Accept => {
                                return Err(todo!());
                            }
                        }
                    } else {
                        todo!()
                    }

                    continue;
                };

                let entry = &mut out[(state_id, item.lookahead)];

                match *entry {
                    Action::Err => {
                        *entry = Action::Reduce(item.rule_id);
                    }

                    Action::Shift(_) | Action::Reduce(_) | Action::Accept => return Err(todo!()),
                }
            }
        }

        todo!();

        Ok(out)
    }
}

impl Index<(StateId, EoiOrTerminal)> for Table {
    type Output = Action;

    fn index(&self, index: (StateId, EoiOrTerminal)) -> &Self::Output {
        todo!()
    }
}

impl IndexMut<(StateId, EoiOrTerminal)> for Table {
    fn index_mut(&mut self, index: (StateId, EoiOrTerminal)) -> &mut Self::Output {
        todo!()
    }
}

impl Index<(StateId, TerminalId)> for Table {
    type Output = Action;

    fn index(&self, (s, t): (StateId, TerminalId)) -> &Self::Output {
        &self[(s, EoiOrTerminal::Terminal(t))]
    }
}

impl IndexMut<(StateId, TerminalId)> for Table {
    fn index_mut(&mut self, (s, t): (StateId, TerminalId)) -> &mut Self::Output {
        &mut self[(s, EoiOrTerminal::Terminal(t))]
    }
}

impl Index<(StateId, OriginalNonTerminalId)> for Table {
    type Output = Goto;

    fn index(&self, index: (StateId, OriginalNonTerminalId)) -> &Self::Output {
        todo!()
    }
}

impl IndexMut<(StateId, OriginalNonTerminalId)> for Table {
    fn index_mut(&mut self, index: (StateId, OriginalNonTerminalId)) -> &mut Self::Output {
        todo!()
    }
}

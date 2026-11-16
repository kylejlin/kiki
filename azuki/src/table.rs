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

/// Identifies a conflicting entry using IDs from the inputs to `Table::new`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TableConflictError {
    pub state_id: StateId,
    pub lookahead: EoiOrTerminal,
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
                    let entry = out[(start_id, terminal_id)].mut_err().expect("Shifts are inserted once per state/terminal pair, before reductions and accepts");
                    *entry = Action::Shift(dest_id);
                }

                Err(non_terminal_id) => {
                    let entry = out[(start_id, non_terminal_id)]
                        .mut_err()
                        .expect("Gotos are inserted once per state/non-terminal pair");
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
                if rule.lhs.is_synthetic() {
                    assert_eq!(
                        item.lookahead,
                        EoiOrTerminal::Eoi,
                        "The augmented item's lookahead must be EOI"
                    );

                    let entry = out[(state_id, item.lookahead)]
                        .mut_err()
                        .expect("The augmented item sorts first, and EOI cannot be shifted");
                    *entry = Action::Accept;

                    continue;
                }

                let Some(entry) = out[(state_id, item.lookahead)].mut_err() else {
                    return Err(TableConflictError {
                        state_id,
                        lookahead: item.lookahead,
                    });
                };
                *entry = Action::Reduce(item.rule_id);
            }
        }

        Ok(out)
    }
}

impl Index<(StateId, EoiOrTerminal)> for Table {
    type Output = Action;

    fn index(&self, (s, t): (StateId, EoiOrTerminal)) -> &Self::Output {
        let width = self.terminal_count + 1;
        let start = s.raw * width;
        &self.actions[start..start + width][t.into_usize()]
    }
}

impl IndexMut<(StateId, EoiOrTerminal)> for Table {
    fn index_mut(&mut self, (s, t): (StateId, EoiOrTerminal)) -> &mut Self::Output {
        let width = self.terminal_count + 1;
        let start = s.raw * width;
        &mut self.actions[start..start + width][t.into_usize()]
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

    fn index(&self, (s, n): (StateId, OriginalNonTerminalId)) -> &Self::Output {
        let width = self.original_non_terminal_count;
        let start = s.raw * width;
        &self.gotos[start..start + width][n.zero_based_index()]
    }
}

impl IndexMut<(StateId, OriginalNonTerminalId)> for Table {
    fn index_mut(&mut self, (s, n): (StateId, OriginalNonTerminalId)) -> &mut Self::Output {
        let width = self.original_non_terminal_count;
        let start = s.raw * width;
        &mut self.gotos[start..start + width][n.zero_based_index()]
    }
}

impl Action {
    fn mut_err(&mut self) -> Option<&mut Self> {
        match self {
            Self::Err => Some(self),
            _ => None,
        }
    }
}

impl Goto {
    fn mut_err(&mut self) -> Option<&mut Self> {
        match self {
            Self::Err => Some(self),
            _ => None,
        }
    }
}

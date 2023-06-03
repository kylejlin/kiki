use crate::data::{machine::*, table::*, validated_file::*, KikiErr, *};

use std::collections::HashMap;

pub fn machine_to_table(machine: &Machine, file: &File) -> Result<Table, KikiErr> {
    ImmutContext::new(machine, file).get_table()
}

struct ImmutContext<'a> {
    machine: &'a Machine,
    file: &'a File,
    rules: Vec<Rule<'a>>,
}

impl ImmutContext<'_> {
    fn new<'a>(machine: &'a Machine, file: &'a File) -> ImmutContext<'a> {
        ImmutContext {
            machine,
            file,
            rules: file.get_rules().collect(),
        }
    }
}

impl ImmutContext<'_> {
    fn get_table(&self) -> Result<Table, KikiErr> {
        let mut builder = TableBuilder::new();
        self.add_actions_to_table(&mut builder)?;
        self.add_gotos_to_table(&mut builder)?;
        Ok(self.build_as_is(builder))
    }
}

#[derive(Debug)]
struct TableBuilder<'a> {
    pub actions: HashMap<(StateIndex, Quasiterminal<'a>), (RuleIndex, Action)>,
    pub gotos: HashMap<(StateIndex, &'a str), Goto>,
}

impl TableBuilder<'_> {
    fn new<'a>() -> TableBuilder<'a> {
        TableBuilder {
            actions: HashMap::new(),
            gotos: HashMap::new(),
        }
    }
}

impl ImmutContext<'_> {
    fn add_actions_to_table(&self, builder: &mut TableBuilder) -> Result<(), KikiErr> {
        for i in 0..self.machine.states.len() {
            self.add_state_actions_to_table(builder, StateIndex(i))?;
        }
        Ok(())
    }

    fn add_state_actions_to_table(
        &self,
        builder: &mut TableBuilder,
        state_index: StateIndex,
    ) -> Result<(), KikiErr> {
        let state = &self.machine.states[state_index.0];
        for item in &state.items {
            self.add_item_action_to_table(builder, state_index, item)?;
        }
        Ok(())
    }

    fn add_item_action_to_table(
        &self,
        builder: &mut TableBuilder,
        state_index: StateIndex,
        item: &Item,
    ) -> Result<(), KikiErr> {
        match item.rule_index {
            RuleIndex::Augmented => {
                self.add_augmented_item_action_to_table(builder, state_index, item.dot)
            }
            RuleIndex::Original(rule_index) => {
                self.add_original_item_action_to_table(builder, state_index, item, rule_index)
            }
        }
    }

    fn add_augmented_item_action_to_table(
        &self,
        builder: &mut TableBuilder,
        state_index: StateIndex,
        dot: usize,
    ) -> Result<(), KikiErr> {
        if dot == 0 {
            return Ok(());
        }

        builder.set_action(
            state_index,
            Quasiterminal::Eof,
            RuleIndex::Augmented,
            Action::Accept,
        )
    }

    fn add_original_item_action_to_table(
        &self,
        builder: &mut TableBuilder,
        state_index: StateIndex,
        item: &Item,
        rule_index: usize,
    ) -> Result<(), KikiErr> {
        todo!()
    }
}

impl<'a> TableBuilder<'a> {
    fn set_action(
        &mut self,
        state_index: StateIndex,
        quasiterminal: Quasiterminal<'a>,
        rule_index: RuleIndex,
        action: Action,
    ) -> Result<(), KikiErr> {
        if let Some(_) = self.actions.get(&(state_index, quasiterminal)) {
            return Err(todo!());
        }

        self.actions
            .insert((state_index, quasiterminal), (rule_index, action));
        Ok(())
    }
}

impl ImmutContext<'_> {
    fn add_gotos_to_table(&self, builder: &mut TableBuilder) -> Result<(), KikiErr> {
        todo!()
    }
}

impl ImmutContext<'_> {
    fn build_as_is(&self, builder: TableBuilder) -> Table {
        let mut table = get_empty_table(self.machine, self.file);

        for ((state, quasiterminal), (_, action)) in builder.actions {
            table.set_action(state, quasiterminal, action);
        }

        for ((state, nonterminal), goto) in builder.gotos {
            table.set_goto(state, &nonterminal, goto);
        }

        table
    }
}

fn get_empty_table(machine: &Machine, file: &File) -> Table {
    let terminals = get_terminals(file);
    let nonterminals = get_nonterminals(file);
    let actions = get_empty_action_table(&machine.states, &terminals);
    let gotos = get_empty_goto_table(&machine.states, &nonterminals);
    Table {
        start: machine.start,
        terminals,
        nonterminals,
        actions,
        gotos,
    }
}

fn get_terminals(file: &File) -> Vec<DollarlessTerminalName> {
    file.terminal_enum
        .variants
        .iter()
        .map(|variant| variant.dollarless_name.clone())
        .collect()
}

fn get_nonterminals(file: &File) -> Vec<String> {
    file.nonterminals
        .iter()
        .map(|nonterminal| nonterminal.name().to_owned())
        .collect()
}

fn get_empty_action_table(states: &[State], terminals: &[DollarlessTerminalName]) -> Vec<Action> {
    let size = states.len() * (terminals.len() + 1);
    vec![Action::Err; size]
}

fn get_empty_goto_table(states: &[State], nonterminals: &[String]) -> Vec<Goto> {
    let size = states.len() * nonterminals.len();
    vec![Goto::Err; size]
}

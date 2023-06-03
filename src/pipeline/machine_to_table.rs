use crate::data::{machine::*, table::*, validated_file::*, KikiErr, *};

use std::collections::HashMap;

pub fn machine_to_table(machine: &Machine, file: &File) -> Result<Table, KikiErr> {
    let builder = TableBuilder::new();
    let context = ImmutContext::new(machine, file);
    builder.build(&context)
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

impl TableBuilder<'_> {
    fn build(mut self, context: &ImmutContext) -> Result<Table, KikiErr> {
        context.fill_empty_table_builder(&mut self);
        self.convert_to_table(context)
    }

    fn convert_to_table(self, context: &ImmutContext) -> Result<Table, KikiErr> {
        let mut table = get_empty_table(context.machine, context.file);

        for ((state, quasiterminal), (_, action)) in self.actions {
            table.set_action(state, quasiterminal, action);
        }

        for ((state, nonterminal), goto) in self.gotos {
            table.set_goto(state, &nonterminal, goto);
        }

        Ok(table)
    }
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
    fn fill_empty_table_builder(&self, builder: &mut TableBuilder) -> Result<(), KikiErr> {
        self.add_actions_to_table(builder)?;
        self.add_gotos_to_table(builder)?;
        Ok(())
    }

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
            self.add_item_actions_to_table(builder, state_index, item)?;
        }
        Ok(())
    }

    fn add_item_actions_to_table(
        &self,
        builder: &mut TableBuilder,
        state_index: StateIndex,
        item: &Item,
    ) -> Result<(), KikiErr> {
        todo!()
    }

    fn add_gotos_to_table(&self, builder: &mut TableBuilder) -> Result<(), KikiErr> {
        todo!()
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

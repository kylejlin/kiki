use crate::data::{machine::*, table::*, validated_file::*, KikiErr, *};

pub fn machine_to_table(machine: &Machine, file: &File) -> Result<Table, KikiErr> {
    let table = get_empty_table(machine, file);
    let context = ImmutContext::new(machine, file);
    context.fill_empty_table(table)
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

struct ImmutContext<'a> {
    machine: &'a Machine,
    rules: Vec<Rule<'a>>,
}

impl ImmutContext<'_> {
    fn new<'a>(machine: &'a Machine, file: &'a File) -> ImmutContext<'a> {
        ImmutContext {
            machine,
            rules: file.get_rules().collect(),
        }
    }
}

impl ImmutContext<'_> {
    fn fill_empty_table(&self, mut table: Table) -> Result<Table, KikiErr> {
        self.add_actions_to_table(&mut table)?;
        self.add_gotos_to_table(&mut table)?;
        Ok(table)
    }

    fn add_actions_to_table(&self, table: &mut Table) -> Result<(), KikiErr> {
        for i in 0..self.machine.states.len() {
            self.add_state_actions_to_table(table, StateIndex(i))?;
        }
        Ok(())
    }

    fn add_state_actions_to_table(
        &self,
        table: &mut Table,
        state_index: StateIndex,
    ) -> Result<(), KikiErr> {
        let state = &self.machine.states[state_index.0];
        for item in &state.items {
            self.add_item_actions_to_table(table, state_index, item)?;
        }
        Ok(())
    }

    fn add_item_actions_to_table(
        &self,
        table: &mut Table,
        state_index: StateIndex,
        item: &Item,
    ) -> Result<(), KikiErr> {
        todo!()
    }

    fn add_gotos_to_table(&self, table: &mut Table) -> Result<(), KikiErr> {
        todo!()
    }
}

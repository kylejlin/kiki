use crate::data::{machine::*, table::*, validated_file::*, KikiErr, *};

pub fn machine_to_table(machine: &Machine, file: &File) -> Result<Table, KikiErr> {
    let mut table = get_empty_table(machine, file);
    let rules: Vec<Rule> = file.get_rules().collect();
    add_actions_to_table(&mut table, machine, &rules)?;
    add_gotos_to_table(&mut table, machine, &rules)?;
    Ok(table)
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

fn add_actions_to_table(
    table: &mut Table,
    machine: &Machine,
    rules: &[Rule],
) -> Result<(), KikiErr> {
    for (state_index, state) in machine
        .states
        .iter()
        .enumerate()
        .map(|(i, state)| (StateIndex(i), state))
    {
        add_state_actions_to_table(table, state_index, state, &machine.transitions, rules)?;
    }
    Ok(())
}

fn add_state_actions_to_table(
    table: &mut Table,
    state_index: StateIndex,
    state: &State,
    transitions: &Oset<Transition>,
    rules: &[Rule],
) -> Result<(), KikiErr> {
    for item in &state.items {
        add_item_actions_to_table(table, state_index, item, transitions, rules)?;
    }
    Ok(())
}

fn add_item_actions_to_table(
    table: &mut Table,
    state_index: StateIndex,
    item: &Item,
    transitions: &Oset<Transition>,
    rules: &[Rule],
) -> Result<(), KikiErr> {
    todo!()
}

fn add_gotos_to_table(table: &mut Table, machine: &Machine, rules: &[Rule]) -> Result<(), KikiErr> {
    todo!()
}

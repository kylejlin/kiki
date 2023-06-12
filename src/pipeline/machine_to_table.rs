use crate::data::{machine::*, table::*, validated_file::*, KikiErr, *};

use std::collections::HashMap;

pub fn machine_to_table(machine: &Machine, file: &File) -> Result<Table, KikiErr> {
    ImmutContext::new(machine, file).get_table()
}

#[derive(Debug)]
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
        let mut builder = TableBuilder::new(self);
        self.add_actions_to_table(&mut builder)?;
        self.add_gotos_to_table(&mut builder);
        Ok(self.build_as_is(builder))
    }
}

#[derive(Debug)]
struct TableBuilder<'a> {
    actions: HashMap<(StateIndex, Quasiterminal<'a>), (&'a StateItem, Action)>,
    gotos: HashMap<(StateIndex, &'a str), Goto>,

    context: &'a ImmutContext<'a>,
}

impl TableBuilder<'_> {
    fn new<'a>(context: &'a ImmutContext<'a>) -> TableBuilder<'a> {
        TableBuilder {
            actions: HashMap::new(),
            gotos: HashMap::new(),

            context,
        }
    }
}

impl<'a> ImmutContext<'a> {
    fn add_actions_to_table(&self, builder: &mut TableBuilder<'a>) -> Result<(), KikiErr> {
        for i in 0..self.machine.states.len() {
            self.add_state_actions_to_table(builder, StateIndex(i))?;
        }
        Ok(())
    }

    fn add_state_actions_to_table(
        &self,
        builder: &mut TableBuilder<'a>,
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
        builder: &mut TableBuilder<'a>,
        state_index: StateIndex,
        item: &'a StateItem,
    ) -> Result<(), KikiErr> {
        match item.rule_index {
            RuleIndex::Augmented => {
                self.add_augmented_item_action_to_table(builder, state_index, item)
            }
            RuleIndex::Original(rule_index) => {
                self.add_original_item_action_to_table(builder, state_index, item, rule_index)
            }
        }
    }

    fn add_augmented_item_action_to_table(
        &self,
        builder: &mut TableBuilder<'a>,
        state_index: StateIndex,
        item: &'a StateItem,
    ) -> Result<(), KikiErr> {
        if item.dot == 0 {
            return Ok(());
        }

        builder.set_action(state_index, Quasiterminal::Eof, item, Action::Accept)
    }

    fn add_original_item_action_to_table(
        &self,
        builder: &mut TableBuilder<'a>,
        state_index: StateIndex,
        item: &'a StateItem,
        rule_index: usize,
    ) -> Result<(), KikiErr> {
        let rule = &self.rules[rule_index];

        if item.dot == rule.fieldset.len() {
            self.add_original_reduction_to_table(builder, state_index, item, rule_index)
        } else if let IdentOrTerminalIdent::Terminal(terminal) =
            rule.fieldset.get_symbol_ident(item.dot)
        {
            self.add_original_shift_to_table(builder, state_index, item, &terminal.name)
        } else {
            Ok(())
        }
    }

    fn add_original_shift_to_table(
        &self,
        builder: &mut TableBuilder<'a>,
        state_index: StateIndex,
        item: &'a StateItem,
        terminal: &'a DollarlessTerminalName,
    ) -> Result<(), KikiErr> {
        let dest = self.machine.get_shift_dest(state_index, terminal).unwrap();
        builder.set_action(
            state_index,
            Quasiterminal::Terminal(terminal),
            item,
            Action::Shift(dest),
        )
    }

    fn add_original_reduction_to_table(
        &self,
        builder: &mut TableBuilder<'a>,
        state_index: StateIndex,
        item: &'a StateItem,
        rule_index: usize,
    ) -> Result<(), KikiErr> {
        builder.set_action(
            state_index,
            item.lookahead.as_quasiterminal(),
            item,
            Action::Reduce(rule_index),
        )
    }
}

impl<'a> TableBuilder<'a> {
    fn set_action(
        &mut self,
        state_index: StateIndex,
        quasiterminal: Quasiterminal<'a>,
        item: &'a StateItem,
        action: Action,
    ) -> Result<(), KikiErr> {
        if let Some((conflicting_item, _)) = self.actions.get(&(state_index, quasiterminal)) {
            return Err(KikiErr::TableConflict(Box::new(TableConflictErr {
                state_index,
                items: ((*conflicting_item).clone(), item.clone()),
                file: self.context.file.clone(),
                machine: self.context.machine.clone(),
            })));
        }

        self.actions
            .insert((state_index, quasiterminal), (item, action));
        Ok(())
    }
}

impl<'a> ImmutContext<'a> {
    fn add_gotos_to_table(&self, builder: &mut TableBuilder<'a>) {
        for transition in &self.machine.transitions {
            if let Symbol::Nonterminal(nonterminal) = &transition.symbol {
                builder.set_goto(transition.from, &nonterminal, Goto::State(transition.to));
            }
        }
    }
}

impl<'a> TableBuilder<'a> {
    fn set_goto(&mut self, state_index: StateIndex, nonterminal: &'a str, goto: Goto) {
        if let Some(old_goto) = self.gotos.get(&(state_index, nonterminal)) {
            let StateIndex(state_index) = state_index;
            panic!("Impossible: goto conflict. State index: {state_index}. Old goto: {old_goto:?}. New goto: {goto:?}");
        }

        self.gotos.insert((state_index, nonterminal), goto);
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::pipeline::validated_ast_to_machine::tests as ast_to_machine;

    use pretty_assertions::assert_eq;

    #[test]
    fn balanced_parens() {
        let file = ast_to_machine::balanced_parens_input();
        let machine = ast_to_machine::balanced_parens_expected_output();

        // TODO Delete
        println!("{:#?}", machine_to_table(&machine, &file));

        let actual = machine_to_table(&machine, &file).unwrap();
        let expected = balanced_parens_expected_output(&machine);
        assert_eq!(expected, actual);
    }

    fn balanced_parens_expected_output(machine: &Machine) -> Table {
        Table {
            start: machine.start,
            terminals: vec!["LParen", "RParen"]
                .into_iter()
                .map(DollarlessTerminalName::remove_dollars)
                .collect(),
            nonterminals: vec!["Expr"].into_iter().map(str::to_owned).collect(),
            actions: vec![],
            gotos: vec![],
        }
    }
}

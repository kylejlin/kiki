use crate::data::{
    ast::{Fieldset, NamedFieldset, TupleFieldset},
    machine::*,
    validated_file::*,
    KikiErr, Oset,
};
use std::collections::VecDeque;

/// Converts the AST to a finite state machine (FSM).
pub fn validated_ast_to_machine(file: &File) -> Result<Machine, KikiErr> {
    let builder = MachineBuilder::new(file);
    builder.build()
}

#[derive(Debug, Clone)]
struct MachineBuilder<'a> {
    context: ImmutContext<'a>,
    machine: Machine,
    queue: VecDeque<StateIndex>,
}

#[derive(Debug, Clone)]
struct ImmutContext<'a> {
    start_nonterminal: String,
    rules: Vec<Rule<'a>>,
}

impl MachineBuilder<'_> {
    fn new(file: &File) -> MachineBuilder {
        let context = ImmutContext {
            start_nonterminal: file.start.clone(),
            rules: file.get_rules().collect(),
        };
        let start_state = context.get_start_state();
        MachineBuilder {
            context,
            machine: Machine {
                states: vec![start_state],
                transitions: Oset::new(),
            },
            queue: VecDeque::from([StateIndex(0)]),
        }
    }
}

impl MachineBuilder<'_> {
    fn build(mut self) -> Result<Machine, KikiErr> {
        while let Some(state_index) = self.queue.pop_front() {
            self.enqueue_transition_targets(state_index);
        }
        Ok(self.machine)
    }

    fn enqueue_state_if_needed(&mut self, state: State) -> StateIndex {
        if let Some(index) = self.get_index_of_mergable(&state) {
            self.merge(index, state.items)
        } else {
            self.enqueue_new_state(state)
        }
    }

    fn get_index_of_mergable(&self, state: &State) -> Option<StateIndex> {
        self.machine
            .states
            .iter()
            .enumerate()
            .find_map(|(i, existing_state)| {
                if are_cores_equal(state, existing_state) {
                    Some(StateIndex(i))
                } else {
                    None
                }
            })
    }

    fn merge(&mut self, index: StateIndex, items: Oset<Item>) -> StateIndex {
        let were_items_added = self.add_items_if_needed(index, items);

        if were_items_added {
            self.queue.push_back(index);
        }

        return index;
    }

    /// Returns true if items were added.
    fn add_items_if_needed(&mut self, index: StateIndex, items: Oset<Item>) -> bool {
        let state = self.state_mut(index);
        let mut was_item_added = false;

        for item in items {
            if !state.items.contains(&item) {
                state.items.insert(item);
                was_item_added = true;
            }
        }

        was_item_added
    }

    fn enqueue_new_state(&mut self, state: State) -> StateIndex {
        let index = StateIndex(self.machine.states.len());
        self.machine.states.push(state);
        self.queue.push_back(index);
        index
    }

    fn enqueue_transition_targets(&mut self, state_index: StateIndex) {
        let next_symbols = self.get_symbols_right_of_dot(state_index);
        for symbol in &next_symbols {
            self.enqueue_transition_target(state_index, symbol);
        }
    }

    fn get_symbols_right_of_dot(&self, state_index: StateIndex) -> Oset<Symbol> {
        let state = self.state(state_index);
        state
            .items
            .iter()
            .filter_map(|item| self.get_symbol_right_of_dot(item))
            .collect()
    }

    fn get_symbol_right_of_dot(&self, item: &Item) -> Option<Symbol> {
        self.context.get_symbol_right_of_dot(item)
    }

    fn enqueue_transition_target(&mut self, state_index: StateIndex, symbol: &Symbol) {
        let target = self.get_transition_target(state_index, symbol);
        let target_index = self.enqueue_state_if_needed(target);
        let transition = Transition {
            from: state_index,
            to: target_index,
            symbol: symbol.clone(),
        };
        self.machine.transitions.insert(transition);
    }

    fn get_transition_target(&self, state_index: StateIndex, symbol: &Symbol) -> State {
        let items = self.get_transition_items(state_index, symbol);
        self.get_closure(&items)
    }

    fn get_closure(&self, items: &[Item]) -> State {
        self.context.get_closure(items)
    }

    fn get_transition_items(&self, state_index: StateIndex, symbol: &Symbol) -> Vec<Item> {
        let state = self.state(state_index);
        state
            .items
            .iter()
            .filter_map(|item| self.advance(item, symbol))
            .collect()
    }

    /// If `item` is `A -> alpha . B beta` and `symbol` is `B`,
    /// then this returns `Some(A -> alpha B . beta)`.
    fn advance(&self, item: &Item, symbol: &Symbol) -> Option<Item> {
        let right_of_dot = self.get_symbol_right_of_dot(item);
        if right_of_dot.as_ref() == Some(symbol) {
            Some(Item {
                rule_index: item.rule_index,
                lookahead: item.lookahead.clone(),
                dot: item.dot + 1,
            })
        } else {
            None
        }
    }
}

impl MachineBuilder<'_> {
    fn state(&self, index: StateIndex) -> &State {
        &self.machine.states[index.0]
    }

    fn state_mut(&mut self, index: StateIndex) -> &mut State {
        &mut self.machine.states[index.0]
    }
}

impl ImmutContext<'_> {
    fn get_start_state(&self) -> State {
        self.get_closure(&[Item {
            rule_index: RuleIndex::Augmented,
            lookahead: Lookahead::Eof,
            dot: 0,
        }])
    }

    fn get_closure(&self, items: &[Item]) -> State {
        let mut queue: VecDeque<Item> = items.iter().cloned().collect();
        let mut items = Oset::new();

        while let Some(next) = queue.pop_front() {
            if items.contains(&next) {
                continue;
            }

            self.enqueue_closure_implied_items(&mut queue, &next);
            items.insert(next);
        }

        State { items }
    }

    fn enqueue_closure_implied_items(&self, queue: &mut VecDeque<Item>, implicator: &Item) {
        for implied in self.get_closure_implied_items(implicator) {
            queue.push_back(implied);
        }
    }

    fn get_closure_implied_items(&self, item: &Item) -> Vec<Item> {
        match self.get_symbol_right_of_dot(item) {
            Some(Symbol::Nonterminal(name)) => {
                let lookaheads = self.get_first_after_dot(item);
                self.get_closure_implied_items_for_nonterminal(name, lookaheads)
            }
            Some(Symbol::Terminal(_)) | None => {
                vec![]
            }
        }
    }

    fn get_first_after_dot(&self, item: &Item) -> Oset<Lookahead> {
        todo!()
    }

    fn get_closure_implied_items_for_nonterminal(
        &self,
        nonterminal_name: String,
        lookaheads: Oset<Lookahead>,
    ) -> Vec<Item> {
        lookaheads
            .into_iter()
            .flat_map(|lookahead| {
                self.get_closure_implied_items_for_nonterminal_with_lookahead(
                    nonterminal_name.clone(),
                    lookahead,
                )
            })
            .collect()
    }

    fn get_closure_implied_items_for_nonterminal_with_lookahead(
        &self,
        nonterminal_name: String,
        lookahead: Lookahead,
    ) -> Vec<Item> {
        self.get_rule_indices_for_nonterminal(&nonterminal_name)
            .into_iter()
            .map(|rule_index| Item {
                rule_index: RuleIndex::Original(rule_index),
                lookahead: lookahead.clone(),
                dot: 0,
            })
            .collect()
    }

    fn get_rule_indices_for_nonterminal<'a>(
        &'a self,
        nonterminal_name: &'a str,
    ) -> impl Iterator<Item = usize> + 'a {
        self.rules
            .iter()
            .enumerate()
            .filter_map(move |(index, rule)| {
                if rule.constructor_name.type_name() == nonterminal_name {
                    Some(index)
                } else {
                    None
                }
            })
    }

    fn get_symbol_right_of_dot(&self, item: &Item) -> Option<Symbol> {
        match item.rule_index {
            RuleIndex::Original(rule_index) => {
                self.get_symbol_right_of_dot_for_original_rule(item.dot, rule_index)
            }
            RuleIndex::Augmented => self.get_symbol_right_of_dot_for_augmented_rule(item.dot),
        }
    }

    fn get_symbol_right_of_dot_for_augmented_rule(&self, dot: usize) -> Option<Symbol> {
        if dot == 0 {
            Some(Symbol::Nonterminal(self.start_nonterminal.clone()))
        } else {
            None
        }
    }

    fn get_symbol_right_of_dot_for_original_rule(
        &self,
        dot: usize,
        rule_index: usize,
    ) -> Option<Symbol> {
        let rule = &self.rules[rule_index];
        get_nth_field_symbol(dot, rule.fieldset)
    }
}

fn are_cores_equal(_a: &State, _b: &State) -> bool {
    todo!()
}

fn get_nth_field_symbol(n: usize, fieldset: &Fieldset) -> Option<Symbol> {
    match fieldset {
        Fieldset::Empty => None,
        Fieldset::Named(named) => get_nth_field_symbol_from_named(n, named),
        Fieldset::Tuple(tuple) => get_nth_field_symbol_from_tuple(n, tuple),
    }
}

fn get_nth_field_symbol_from_named(n: usize, named: &NamedFieldset) -> Option<Symbol> {
    named.fields.get(n).map(|field| field.symbol.clone().into())
}

fn get_nth_field_symbol_from_tuple(n: usize, tuple: &TupleFieldset) -> Option<Symbol> {
    tuple
        .fields
        .get(n)
        .map(|field| field.symbol().clone().into())
}

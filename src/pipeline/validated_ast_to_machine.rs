use crate::data::{machine::*, validated_file::*, KikiErr, Oset};
use std::collections::VecDeque;

/// Converts the AST to a finite state machine (FSM).
pub fn validated_ast_to_machine(file: &File) -> Result<Machine, KikiErr> {
    let builder = MachineBuilder::new(file);
    builder.build()
}

#[derive(Debug, Clone)]
struct MachineBuilder<'a> {
    file: &'a File,
    machine: Machine,
    queue: VecDeque<StateIndex>,
}

impl MachineBuilder<'_> {
    fn new(file: &File) -> MachineBuilder {
        MachineBuilder {
            file,
            machine: Machine {
                states: vec![get_start_state(file)],
                transitions: vec![],
            },
            queue: [StateIndex(0)].into_iter().collect(),
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

    fn get_closure(&self, items: &[Item]) -> State {
        get_closure(items, self.file)
    }

    fn enqueue_state(&mut self, state: State) -> StateIndex {
        if let Some(index) = self.get_index_of_mergable(&state) {
            self.merge(index, state.items);
            self.queue.push_back(index);
            return index;
        }

        self.enqueue_new_state(state)
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

    fn merge(&mut self, index: StateIndex, items: Oset<Item>) {
        todo!()
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
        todo!()
    }

    fn enqueue_transition_target(&mut self, state_index: StateIndex, symbol: &Symbol) {
        let target = self.get_transition_target(state_index, symbol);
        self.enqueue_state(target);
    }

    fn get_transition_target(&self, state_index: StateIndex, symbol: &Symbol) -> State {
        let items = self.get_transition_items(state_index, symbol);
        self.get_closure(&items)
    }

    fn get_transition_items(&self, state_index: StateIndex, symbol: &Symbol) -> Vec<Item> {
        let state = &self.machine.states[state_index.0];
        state
            .items
            .iter()
            .filter_map(|item| self.advance(item, symbol))
            .collect()
    }

    /// If `item` is `A -> alpha . B beta` and `symbol` is `B`,
    /// then this returns `Some(A -> alpha B . beta)`.
    fn advance(&self, item: &Item, symbol: &Symbol) -> Option<Item> {
        todo!()
    }
}

fn get_start_state(file: &File) -> State {
    get_closure(
        &[Item {
            rule: RuleIndex::Augmented,
            lookahead: Lookahead::Eof,
            dot: 0,
        }],
        file,
    )
}

fn get_closure(_items: &[Item], _file: &File) -> State {
    todo!()
}

fn are_cores_equal(_a: &State, _b: &State) -> bool {
    todo!()
}

use crate::data::{machine::*, validated_file::*, KikiErr};
use std::collections::VecDeque;

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
                states: vec![],
                transitions: vec![],
            },
            queue: VecDeque::new(),
        }
    }
}

impl MachineBuilder<'_> {
    fn build(mut self) -> Result<Machine, KikiErr> {
        let start_state = self.get_closure(&[Item {
            rule: RuleIndex::Augmented,
            lookahead: Lookahead::Eof,
            dot: 0,
        }]);
        let start_state_index = self.add_state(start_state);
        self.queue.push_back(start_state_index);
        while let Some(state_index) = self.queue.pop_front() {
            self.enqueue_transition_states(state_index);
        }
        Ok(self.machine)
    }

    fn get_closure(&self, _items: &[Item]) -> State {
        todo!()
    }

    fn add_state(&mut self, _state: State) -> StateIndex {
        todo!()
    }

    fn enqueue_transition_states(&mut self, _state_index: StateIndex) {
        todo!()
    }
}

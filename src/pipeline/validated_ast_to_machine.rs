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
            self.enqueue_transition_states(state_index);
        }
        Ok(self.machine)
    }

    fn get_closure(&self, items: &[Item]) -> State {
        get_closure(items, self.file)
    }

    fn add_state(&mut self, _state: State) -> StateIndex {
        todo!()
    }

    fn enqueue_transition_states(&mut self, _state_index: StateIndex) {
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

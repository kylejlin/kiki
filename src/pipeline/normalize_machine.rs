use crate::data::{
    machine::{Machine, StateIndex, Transition},
    unnormalized_machine::UnnormalizedMachine,
    IndexUpdater, Oset,
};

use crate::pipeline::sort_and_get_index_updater::sort_and_get_index_updater;

/// The first state must be the start state.
pub fn normalize_machine(unnormalized: UnnormalizedMachine) -> Machine {
    let states = unnormalized.states;
    let transitions: Oset<Transition> = unnormalized.transitions.into_iter().collect();
    let (states, updater) = sort_and_get_index_updater(states);
    let transitions = update_transitions(transitions, &updater);
    let start = StateIndex(updater.update(0));
    Machine {
        start,
        states: states.into_iter().collect(),
        transitions,
    }
}

fn update_transitions(transitions: Oset<Transition>, updater: &IndexUpdater) -> Oset<Transition> {
    transitions
        .into_iter()
        .map(|transition| update_transition(transition, updater))
        .collect()
}

fn update_transition(transition: Transition, updater: &IndexUpdater) -> Transition {
    Transition {
        from: StateIndex(updater.update(transition.from.0)),
        to: StateIndex(updater.update(transition.to.0)),
        symbol: transition.symbol,
    }
}

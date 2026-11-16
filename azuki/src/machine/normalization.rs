use super::*;

impl NonNormalizedMachine {
    pub fn normalize(self) -> Machine {
        let NonNormalizedMachine {
            states,
            transitions,
        } = self;

        let mut indexed_states: Vec<_> = states.into_iter().enumerate().collect();
        indexed_states.sort_unstable_by(|(_, a), (_, b)| a.cmp(b));

        let mut ids = vec![StateId { raw: usize::MAX }; indexed_states.len()];
        for (raw, (old_raw, _)) in indexed_states.iter().enumerate() {
            ids[*old_raw] = StateId { raw };
        }

        let start_id = ids[NonNormalizedStateId::START.raw];
        let states = indexed_states.into_iter().map(|(_, state)| state).collect();
        let transitions = transitions
            .into_iter()
            .map(
                |NonNormalizedTransition {
                     start_id,
                     dest_id,
                     symbol_id,
                 }| Transition {
                    start_id: ids[start_id.raw],
                    dest_id: ids[dest_id.raw],
                    symbol_id,
                },
            )
            .collect();

        Machine {
            start_id,
            states,
            transitions,
        }
    }
}

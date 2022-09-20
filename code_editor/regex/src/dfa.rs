use {makepad_sparse_set::SparseSet, std::collections::HashMap};

const UNKNOWN_STATE_PTR: StatePtr = 1 << 31;

#[derive(Clone, Debug)]
pub struct RunContext<'a> {
    state_cache: &'a mut HashMap<State, StatePtr>,
    states: &'a mut Vec<State>,
    next_state_count_per_state: &'a mut usize,
    next_states: &'a mut Vec<StatePtr>,
    current_states: &'a mut SparseSet,
    next_states: &'a mut SparseSet,
}

impl States {
    fn state(&self, state: StatePtr) -> &State {
        &self.state_ids[state as usize / self.next_state_count_per_state as usize]
    }

    fn next_state(&self, state: StatePtr, byte_class: u8) -> &StatePtr {
        &self.next_states[state as usize + byte_class as usize]
    }

    fn next_state_mut(&mut self, state: StatePtr, byte_class: u8) -> &mut StatePtr {
        &mut self.next_states[state as usize + byte_class as usize]
    }

    fn get_or_create_next_state(&mut self, current_state: StatePtr, byte: Option<u8>) {

    }

    fn get_or_add_state(&mut self, state: State) -> StatePtr {
        if let Some(state_ptr) = self.state_cache.get(state) {
            return state_ptr;
        }
        let state_ptr = self.add_state(state);
        self.state_cache.insert(state, state_ptr);
        state_ptr
    }

    fn add_state(&mut self, state: State) -> StatePtr {
        let state_ptr = self.states.len();
        self.states.push(state);
        self.next_states.extend(iter::repeat(UNKNOWN_STATE_PTR).take(self.next_state_count_per_state as usize));
        state_ptr
    }
}

#[derive(Clone, Debug)]
struct State {

}

type StatePtr = u32;
use {crate::program::{Instr, InstrPtr}, makepad_sparse_set::SparseSet};

#[derive(Clone, Debug)]
pub struct Nfa {
    current_threads: Threads,
    next_threads: Threads,
}

#[derive(Clone, Debug)]
struct Threads {
    instrs: SparseSet,
    slots: Slots,
}

impl Threads {
    fn add_thread(&mut self, instrs: &[Instr], instr_ptr: InstrPtr) {

    }
}

#[derive(Clone, Debug)]
struct Slots {
    slot_count_per_thread: usize,
    slots: Box<[Option<usize>]>,
}

impl Slots {
    fn new(thread_count: usize, slot_count_per_thread: usize) -> Self {
        Slots {
            slot_count_per_thread,
            slots: vec![None; thread_count * slot_count_per_thread].into_boxed_slice(),
        }
    }

    fn get(&self, instr: InstrPtr) -> &[Option<usize>] {
        &self.slots[instr * self.slot_count_per_thread..][..self.slot_count_per_thread]
    }

    fn get_mut(&mut self, instr: InstrPtr) -> &mut [Option<usize>] {
        &mut self.slots[instr * self.slot_count_per_thread..][..self.slot_count_per_thread]
    }
}
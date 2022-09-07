use {
    crate::{
        program::{Instr, InstrPtr, Pred},
        Cursor, Program, SparseSet,
    },
    std::{collections::HashMap, error, fmt, rc::Rc},
};

const MAX_STATE_PTR: StatePtr = (1 << 30) - 1;
const MATCHED_FLAG: StatePtr = 1 << 30;
const UNKNOWN_STATE_PTR: StatePtr = 1 << 31;
const DEAD_STATE_PTR: StatePtr = (1 << 31) + 1;
const ERROR_STATE_PTR: StatePtr = (1 << 31) + 2;

#[derive(Clone, Debug)]
pub struct Dfa {
    start_state_cache: Box<[StatePtr]>,
    state_cache: HashMap<StateId, StatePtr>,
    state_ids: Vec<StateId>,
    next_states: Vec<StatePtr>,
    current_threads: Threads,
    next_threads: Threads,
    stack: Vec<InstrPtr>,
}

impl Dfa {
    pub(crate) fn new() -> Self {
        Self {
            start_state_cache: vec![UNKNOWN_STATE_PTR; 1 << 5].into_boxed_slice(),
            state_cache: HashMap::new(),
            state_ids: Vec::new(),
            next_states: Vec::new(),
            current_threads: Threads::new(0),
            next_threads: Threads::new(0),
            stack: Vec::new(),
        }
    }

    pub(crate) fn run<C: Cursor>(
        &mut self,
        program: &Program,
        cursor: C,
        options: Options,
    ) -> Result<Option<usize>, RunError> {
        if !self.current_threads.instrs.capacity() != program.instrs.len() {
            self.current_threads = Threads::new(program.instrs.len());
            self.next_threads = Threads::new(program.instrs.len());
        }
        RunContext {
            start_state_cache: &mut self.start_state_cache,
            state_cache: &mut self.state_cache,
            state_ids: &mut self.state_ids,
            next_states: &mut self.next_states,
            current_threads: &mut self.current_threads,
            next_threads: &mut self.next_threads,
            stack: &mut self.stack,
            program,
            cursor,
            options,
        }
        .run()
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Options {
    pub(crate) stop_after_first_match: bool,
    pub(crate) continue_until_last_match: bool,
}

#[derive(Clone, Debug)]
pub struct RunError;

impl fmt::Display for RunError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl error::Error for RunError {}

struct RunContext<'a, C> {
    start_state_cache: &'a mut [StatePtr],
    state_cache: &'a mut HashMap<StateId, StatePtr>,
    state_ids: &'a mut Vec<StateId>,
    next_states: &'a mut Vec<StatePtr>,
    current_threads: &'a mut Threads,
    next_threads: &'a mut Threads,
    stack: &'a mut Vec<InstrPtr>,
    program: &'a Program,
    cursor: C,
    options: Options,
}

impl<'a, C: Cursor> RunContext<'a, C> {
    fn run(&mut self) -> Result<Option<usize>, RunError> {
        let mut matched = None;
        let mut current_state = UNKNOWN_STATE_PTR;
        let mut next_state = self.get_or_create_start_state();
        while !self.cursor.is_at_end_of_text() {
            while next_state <= MAX_STATE_PTR && !self.cursor.is_at_end_of_text() {
                current_state = next_state;
                let byte = self.cursor.next_byte().unwrap();
                let byte_class = self.program.byte_classes.get(byte);
                next_state = *self.next_state(current_state, byte_class);
            }
            if next_state == UNKNOWN_STATE_PTR {
                let byte = self.cursor.prev_byte().unwrap();
                self.cursor.next_byte().unwrap();
                let byte_class = self.program.byte_classes.get(byte);
                next_state = self.get_or_create_next_state(&mut current_state, Some(byte));
                *self.next_state_mut(current_state, byte_class) = next_state;
            }
            if next_state & MATCHED_FLAG != 0 {
                self.cursor.prev_byte().unwrap();
                matched = Some(self.cursor.byte_position());
                self.cursor.next_byte().unwrap();
                if self.options.stop_after_first_match {
                    return Ok(matched);
                }
                next_state &= !MATCHED_FLAG;
            } else if next_state == DEAD_STATE_PTR {
                return Ok(matched);
            } else if next_state == ERROR_STATE_PTR {
                return Err(RunError);
            }
        }
        next_state &= MAX_STATE_PTR;
        current_state = next_state;
        next_state = self.get_or_create_next_state(&mut current_state, None);
        if next_state & MATCHED_FLAG != 0 {
            matched = Some(self.cursor.byte_position());
        }
        Ok(matched)
    }

    fn get_or_create_start_state(&mut self) -> StatePtr {
        use crate::CharExt;

        let prev_byte_is_ascii_word = self
            .cursor
            .peek_prev_byte()
            .map_or(false, |b| (b as char).is_ascii_word());
        let next_byte_is_ascii_word = self
            .cursor
            .peek_next_byte()
            .map_or(false, |b| (b as char).is_ascii_word());
        let preds = Preds {
            is_at_start_of_text: self.cursor.is_at_start_of_text(),
            is_at_end_of_text: self.cursor.is_at_end_of_text(),
            is_at_ascii_word_boundary: prev_byte_is_ascii_word != next_byte_is_ascii_word,
        };
        let bits = preds.to_bits() as usize;
        match self.start_state_cache[bits] {
            UNKNOWN_STATE_PTR => {
                let mut flags = Flags::default();
                self.current_threads.add_thread(
                    self.program.start,
                    preds,
                    &mut flags,
                    &self.program.instrs,
                    &mut self.stack,
                );
                let state_id = StateId::new(flags, self.current_threads.instrs.as_slice());
                self.current_threads.instrs.clear();
                let state = self.get_or_create_state(state_id, None);
                self.start_state_cache[bits] = state;
                state
            }
            state => state,
        }
    }

    fn get_or_create_next_state(&mut self, state: &mut StatePtr, byte: Option<u8>) -> StatePtr {
        use {crate::CharExt, std::mem};

        let state_id = &self.state_ids[*state as usize];
        for instr in state_id.instrs() {
            self.current_threads.instrs.insert(instr);
        }
        let mut flags = Flags::default();
        let next_byte_is_ascii_word = byte.map_or(false, |byte| (byte as char).is_ascii_word());
        if next_byte_is_ascii_word {
            flags.set_prev_byte_is_ascii_word();
        }
        if state_id.flags.contains_assert_instr() {
            let prev_byte_is_ascii_word = self.state_ids[*state as usize]
                .flags
                .prev_byte_is_ascii_word();
            let preds = Preds {
                is_at_end_of_text: byte.is_none(),
                is_at_ascii_word_boundary: prev_byte_is_ascii_word != next_byte_is_ascii_word,
                ..Preds::default()
            };
            for &instr in &self.current_threads.instrs {
                self.next_threads.add_thread(
                    instr,
                    preds,
                    &mut flags,
                    &self.program.instrs,
                    &mut self.stack,
                );
            }
            mem::swap(&mut self.current_threads, &mut self.next_threads);
            self.next_threads.instrs.clear();
        }
        for &instr in self.current_threads.instrs.as_slice() {
            match self.program.instrs[instr] {
                Instr::Match => {
                    flags.set_matched();
                    if !self.options.continue_until_last_match {
                        break;
                    }
                }
                Instr::ByteRange(byte_range, next) => {
                    if byte.map_or(false, |byte| byte_range.contains(&byte)) {
                        self.next_threads.add_thread(
                            next,
                            Preds::default(),
                            &mut flags,
                            &self.program.instrs,
                            &mut self.stack,
                        );
                    }
                }
                _ => {}
            }
        }
        if !flags.matched() && self.next_threads.instrs.is_empty() {
            return DEAD_STATE_PTR;
        }
        let next_state_id = StateId::new(flags, self.next_threads.instrs.as_slice());
        self.current_threads.instrs.clear();
        self.next_threads.instrs.clear();
        let mut next_state = self.get_or_create_state(next_state_id, Some(state));
        if flags.matched() {
            next_state |= MATCHED_FLAG;
        }
        next_state
    }

    fn next_state(&self, state: StatePtr, byte_class: u8) -> &StatePtr {
        &self.next_states
            [state as usize * self.program.byte_classes.len() as usize + byte_class as usize]
    }

    fn next_state_mut(&mut self, state: StatePtr, byte_class: u8) -> &mut StatePtr {
        &mut self.next_states
            [state as usize * self.program.byte_classes.len() as usize + byte_class as usize]
    }

    fn get_or_create_state(
        &mut self,
        state_id: StateId,
        retained_state: Option<&mut StatePtr>,
    ) -> StatePtr {
        if let Some(&state) = self.state_cache.get(&state_id) {
            return state;
        }
        match retained_state {
            Some(retained_state) => {
                let retained_state_id = self.state_ids[*retained_state as usize].clone();
                self.clear_state_cache();
                *retained_state = self.create_state(retained_state_id);
            }
            None => self.clear_state_cache(),
        }
        self.create_state(state_id)
    }

    fn create_state(&mut self, state_id: StateId) -> StatePtr {
        use std::iter;

        let state_ptr = self.state_ids.len() as StatePtr;
        self.state_ids.push(state_id);
        self.next_states
            .extend(iter::repeat(UNKNOWN_STATE_PTR).take(self.program.byte_classes.len() as usize));
        state_ptr
    }

    fn clear_state_cache(&mut self) {
        for state in self.start_state_cache.iter_mut() {
            *state = UNKNOWN_STATE_PTR;
        }
        self.state_cache.clear();
        self.state_ids.clear();
    }
}

type StatePtr = u32;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct StateId {
    flags: Flags,
    bytes: Rc<[u8]>,
}

impl StateId {
    fn new(flags: Flags, instrs: &[InstrPtr]) -> Self {
        use makepad_varint::WriteVarint;

        let mut bytes = Vec::new();
        let mut prev_instr = 0;
        for &instr in instrs {
            let instr = instr as i32;
            bytes.write_vari32(instr - prev_instr).unwrap();
            prev_instr = instr;
        }
        Self {
            flags,
            bytes: Rc::from(bytes),
        }
    }

    fn instrs(&self) -> Instrs<'_> {
        Instrs {
            prev_instr: 0,
            bytes: &self.bytes,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Flags(u8);

impl Flags {
    fn matched(&self) -> bool {
        self.0 & 1 << 0 != 0
    }

    fn set_matched(&mut self) {
        self.0 |= 1 << 0
    }

    fn contains_assert_instr(&self) -> bool {
        self.0 & 1 << 1 != 0
    }

    fn set_contains_assert_instr(&mut self) {
        self.0 |= 1 << 1
    }

    fn prev_byte_is_ascii_word(&self) -> bool {
        self.0 & 1 << 2 != 0
    }

    fn set_prev_byte_is_ascii_word(&mut self) {
        self.0 |= 1 << 2;
    }
}

#[derive(Debug)]
struct Instrs<'a> {
    prev_instr: i32,
    bytes: &'a [u8],
}

impl<'a> Iterator for Instrs<'a> {
    type Item = InstrPtr;

    fn next(&mut self) -> Option<Self::Item> {
        use makepad_varint::ReadVarint;

        if self.bytes.is_empty() {
            return None;
        }
        let instr = self.prev_instr + (&mut self.bytes).read_vari32().unwrap();
        self.prev_instr = instr;
        Some(instr as InstrPtr)
    }
}

#[derive(Clone, Debug)]
struct Threads {
    instrs: SparseSet,
}

impl Threads {
    fn new(thread_count: usize) -> Self {
        Self {
            instrs: SparseSet::new(thread_count),
        }
    }

    fn add_thread(
        &mut self,
        instr: InstrPtr,
        preds: Preds,
        flags: &mut Flags,
        instrs: &[Instr],
        stack: &mut Vec<InstrPtr>,
    ) {
        stack.push(instr);
        while let Some(mut instr) = stack.pop() {
            loop {
                if !self.instrs.insert(instr) {
                    break;
                }
                match instrs[instr] {
                    Instr::Empty(next) | Instr::Save(_, next) => instr = next,
                    Instr::Assert(pred, next) => {
                        if match pred {
                            Pred::IsAtStartOfText => preds.is_at_start_of_text,
                            Pred::IsAtEndOfText => preds.is_at_end_of_text,
                            Pred::IsAtWordBoundary => preds.is_at_ascii_word_boundary,
                            Pred::IsNotAtWordBoundary => !preds.is_at_ascii_word_boundary,
                        } {
                            instr = next;
                        } else {
                            flags.set_contains_assert_instr();
                        }
                    }
                    Instr::Split(next_0, next_1) => {
                        stack.push(next_1);
                        instr = next_0;
                    }
                    _ => break,
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Preds {
    is_at_start_of_text: bool,
    is_at_end_of_text: bool,
    is_at_ascii_word_boundary: bool,
}

impl Preds {
    fn to_bits(self) -> u8 {
        let mut bits = 0;
        bits |= (self.is_at_start_of_text as u8) << 0;
        bits |= (self.is_at_end_of_text as u8) << 1;
        bits
    }
}

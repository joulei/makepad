use {
    crate::{
        program::{Instr, InstrPtr, Predicate},
        Cursor, Program,
    },
    makepad_sparse_set::SparseSet,
    std::{collections::HashMap, rc::Rc},
};

const MAX_STATE_PTR: StatePtr = (1 << 30) - 1;
const MATCHING_STATE_FLAG: StatePtr = 1 << 30;
const UNKNOWN_STATE_PTR: StatePtr = 1 << 31;
const DEAD_STATE_PTR: StatePtr = (1 << 31) + 1;
const ERROR_STATE_PTR: StatePtr = (1 << 31) + 2;

#[derive(Clone, Debug)]
pub(crate) struct Dfa {
    start_state_cache: Box<[StatePtr]>,
    state_cache: HashMap<State, StatePtr>,
    states: Vec<State>,
    transitions: Vec<StatePtr>,
    current_threads: SparseSet,
    next_threads: SparseSet,
    total_state_size: usize,
    add_thread_stack: Vec<InstrPtr>,
}

impl Dfa {
    pub(crate) fn new(program: &Program) -> Self {
        Self {
            start_state_cache: vec![UNKNOWN_STATE_PTR; 1 << 5].into_boxed_slice(),
            state_cache: HashMap::new(),
            states: Vec::new(),
            transitions: Vec::new(),
            current_threads: SparseSet::new(program.instrs.len()),
            next_threads: SparseSet::new(program.instrs.len()),
            total_state_size: 0,
            add_thread_stack: Vec::new(),
        }
    }

    pub(crate) fn run<C: Cursor>(
        &mut self,
        program: &Program,
        cursor: C,
        options: Options,
    ) -> Result<Option<usize>, RunError> {
        let last_clear = cursor.byte_position();
        RunContext {
            start_state_cache: &mut self.start_state_cache,
            state_cache: &mut self.state_cache,
            states: &mut self.states,
            transitions: &mut self.transitions,
            current_threads: &mut self.current_threads,
            next_threads: &mut self.next_threads,
            total_state_size: &mut self.total_state_size,
            add_thread_stack: &mut self.add_thread_stack,
            program,
            cursor,
            options,
            last_clear,
        }
        .run()
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Options {
    stop_after_first_match: bool,
    continue_after_leftmost_match: bool,
    max_size: usize,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct RunError;

impl Default for Options {
    fn default() -> Self {
        Self {
            stop_after_first_match: false,
            continue_after_leftmost_match: false,
            max_size: 1 << 20,
        }
    }
}

#[derive(Debug)]
struct RunContext<'a, C> {
    start_state_cache: &'a mut Box<[StatePtr]>,
    state_cache: &'a mut HashMap<State, StatePtr>,
    states: &'a mut Vec<State>,
    transitions: &'a mut Vec<StatePtr>,
    current_threads: &'a mut SparseSet,
    next_threads: &'a mut SparseSet,
    total_state_size: &'a mut usize,
    add_thread_stack: &'a mut Vec<InstrPtr>,
    program: &'a Program,
    cursor: C,
    options: Options,
    last_clear: usize,
}

impl<'a, C: Cursor> RunContext<'a, C> {
    fn run(&mut self) -> Result<Option<usize>, RunError> {
        let mut last_match = None;
        let mut current_state_ptr = UNKNOWN_STATE_PTR;
        let mut next_state_ptr = self.get_or_create_start_state()?;
        while !self.cursor.is_at_end_of_text() {
            while next_state_ptr <= MAX_STATE_PTR && !self.cursor.is_at_end_of_text() {
                current_state_ptr = next_state_ptr;
                let byte = self.cursor.next_byte().unwrap();
                next_state_ptr =
                    *self.transition(current_state_ptr, self.program.byte_classes[byte as usize]);
            }
            if next_state_ptr == UNKNOWN_STATE_PTR {
                let byte = self.cursor.peek_prev_byte().unwrap();
                next_state_ptr =
                    self.get_or_create_next_state(&mut current_state_ptr, Some(byte))?;
                *self.transition_mut(current_state_ptr, self.program.byte_classes[byte as usize]) =
                    next_state_ptr;
            }
            if next_state_ptr & MATCHING_STATE_FLAG != 0 {
                last_match = Some(self.cursor.byte_position() - 1);
                if self.options.stop_after_first_match {
                    return Ok(last_match);
                }
                next_state_ptr &= !MATCHING_STATE_FLAG;
            }
            if next_state_ptr == DEAD_STATE_PTR {
                return Ok(last_match);
            }
            if next_state_ptr == ERROR_STATE_PTR {
                return Err(RunError);
            }
        }
        current_state_ptr = next_state_ptr & MAX_STATE_PTR;
        next_state_ptr = self.get_or_create_next_state(&mut current_state_ptr, None)?;
        if next_state_ptr & MATCHING_STATE_FLAG != 0 {
            last_match = Some(self.cursor.byte_position());
        }
        Ok(last_match)
    }

    fn state(&self, state: StatePtr) -> &State {
        &self.states[state as usize / self.program.byte_class_count()]
    }

    fn transition(&self, state: StatePtr, byte_class: u8) -> &StatePtr {
        &self.transitions[state as usize + byte_class as usize]
    }

    fn size(&self) -> usize {
        use std::mem;

        self.start_state_cache.len() * mem::size_of::<StatePtr>()
            + self.state_cache.len() * (mem::size_of::<State>() + mem::size_of::<StatePtr>())
            + self.states.len() * mem::size_of::<State>()
            + self.transitions.len() * mem::size_of::<StatePtr>()
            + 2 * self.program.instrs.len() * mem::size_of::<usize>()
            + *self.total_state_size
    }

    fn transition_mut(&mut self, state: StatePtr, byte_class: u8) -> &mut StatePtr {
        &mut self.transitions[state as usize + byte_class as usize]
    }

    fn get_or_create_start_state(&mut self) -> Result<StatePtr, RunError> {
        use crate::CharExt;

        let prev_byte = self.cursor.peek_prev_byte();
        let prev_byte_is_cr = prev_byte.map_or(false, |byte| byte == 0x0D);
        let prev_byte_is_lf = prev_byte.map_or(false, |byte| byte == 0x0A);
        let prev_byte_is_word = prev_byte.map_or(false, |byte| (byte as char).is_ascii_word());
        let next_byte = self.cursor.peek_next_byte();
        let next_byte_is_cr = next_byte.map_or(false, |byte| byte == 0x0D);
        let next_byte_is_lf = next_byte.map_or(false, |byte| byte == 0x0A);
        let next_byte_is_word = next_byte.map_or(false, |byte| (byte as char).is_ascii_word());
        let predicates = Predicates {
            is_at_start_of_text: prev_byte.is_none(),
            is_at_end_of_text: next_byte.is_none(),
            is_at_start_of_line: prev_byte_is_lf || prev_byte_is_cr && !next_byte_is_lf,
            is_at_end_of_line: next_byte_is_lf && !prev_byte_is_cr || next_byte_is_cr,
            is_at_word_boundary: prev_byte_is_word != next_byte_is_word,
        };
        let start_state_index = predicates.to_bits() as usize;
        let start_state_ptr = self.start_state_cache[start_state_index];
        if start_state_ptr != UNKNOWN_STATE_PTR {
            return Ok(start_state_ptr);
        }
        let mut start_state_flags = StateFlags::default();
        add_thread(
            &mut self.current_threads,
            &self.program.instrs,
            self.program.start,
            predicates,
            &mut start_state_flags,
            &mut self.add_thread_stack,
        );
        let start_state = State::new(start_state_flags, self.current_threads.as_slice());
        self.current_threads.clear();
        let start_state_ptr = self.get_or_add_state(start_state, None)?;
        self.start_state_cache[start_state_index] = start_state_ptr;
        Ok(start_state_ptr)
    }

    fn get_or_create_next_state(
        &mut self,
        current_state_ptr: &mut StatePtr,
        byte: Option<u8>,
    ) -> Result<StatePtr, RunError> {
        use {crate::CharExt, std::mem};

        let current_state = self.state(*current_state_ptr).clone();
        for instr in current_state.instrs() {
            self.current_threads.insert(instr);
        }
        let mut next_state_flags = StateFlags::default();
        let next_byte_is_cr = byte.map_or(false, |byte| byte == 0x0D);
        let next_byte_is_lf = byte.map_or(false, |byte| byte == 0x0A);
        let next_byte_is_word = byte.map_or(false, |byte| (byte as char).is_ascii_word());
        let current_state_flags = current_state.flags();
        if current_state_flags.is_asserting() {
            let prev_byte_is_cr = current_state_flags.prev_byte_is_cr();
            let prev_byte_is_lf = current_state_flags.prev_byte_is_lf();
            let prev_byte_is_word = current_state_flags.prev_byte_is_word();
            let predicates = Predicates {
                is_at_end_of_text: byte.is_none(),
                is_at_start_of_line: prev_byte_is_lf || prev_byte_is_cr && !next_byte_is_lf,
                is_at_end_of_line: next_byte_is_lf && !prev_byte_is_cr || next_byte_is_cr,
                is_at_word_boundary: prev_byte_is_word != next_byte_is_word,
                ..Predicates::default()
            };
            for &instr in self.current_threads.iter() {
                add_thread(
                    self.next_threads,
                    &self.program.instrs,
                    instr,
                    predicates,
                    &mut next_state_flags,
                    &mut self.add_thread_stack,
                );
            }
            mem::swap(&mut self.current_threads, &mut self.next_threads);
            self.next_threads.clear();
        }
        if next_byte_is_cr {
            next_state_flags.set_prev_byte_is_cr();
        }
        if next_byte_is_lf {
            next_state_flags.set_prev_byte_is_lf();
        }
        if next_byte_is_word {
            next_state_flags.set_prev_byte_is_word();
        }
        for &instr in self.current_threads.iter() {
            match self.program.instrs[instr] {
                Instr::Match => {
                    next_state_flags.set_is_matching();
                    if !self.options.continue_after_leftmost_match {
                        break;
                    }
                }
                Instr::ByteRange(byte_range, to) => {
                    if byte.map_or(false, |byte| byte_range.contains(&byte)) {
                        add_thread(
                            &mut self.next_threads,
                            &self.program.instrs,
                            to,
                            Predicates::default(),
                            &mut next_state_flags,
                            self.add_thread_stack,
                        );
                    }
                }
                _ => {}
            }
        }
        if !next_state_flags.is_matching() && self.next_threads.is_empty() {
            return Ok(DEAD_STATE_PTR);
        }
        let next_state = State::new(current_state_flags, self.next_threads.as_slice());
        self.current_threads.clear();
        self.next_threads.clear();
        let mut next_state_ptr = self.get_or_add_state(next_state, Some(current_state_ptr))?;
        if next_state_flags.is_matching() {
            next_state_ptr |= MATCHING_STATE_FLAG;
        }
        Ok(next_state_ptr)
    }

    fn get_or_add_state(
        &mut self,
        state: State,
        retained_state_ptr: Option<&mut StatePtr>,
    ) -> Result<StatePtr, RunError> {
        if let Some(&state_ptr) = self.state_cache.get(&state) {
            return Ok(state_ptr);
        }
        if self.size() > self.options.max_size {
            match retained_state_ptr {
                Some(retained_state_ptr) => {
                    let retained_state = self.state(*retained_state_ptr).clone();
                    self.clear()?;
                    *retained_state_ptr = self.add_state(retained_state);
                }
                None => self.clear()?,
            }
        }
        let state_ptr = self.add_state(state.clone());
        let state_size = state.size();
        self.state_cache.insert(state, state_ptr);
        Ok(state_ptr)
    }

    fn add_state(&mut self, state: State) -> StatePtr {
        use std::iter;

        let state_ptr = self.states.len() as u32;
        let state_size = state.size();
        self.states.push(state);
        self.transitions
            .extend(iter::repeat(UNKNOWN_STATE_PTR).take(self.program.byte_class_count()));
        *self.total_state_size += state_size;
        state_ptr
    }

    fn clear(&mut self) -> Result<(), RunError> {
        if self.cursor.byte_position() - self.last_clear < 10 * self.states.len() {
            return Err(RunError);
        }
        for state_ptr in self.start_state_cache.iter_mut() {
            *state_ptr = UNKNOWN_STATE_PTR;
        }
        self.state_cache.clear();
        self.states.clear();
        self.transitions.clear();
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    bytes: Rc<[u8]>,
}

impl State {
    fn new(flags: StateFlags, instrs: &[InstrPtr]) -> Self {
        use makepad_varint::WriteVarint;

        let mut bytes = vec![flags.0];
        let mut prev_instr = 0;
        for &instr in instrs {
            let instr = instr as i32;
            bytes.write_vari32(instr - prev_instr).unwrap();
            prev_instr = instr;
        }
        Self {
            bytes: Rc::from(bytes),
        }
    }

    fn size(&self) -> usize {
        self.bytes.len()
    }

    fn flags(&self) -> StateFlags {
        StateFlags(self.bytes[0])
    }

    fn instrs(&self) -> Instrs<'_> {
        Instrs {
            bytes: &self.bytes[1..],
            prev_instr: 0,
        }
    }
}

type StatePtr = u32;

#[derive(Debug, Default)]
struct StateFlags(u8);

impl StateFlags {
    fn is_matching(&self) -> bool {
        self.0 & 1 << 0 != 0
    }

    fn is_asserting(&self) -> bool {
        self.0 & 1 << 1 != 0
    }

    fn prev_byte_is_lf(&self) -> bool {
        self.0 & 1 << 2 != 0
    }

    fn prev_byte_is_cr(&self) -> bool {
        self.0 & 1 << 3 != 0
    }

    fn prev_byte_is_word(&self) -> bool {
        self.0 & 1 << 4 != 0
    }

    fn set_is_matching(&mut self) {
        self.0 |= 1 << 0;
    }

    fn set_is_asserting(&mut self) {
        self.0 |= 1 << 1;
    }

    fn set_prev_byte_is_lf(&mut self) {
        self.0 |= 1 << 2;
    }

    fn set_prev_byte_is_cr(&mut self) {
        self.0 |= 1 << 3;
    }

    fn set_prev_byte_is_word(&mut self) {
        self.0 |= 1 << 4;
    }
}

#[derive(Debug)]
struct Instrs<'a> {
    bytes: &'a [u8],
    prev_instr: i32,
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

#[derive(Clone, Copy, Default)]
struct Predicates {
    is_at_start_of_text: bool,
    is_at_end_of_text: bool,
    is_at_start_of_line: bool,
    is_at_end_of_line: bool,
    is_at_word_boundary: bool,
}

impl Predicates {
    fn to_bits(self) -> u8 {
        let mut bits = 0;
        bits |= (self.is_at_start_of_text as u8) << 0;
        bits |= (self.is_at_end_of_text as u8) << 1;
        bits |= (self.is_at_start_of_line as u8) << 2;
        bits |= (self.is_at_end_of_line as u8) << 3;
        bits |= (self.is_at_word_boundary as u8) << 4;
        bits
    }
}

fn add_thread(
    threads: &mut SparseSet,
    instrs: &[Instr],
    instr_ptr: InstrPtr,
    predicates: Predicates,
    flags: &mut StateFlags,
    stack: &mut Vec<InstrPtr>,
) {
    stack.push(instr_ptr);
    while let Some(mut instr_ptr) = stack.pop() {
        if !threads.insert(instr_ptr) {
            break;
        }
        match instrs[instr_ptr] {
            Instr::Assert(predicate, to) => {
                if !match predicate {
                    Predicate::IsAtStartOfText => predicates.is_at_start_of_text,
                    Predicate::IsAtEndOfText => predicates.is_at_end_of_text,
                    Predicate::IsAtStartOfLine => predicates.is_at_start_of_line,
                    Predicate::IsAtEndOfLine => predicates.is_at_end_of_line,
                    Predicate::IsAtWordBoundary => predicates.is_at_word_boundary,
                    Predicate::IsNotAtWordBoundary => !predicates.is_at_word_boundary,
                } {
                    flags.set_is_asserting();
                    break;
                }
                instr_ptr = to;
            }
            Instr::Split(to_0, to_1) => {
                stack.push(to_1);
                instr_ptr = to_0;
            }
            _ => break,
        }
    }
}

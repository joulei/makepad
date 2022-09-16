#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct Prog {
    pub(crate) instrs: Vec<Instr>,
    pub(crate) start: usize,
}

pub(crate) type InstrPtr = usize;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) enum Instr {
    Match,
    Empty(InstrPtr),
    Char(char, InstrPtr),
    Split(InstrPtr, InstrPtr),
}

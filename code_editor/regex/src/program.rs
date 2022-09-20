use {crate::Range, std::ops::Deref};

#[derive(Clone, Debug)]
pub(crate) struct Program {
    instrs: Vec<Instr>,
}

impl Deref for Program {
    type Target = [Instr];

    fn deref(&self) -> &Self::Target {
        &*self.instrs
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Instr {
    ByteRange(Range<u8>, InstrPtr),
    Split(InstrPtr, InstrPtr),
}

pub(crate) type InstrPtr = usize;

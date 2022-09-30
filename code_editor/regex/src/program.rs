use {crate::Range, std::ops::Deref};

#[derive(Clone, Debug)]
pub(crate) struct Program {
    pub(crate) instrs: Vec<Instr>,
    pub(crate) start: InstrPtr,
    pub(crate) byte_classes: Box<[u8]>,
}

impl Program {
    pub(crate) fn byte_class_count(&self) -> usize {
        self.byte_classes[255] as usize + 1
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Instr {
    Match,
    ByteRange(Range<u8>, InstrPtr),
    Assert(Predicate, InstrPtr),
    Split(InstrPtr, InstrPtr),
}

pub(crate) type InstrPtr = usize;

#[derive(Clone, Copy, Debug)]
pub(crate) enum Predicate {
    IsAtStartOfText,
    IsAtEndOfText,
    IsAtStartOfLine,
    IsAtEndOfLine,
    IsAtWordBoundary,
    IsNotAtWordBoundary,
}

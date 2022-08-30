use crate::{CharClass, Range};

#[derive(Clone, Debug, Default)]
pub(crate) struct CaseFolder {}

impl CaseFolder {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn fold(&mut self, char_range: Range<char>, output: &mut CharClass) {
        unimplemented!()
    }
}

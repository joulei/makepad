use super::{Graphemes, StrExt};

#[derive(Clone, Debug)]
pub struct GraphemeIndices<'a> {
    graphemes: Graphemes<'a>,
    start_offset: usize,
}

impl<'a> GraphemeIndices<'a> {
    pub(super) fn new(string: &'a str) -> Self {
        Self {
            graphemes: string.graphemes(),
            start_offset: string.as_ptr() as usize,
        }
    }
}

impl<'a> Iterator for GraphemeIndices<'a> {
    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let grapheme = self.graphemes.next()?;
        Some((grapheme.as_ptr() as usize - self.start_offset, grapheme))
    }
}

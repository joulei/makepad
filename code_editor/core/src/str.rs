pub trait StrExt {
    fn is_grapheme_boundary(&self, index: usize) -> bool;
    fn next_grapheme_boundary(&self, index: usize) -> Option<usize>;
    fn prev_grapheme_boundary(&self, index: usize) -> Option<usize>;
    fn graphemes(&self) -> Graphemes<'_>;
    fn grapheme_indices(&self) -> GraphemeIndices<'_>;
}

impl StrExt for str {
    fn is_grapheme_boundary(&self, index: usize) -> bool {
        self.is_char_boundary(index)
    }

    fn next_grapheme_boundary(&self, mut index: usize) -> Option<usize> {
        if index == self.len() {
            return None;
        }
        index += 1;
        while !self.is_grapheme_boundary(index) {
            index += 1;
        }
        Some(index)
    }

    fn prev_grapheme_boundary(&self, mut index: usize) -> Option<usize> {
        if index == 0 {
            return None;
        }
        index -= 1;
        while !self.is_grapheme_boundary(index) {
            index -= 1;
        }
        Some(index)
    }

    fn graphemes(&self) -> Graphemes<'_> {
        Graphemes::new(self)
    }

    fn grapheme_indices(&self) -> GraphemeIndices<'_> {
        GraphemeIndices::new(self)
    }
}

#[derive(Clone, Debug)]
pub struct Graphemes<'a> {
    string: &'a str,
}

impl<'a> Graphemes<'a> {
    pub(super) fn new(string: &'a str) -> Self {
        Self { string }
    }
}

impl<'a> Iterator for Graphemes<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.string.next_grapheme_boundary(0)?;
        let (string_0, string_1) = self.string.split_at(index);
        self.string = string_1;
        Some(string_0)
    }
}


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

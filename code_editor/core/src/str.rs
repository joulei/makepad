mod grapheme_indices;
mod graphemes;

pub use self::{grapheme_indices::GraphemeIndices, graphemes::Graphemes};

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

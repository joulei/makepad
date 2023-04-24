use super::StrExt;

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

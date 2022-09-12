use {super::Cursor, crate::cursor::WordCursor};

#[derive(Clone, Debug)]
pub struct Words<'a> {
    string: &'a str,
    cursor_front: WordCursor<Cursor<'a>>,
    cursor_back: WordCursor<Cursor<'a>>,
}

impl<'a> Words<'a> {
    pub(super) fn new(string: &'a str) -> Self {
        use crate::Cursor as _;

        Self {
            string,
            cursor_front: Cursor::front(string).into_word_cursor(),
            cursor_back: Cursor::back(string).into_word_cursor(),
        }
    }
}

impl<'a> Iterator for Words<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor_front.byte_position() == self.cursor_back.byte_position() {
            return None;
        }
        let start = self.cursor_front.byte_position();
        self.cursor_front.move_next_word();
        Some(&self.string[start..self.cursor_front.byte_position()])
    }
}

impl<'a> DoubleEndedIterator for Words<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.cursor_front.byte_position() == self.cursor_back.byte_position() {
            return None;
        }
        let end = self.cursor_back.byte_position();
        self.cursor_back.move_prev_word();
        Some(&self.string[self.cursor_back.byte_position()..end])
    }
}

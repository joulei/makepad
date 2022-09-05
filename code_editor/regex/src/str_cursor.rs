use crate::cursor::Cursor;

#[derive(Clone, Copy, Debug)]
pub(crate) struct StrCursor<'a> {
    string: &'a str,
    byte_position: usize,
}

impl<'a> StrCursor<'a> {
    pub(crate) fn new(string: &'a str) -> Self {
        Self {
            string,
            byte_position: 0,
        }
    }
}

impl<'a> Cursor for StrCursor<'a> {
    fn is_at_start_of_text(&self) -> bool {
        self.byte_position == 0
    }

    fn is_at_end_of_text(&self) -> bool {
        self.byte_position == self.string.len()
    }

    fn is_at_word_boundary(&self) -> bool {
        use crate::CharExt;

        let prev_ch = self.string[..self.byte_position].chars().next_back();
        let next_ch = self.string[self.byte_position..].chars().next();
        prev_ch.map_or(false, |ch| ch.is_word()) != next_ch.map_or(false, |ch| ch.is_word())
    }

    fn byte_position(&self) -> usize {
        self.byte_position
    }

    fn peek_next_byte(&self) -> Option<u8> {
        self.string.as_bytes()[self.byte_position..]
            .first()
            .cloned()
    }

    fn peek_prev_byte(&self) -> Option<u8> {
        self.string.as_bytes()[..self.byte_position()]
            .last()
            .cloned()
    }

    fn move_to(&mut self, position: usize) {
        assert!(position <= self.string.len());
        self.byte_position = position;
    }

    fn next_byte(&mut self) -> Option<u8> {
        if self.byte_position == self.string.len() {
            return None;
        }
        let byte = self.string.as_bytes()[self.byte_position];
        self.byte_position += 1;
        Some(byte)
    }

    fn prev_byte(&mut self) -> Option<u8> {
        if self.byte_position == 0 {
            return None;
        }
        self.byte_position -= 1;
        Some(self.string.as_bytes()[self.byte_position])
    }

    fn next_char(&mut self) -> Option<char> {
        if self.byte_position == self.string.len() {
            return None;
        }
        let ch = self.string[self.byte_position..].chars().next().unwrap();
        self.byte_position += ch.len_utf8();
        Some(ch)
    }

    fn prev_char(&mut self) -> Option<char> {
        if self.byte_position == 0 {
            return None;
        }
        let ch = self.string[..self.byte_position]
            .chars()
            .next_back()
            .unwrap();
        self.byte_position -= ch.len_utf8();
        Some(ch)
    }
}

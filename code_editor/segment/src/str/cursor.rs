#[derive(Clone, Debug)]
pub(super) struct Cursor<'a> {
    string: &'a str,
    byte_position: usize,
}

impl<'a> Cursor<'a> {
    pub(super) fn front(string: &'a str) -> Self {
        Self {
            string,
            byte_position: 0,
        }
    }

    pub(super) fn back(string: &'a str) -> Self {
        Self {
            string,
            byte_position: string.len(),
        }
    }
}

impl<'a> crate::Cursor for Cursor<'a> {
    fn is_at_start(&self) -> bool {
        self.byte_position == 0
    }

    fn is_at_end(&self) -> bool {
        self.byte_position == self.string.len()
    }

    fn is_at_char_boundary(&self) -> bool {
        self.string.is_char_boundary(self.byte_position)
    }

    fn byte_position(&self) -> usize {
        self.byte_position
    }

    fn current_char(&self) -> Option<char> {
        self.string[self.byte_position..].chars().next()
    }

    fn move_to(&mut self, byte_position: usize) {
        assert!(byte_position <= self.string.len());
        self.byte_position = byte_position;
    }

    fn move_next_char(&mut self) {
        self.byte_position += super::utf8_char_width(self.string.as_bytes()[self.byte_position]);
    }

    fn move_prev_char(&mut self) {
        loop {
            self.byte_position -= 1;
            if self.string.as_bytes()[self.byte_position] & 0xC0 != 0x80 {
                break;
            }
        }
    }
}

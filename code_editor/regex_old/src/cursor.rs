pub trait Cursor {
    fn byte_position(&self) -> usize;
    fn peek_next_byte(&self) -> Option<u8>;
    fn peek_prev_byte(&self) -> Option<u8>;
    fn peek_next_char(&self) -> Option<char>;
    fn peek_prev_char(&self) -> Option<char>;
    fn move_to(&mut self, byte_position: usize);
    fn next_byte(&mut self) -> Option<u8>;
    fn prev_byte(&mut self) -> Option<u8>;
    fn next_char(&mut self) -> Option<char>;
    fn prev_char(&mut self) -> Option<char>;

    fn is_at_start_of_text(&self) -> bool {
        self.peek_prev_char().is_none()
    }

    fn is_at_end_of_text(&self) -> bool {
        self.peek_next_char().is_none()
    }

    fn is_at_start_of_line(&self) -> bool {
        unimplemented!()
    }

    fn is_at_end_of_line(&self) -> bool {
        unimplemented!()
    }

    fn is_at_word_boundary(&self) -> bool {
        use crate::CharExt;

        let prev_char_is_word = self.peek_prev_char().map_or(false, |ch| ch.is_word());
        let next_char_is_word = self.peek_next_char().map_or(false, |ch| ch.is_word());
        prev_char_is_word != next_char_is_word
    }

    fn rev(self) -> Rev<Self>
    where
        Self: Sized,
    {
        Rev { cursor: self }
    }
}

impl<'a, T: Cursor> Cursor for &'a mut T {
    fn byte_position(&self) -> usize {
        (**self).byte_position()
    }

    fn peek_next_byte(&self) -> Option<u8> {
        (**self).peek_next_byte()
    }

    fn peek_prev_byte(&self) -> Option<u8> {
        (**self).peek_prev_byte()
    }

    fn peek_next_char(&self) -> Option<char> {
        (**self).peek_next_char()
    }

    fn peek_prev_char(&self) -> Option<char> {
        (**self).peek_prev_char()
    }

    fn move_to(&mut self, position: usize) {
        (**self).move_to(position)
    }

    fn next_byte(&mut self) -> Option<u8> {
        (**self).next_byte()
    }

    fn prev_byte(&mut self) -> Option<u8> {
        (**self).prev_byte()
    }

    fn next_char(&mut self) -> Option<char> {
        (**self).next_char()
    }

    fn prev_char(&mut self) -> Option<char> {
        (**self).prev_char()
    }
}

#[derive(Clone, Debug)]
pub struct Rev<C> {
    cursor: C,
}

impl<C: Cursor> Cursor for Rev<C> {
    fn byte_position(&self) -> usize {
        self.cursor.byte_position()
    }

    fn peek_next_byte(&self) -> Option<u8> {
        self.cursor.peek_prev_byte()
    }

    fn peek_prev_byte(&self) -> Option<u8> {
        self.cursor.peek_next_byte()
    }

    fn peek_next_char(&self) -> Option<char> {
        self.cursor.peek_prev_char()
    }

    fn peek_prev_char(&self) -> Option<char> {
        self.cursor.peek_next_char()
    }

    fn move_to(&mut self, byte_position: usize) {
        self.cursor.move_to(byte_position)
    }

    fn next_byte(&mut self) -> Option<u8> {
        self.cursor.prev_byte()
    }

    fn prev_byte(&mut self) -> Option<u8> {
        self.cursor.next_byte()
    }

    fn next_char(&mut self) -> Option<char> {
        self.cursor.prev_char()
    }

    fn prev_char(&mut self) -> Option<char> {
        self.cursor.next_char()
    }
}

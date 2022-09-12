mod word_cursor;

pub use self::word_cursor::WordCursor;

pub trait Cursor: Clone {
    fn is_at_start(&self) -> bool;
    fn is_at_end(&self) -> bool;
    fn is_at_char_boundary(&self) -> bool;
    fn byte_position(&self) -> usize;
    fn current_char(&self) -> Option<char>;
    fn move_to(&mut self, byte_position: usize);
    fn move_next_char(&mut self);
    fn move_prev_char(&mut self);

    fn into_word_cursor(self) -> WordCursor<Self>
    where
        Self: Sized,
    {
        WordCursor::new(self)
    }
}

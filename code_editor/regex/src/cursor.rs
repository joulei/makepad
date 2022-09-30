pub trait Cursor {
    fn is_at_start_of_text(&self) -> bool;
    fn is_at_end_of_text(&self) -> bool;
    fn byte_position(&self) -> usize;
    fn peek_next_byte(&self) -> Option<u8>;
    fn peek_prev_byte(&self) -> Option<u8>;
    fn next_byte(&mut self) -> Option<u8>;
    fn prev_byte(&mut self) -> Option<u8>;
}

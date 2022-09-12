mod cursor;
mod words;

pub use self::words::Words;

use self::cursor::Cursor;

pub trait StrExt {
    fn words(&self) -> Words;
}

impl StrExt for str {
    fn words(&self) -> Words {
        Words::new(self)
    }
}

fn utf8_char_width(byte: u8) -> usize {
    match byte {
        byte if byte < 0x80 => 1,
        byte if byte < 0xE0 => 2,
        byte if byte < 0xF0 => 3,
        _ => 4,
    }
}

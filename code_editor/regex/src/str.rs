pub(crate) fn utf8_char_width(byte: u8) -> usize {
    match byte {
        byte if byte < 0x80 => 1,
        byte if byte < 0xE0 => 2,
        byte if byte < 0xF0 => 3,
        _ => 4,
    }
}
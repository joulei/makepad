use {crate::Cursor, makepad_ucd::WordBreak};

#[derive(Clone, Debug)]
pub struct WordCursor<C> {
    cursor: C,
    front_cursor: C,
    back_cursor: C,
    prev_word_break: Option<WordBreak>,
    prev_extended_pictographic: bool,
    next_word_break: Option<WordBreak>,
    next_extended_pictographic: bool,
    prev_prev_word_break_skip: Option<WordBreak>,
    prev_word_break_skip: Option<WordBreak>,
    next_word_break_skip: Option<WordBreak>,
    next_next_word_break_skip: Option<WordBreak>,
    regional_indicator_count: usize,
}

impl<C: Cursor> WordCursor<C> {
    pub(super) fn new(cursor: C) -> Self {
        let front_cursor = cursor.clone();
        let back_cursor = cursor.clone();
        let mut cursor = Self {
            cursor,
            front_cursor,
            back_cursor,
            prev_word_break: None,
            prev_extended_pictographic: false,
            next_word_break: None,
            next_extended_pictographic: false,
            prev_prev_word_break_skip: None,
            prev_word_break_skip: None,
            next_word_break_skip: None,
            next_next_word_break_skip: None,
            regional_indicator_count: 0,
        };
        cursor.update_cached_values();
        cursor
    }

    pub fn is_at_start(&self) -> bool {
        self.cursor.is_at_start()
    }

    pub fn is_at_end(&self) -> bool {
        self.cursor.is_at_end()
    }

    pub fn is_at_word_boundary(&mut self) -> bool {
        use makepad_ucd::WordBreak::*;

        // Break at the start and end of text, unless the text is empty.
        if self.is_at_start() {
            // WB1
            return true;
        }
        if self.is_at_end() {
            // WB2
            return true;
        }
        match (self.prev_word_break.unwrap(), self.next_word_break.unwrap()) {
            // Do not break within CRLF.
            (CR, LF) => return false, // WB3

            // Otherwise break before and after Newlines (including CR and LF)
            (Newline | CR | LF, _) => return true, // WB3a
            (_, Newline | CR | LF) => return true, // WB3b

            // Do not break within emoji zwj sequences.
            (ZWJ, _) if self.next_extended_pictographic => return false, // WB3c

            // Keep horizontal whitespace together.
            (WSegSpace, WSegSpace) => return false, // WB3d

            // Ignore Format and Extend characters, except after sot, CR, LF, and Newline. This also
            // has the effect of: Any × (Format | Extend | ZWJ).
            (_, Format | Extend | ZWJ) => return false, // WB4

            _ => {}
        };
        match (
            self.prev_prev_word_break_skip,
            self.prev_word_break_skip,
            self.next_word_break_skip,
            self.next_next_word_break_skip,
        ) {
            // Do not break between most letters.
            (_, Some(ALetter | HebrewLetter), Some(ALetter | HebrewLetter), _) => {
                // WB5
                false
            }

            // Do not break letters across certain punctuation.
            (
                _,
                Some(ALetter | HebrewLetter),
                Some(MidLetter | MidNumLet | SingleQuote),
                Some(ALetter | HebrewLetter),
            ) => false, // WB6
            (
                Some(ALetter | HebrewLetter),
                Some(MidLetter | MidNumLet | SingleQuote),
                Some(ALetter | HebrewLetter),
                _,
            ) => false, // WB7
            (_, Some(HebrewLetter), Some(SingleQuote), _) => false, // WB7a
            (_, Some(HebrewLetter), Some(DoubleQuote), Some(HebrewLetter)) => false, // WB7b
            (Some(HebrewLetter), Some(DoubleQuote), Some(HebrewLetter), _) => false, // WB7c

            // Do not break within sequences of digits, or digits adjacent to letters (“3a”, or
            // “A3”).
            (_, Some(Numeric), Some(Numeric), _) => false, // WB8
            (_, Some(ALetter | HebrewLetter), Some(Numeric), _) => false, // WB9
            (_, Some(Numeric), Some(ALetter | HebrewLetter), _) => false, // WB10

            // Do not break within sequences, such as “3.2” or “3,456.789”.
            (Some(Numeric), Some(MidNum | MidNumLet | SingleQuote), Some(Numeric), _) => false, // WB11
            (_, Some(Numeric), Some(MidNum | MidNumLet | SingleQuote), Some(Numeric)) => false, // WB12

            // Do not break between Katakana.
            (_, Some(Katakana), Some(Katakana), _) => false, // WB13

            // Do not break from extenders.
            (
                _,
                Some(ALetter | HebrewLetter | Numeric | Katakana | ExtendNumLet),
                Some(ExtendNumLet),
                _,
            ) => false, // WB13A
            (_, Some(ExtendNumLet), Some(ALetter | HebrewLetter | Numeric | Katakana), _) => {
                // WB13b
                false
            }

            // Do not break within emoji flag sequences. That is, do not break between regional
            // indicator (RI) symbols if there is an odd number of RI characters before the break
            // point.
            (_, Some(RegionalIndicator), Some(RegionalIndicator), _) => {
                // WB15 + WB16
                self.regional_indicator_count % 2 == 0
            }

            // Otherwise, break everywhere (including around ideographs).
            _ => true, // WB999
        }
    }

    pub fn byte_position(&self) -> usize {
        self.cursor.byte_position()
    }

    pub fn move_to(&mut self, byte_position: usize) {
        self.cursor.move_to(byte_position);
        self.front_cursor.move_to(byte_position);
        self.back_cursor.move_to(byte_position);
        self.update_cached_values()
    }

    pub fn move_next_word(&mut self) {
        use makepad_ucd::{
            Ucd,
            WordBreak::{Extend, Format, RegionalIndicator, ZWJ},
        };

        loop {
            self.cursor.move_next_char();
            self.prev_word_break = self.next_word_break;
            self.prev_extended_pictographic = self.next_extended_pictographic;
            match self.cursor.current_char() {
                Some(ch) => {
                    self.next_word_break = Some(ch.word_break());
                    self.next_extended_pictographic = ch.extended_pictographic();
                }
                None => {
                    self.next_word_break = None;
                    self.next_extended_pictographic = false;
                }
            };
            match self.prev_word_break.unwrap() {
                Extend | Format | ZWJ => {}
                _ => {
                    if self.prev_word_break_skip.is_some() {
                        if self.prev_prev_word_break_skip.is_some() {
                            self.front_cursor.move_next_char();
                        }
                        self.prev_prev_word_break_skip =
                            self.front_cursor.move_next_word_break_skip();
                    }
                    self.prev_word_break_skip = self.prev_word_break;
                    self.next_word_break_skip = self.next_next_word_break_skip;
                    self.next_next_word_break_skip = if self.back_cursor.is_at_end() {
                        None
                    } else {
                        self.back_cursor.move_next_char();
                        self.back_cursor.move_next_word_break_skip()
                    };
                    self.regional_indicator_count = match self.prev_word_break {
                        Some(RegionalIndicator) => self.regional_indicator_count + 1,
                        _ => 0,
                    };
                }
            }
            if self.is_at_word_boundary() {
                break;
            }
        }
    }

    pub fn move_prev_word(&mut self) {
        use makepad_ucd::{
            Ucd,
            WordBreak::{Extend, Format, RegionalIndicator, ZWJ},
        };

        loop {
            self.cursor.move_prev_char();
            self.next_word_break = self.prev_word_break;
            self.next_extended_pictographic = self.prev_extended_pictographic;
            if self.cursor.is_at_start() {
                self.prev_word_break = None;
                self.prev_extended_pictographic = false;
            } else {
                self.cursor.move_prev_char();
                let ch = self.cursor.current_char().unwrap();
                self.prev_word_break = Some(ch.word_break());
                self.prev_extended_pictographic = ch.extended_pictographic();
                self.cursor.move_next_char();
            };
            match self.next_word_break.unwrap() {
                Extend | Format | ZWJ => {}
                _ => {
                    if self.next_word_break_skip.is_some() {
                        self.next_next_word_break_skip =
                            self.back_cursor.move_prev_word_break_skip();
                    };
                    self.next_word_break_skip = self.next_word_break;
                    self.prev_word_break_skip = self.prev_prev_word_break_skip;
                    self.prev_prev_word_break_skip = self.front_cursor.move_prev_word_break_skip();
                    self.regional_indicator_count = match self.next_word_break {
                        Some(RegionalIndicator) => self.regional_indicator_count - 1,
                        _ => match self.prev_word_break_skip {
                            Some(RegionalIndicator) => self.cursor.regional_indicator_count(),
                            _ => 0,
                        },
                    };
                }
            }
            if self.is_at_word_boundary() {
                break;
            }
        }
    }

    fn update_cached_values(&mut self) {
        use makepad_ucd::Ucd;

        if self.cursor.is_at_start() {
            self.prev_word_break = None;
            self.prev_extended_pictographic = false;
        } else {
            self.cursor.move_prev_char();
            let ch = self.cursor.current_char().unwrap();
            self.prev_word_break = Some(ch.word_break());
            self.prev_extended_pictographic = ch.extended_pictographic();
            self.cursor.move_next_char();
        };
        match self.cursor.current_char() {
            Some(ch) => {
                self.next_word_break = Some(ch.word_break());
                self.next_extended_pictographic = ch.extended_pictographic();
            }
            None => {
                self.next_word_break = None;
                self.next_extended_pictographic = false;
            }
        };
        self.prev_word_break_skip = self.front_cursor.move_prev_word_break_skip();
        self.prev_prev_word_break_skip = self.front_cursor.move_prev_word_break_skip();
        self.next_word_break_skip = self.back_cursor.move_next_word_break_skip();
        self.next_next_word_break_skip = if self.back_cursor.is_at_end() {
            None
        } else {
            self.back_cursor.move_next_char();
            self.back_cursor.move_next_word_break_skip()
        };
        self.regional_indicator_count = self.cursor.regional_indicator_count();
    }
}

trait CursorExt: Cursor {
    fn move_next_word_break_skip(&mut self) -> Option<WordBreak> {
        use makepad_ucd::{
            Ucd,
            WordBreak::{Extend, Format, ZWJ},
        };

        loop {
            match self.current_char().map(|ch| ch.word_break()) {
                Some(Extend | Format | ZWJ) => {}
                word_break => break word_break,
            }
            self.move_next_char();
        }
    }

    fn move_prev_word_break_skip(&mut self) -> Option<WordBreak> {
        use makepad_ucd::{
            Ucd,
            WordBreak::{Extend, Format, ZWJ},
        };

        loop {
            if self.is_at_start() {
                break None;
            }
            self.move_prev_char();
            match self.current_char().unwrap().word_break() {
                Extend | Format | ZWJ => {}
                word_break => break Some(word_break),
            }
        }
    }

    fn regional_indicator_count(&mut self) -> usize {
        let mut regional_indicator_count = 0;
        let byte_position = self.byte_position();
        loop {
            match self.move_prev_word_break_skip() {
                Some(WordBreak::RegionalIndicator) => {}
                _ => break,
            }
            regional_indicator_count += 1;
        }
        self.move_to(byte_position);
        regional_indicator_count
    }
}

impl<T: Cursor> CursorExt for T {}

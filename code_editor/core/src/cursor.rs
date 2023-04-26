use crate::{text, text::DeltaDesc};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Cursor {
    pub caret: Position,
    pub anchor: Position,
}

impl Cursor {
    pub fn is_empty(self) -> bool {
        self.caret == self.anchor
    }

    pub fn is_at_first_line(self) -> bool {
        self.caret.position.line_index == 0
    }

    pub fn is_at_last_line(self, context: &Context) -> bool {
        self.caret.position.line_index == context.lines.len() - 1
    }

    pub fn is_at_start_of_line(self) -> bool {
        self.caret.position.byte_index == 0
    }

    pub fn is_at_end_of_line(self, context: &Context) -> bool {
        let line = &context.lines[self.caret.position.line_index];
        self.caret.position.byte_index == line.len()
    }

    pub fn start(self) -> Position {
        self.caret.min(self.anchor)
    }

    pub fn end(self) -> Position {
        self.caret.max(self.anchor)
    }

    pub fn move_left(self, context: &Context) -> Self {
        if !self.is_at_start_of_line() {
            self.move_to_prev_grapheme_boundary(context)
        } else if !self.is_at_first_line() {
            self.move_to_end_of_prev_line(context)
        } else {
            self
        }
    }

    pub fn move_right(self, context: &Context) -> Self {
        if !self.is_at_end_of_line(context) {
            self.move_to_next_grapheme_boundary(context)
        } else if !self.is_at_last_line(context) {
            self.move_to_start_of_next_line()
        } else {
            self
        }
    }

    fn move_to_end_of_prev_line(self, context: &Context) -> Self {
        let prev_line_index = self.caret.position.line_index - 1;
        Self {
            caret: Position {
                position: text::Position {
                    line_index: prev_line_index,
                    byte_index: context.lines[prev_line_index].len(),
                },
                affinity: Affinity::After,
            },
            ..self
        }
    }

    fn move_to_start_of_next_line(self) -> Self {
        Self {
            caret: Position {
                position: text::Position {
                    line_index: self.caret.position.line_index + 1,
                    byte_index: 0,
                },
                affinity: Affinity::Before,
            },
            ..self
        }
    }

    fn move_to_prev_grapheme_boundary(self, context: &Context) -> Self {
        use crate::str::StrExt;

        Self {
            caret: Position {
                position: text::Position {
                    byte_index: context.lines[self.caret.position.line_index]
                        .prev_grapheme_boundary(self.caret.position.byte_index)
                        .unwrap(),
                    ..self.caret.position
                },
                affinity: Affinity::After,
            },
            ..self
        }
    }

    fn move_to_next_grapheme_boundary(self, context: &Context) -> Self {
        use crate::str::StrExt;

        Self {
            caret: Position {
                position: text::Position {
                    byte_index: context.lines[self.caret.position.line_index]
                        .next_grapheme_boundary(self.caret.position.byte_index)
                        .unwrap(),
                    ..self.caret.position
                },
                affinity: Affinity::Before,
            },
            ..self
        }
    }

    pub fn apply_delta(self, delta_len: DeltaDesc) -> Self {
        Self {
            caret: self.caret.apply_delta(delta_len),
            anchor: self.anchor.apply_delta(delta_len),
        }
    }

    pub fn merge(mut self, mut other: Self) -> Option<Self> {
        use std::{cmp::Ordering, mem};

        if self.start() > other.start() {
            mem::swap(&mut self, &mut other);
        }
        match (self.is_empty(), other.is_empty()) {
            (true, true) if self.caret.position == other.caret.position => Some(self),
            (false, true) if other.caret.position <= self.end().position => Some(self),
            (true, false) if self.caret.position == other.start().position => Some(other),
            (false, false) if self.end().position > other.start().position => {
                Some(match self.caret.cmp(&self.anchor) {
                    Ordering::Less => Self {
                        caret: self.caret.min(other.caret),
                        anchor: self.anchor.max(other.anchor),
                    },
                    Ordering::Greater => Self {
                        caret: self.caret.max(other.caret),
                        anchor: self.anchor.min(other.anchor),
                    },
                    Ordering::Equal => unreachable!(),
                })
            }
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Position {
    pub position: text::Position,
    pub affinity: Affinity,
}

impl Position {
    pub fn is_right_before(self, position: text::Position) -> bool {
        self.position == position && self.affinity == Affinity::Before
    }

    pub fn is_right_after(self, position: text::Position) -> bool {
        self.position == position && self.affinity == Affinity::After
    }

    pub fn apply_delta(self, delta_len: DeltaDesc) -> Self {
        Self {
            position: self.position.apply_delta(delta_len),
            affinity: self.affinity,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Affinity {
    Before,
    After,
}

impl Default for Affinity {
    fn default() -> Self {
        Self::Before
    }
}

#[derive(Debug)]
pub struct Context<'a> {
    pub lines: &'a [String],
}

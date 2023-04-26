use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Delta {
    pub range: Range,
    pub replace_with: Vec<String>,
}

impl Delta {
    pub fn desc(&self) -> DeltaDesc {
        DeltaDesc {
            range: self.range,
            replace_with_len: Size {
                line_count: self.replace_with.len() - 1,
                byte_count: self.replace_with.last().unwrap().len(),
            }
        }
    }

    pub fn apply(mut self, lines: &mut Vec<String>) {
        self.replace_with.first_mut().unwrap().replace_range(
            ..0,
            &lines[self.range.start().line_index][..self.range.start().byte_index],
        );
        self
            .replace_with
            .last_mut()
            .unwrap()
            .push_str(&lines[self.range.end().line_index][self.range.end().byte_index..]);
        lines.splice(
            self.range.start().line_index..=self.range.end().line_index,
            self.replace_with,
        );
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DeltaDesc {
    pub range: Range,
    pub replace_with_len: Size,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

impl Range {
    pub fn is_empty(self) -> bool {
        self.start == self.end
    }

    pub fn len(self) -> Size {
        self.end - self.start
    }

    pub fn start(self) -> Position {
        self.start
    }

    pub fn end(self) -> Position {
        self.end
    }

    pub fn apply_delta(self, delta_len: DeltaDesc) -> Self {
        Self {
            start: self.start.apply_delta(delta_len),
            end: self.end.apply_delta(delta_len),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Position {
    pub line_index: usize,
    pub byte_index: usize,
}

impl Position {
    pub fn apply_delta(self, delta_len: DeltaDesc) -> Self {
        if self < delta_len.range.start() {
            self
        } else {
            delta_len.range.start()
                + delta_len.replace_with_len
                + (self.max(delta_len.range.end()) - delta_len.range.end())
        }
    }
}

impl Add<Size> for Position {
    type Output = Self;

    fn add(self, size: Size) -> Self::Output {
        if size.line_count == 0 {
            Self {
                line_index: self.line_index,
                byte_index: self.byte_index + size.byte_count,
            }
        } else {
            Self {
                line_index: self.line_index + size.line_count,
                byte_index: size.byte_count,
            }
        }
    }
}

impl AddAssign<Size> for Position {
    fn add_assign(&mut self, size: Size) {
        *self = *self + size;
    }
}

impl Sub for Position {
    type Output = Size;

    fn sub(self, other: Self) -> Self::Output {
        if self.line_index == other.line_index {
            Size {
                line_count: 0,
                byte_count: other.byte_index - self.byte_index,
            }
        } else {
            Size {
                line_count: other.line_index - self.line_index,
                byte_count: other.byte_index,
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Size {
    pub line_count: usize,
    pub byte_count: usize,
}

impl Add for Size {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if other.line_count == 0 {
            Self {
                line_count: self.line_count,
                byte_count: self.byte_count + other.byte_count,
            }
        } else {
            Self {
                line_count: self.line_count + other.line_count,
                byte_count: other.byte_count,
            }
        }
    }
}

impl AddAssign<Size> for Size {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Size {
    type Output = Size;

    fn sub(self, other: Self) -> Self::Output {
        if self.line_count == other.line_count {
            Self {
                line_count: 0,
                byte_count: other.byte_count - self.byte_count,
            }
        } else {
            Self {
                line_count: other.line_count - self.line_count,
                byte_count: other.byte_count,
            }
        }
    }
}

impl SubAssign<Size> for Size {
    fn sub_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}
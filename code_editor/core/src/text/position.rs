use {
    super::{DeltaLen, Size},
    std::ops::{Add, AddAssign, Sub},
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Position {
    pub line_index: usize,
    pub byte_index: usize,
}

impl Position {
    pub fn apply_delta(self, delta_len: DeltaLen) -> Self {
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

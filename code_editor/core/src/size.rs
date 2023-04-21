use std::ops::{Add, AddAssign, Sub, SubAssign};

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

use {
    super::Affinity,
    crate::{text, text::DeltaLen},
};

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

    pub fn apply_delta(self, delta_len: DeltaLen) -> Self {
        Self {
            position: self.position.apply_delta(delta_len),
            affinity: self.affinity,
        }
    }
}

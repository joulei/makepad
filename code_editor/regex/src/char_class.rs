use {crate::Range, makepad_range_set::RangeSet};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub(crate) struct CharClass {
    range_set: RangeSet<u32>,
}

impl CharClass {
    pub(crate) fn new() -> Self {
        CharClass::default()
    }

    pub(crate) fn any() -> Self {
        let mut char_class = Self::new();
        char_class.insert(Range::new('\0', char::MAX));
        char_class
    }

    pub(crate) fn from_sorted_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Range<char>>,
    {
        Self {
            range_set: RangeSet::from_sorted_vec(
                iter.into_iter()
                    .map(|range| range.start as u32..range.end as u32 + 1)
                    .collect(),
            ),
        }
    }

    pub(crate) fn contains(&self, ch: char) -> bool {
        self.range_set.contains(&(ch as u32..ch as u32 + 1))
    }

    pub(crate) fn iter(&self) -> Iter<'_> {
        Iter {
            iter: self.range_set.iter(),
        }
    }

    pub(crate) fn negate(&self, output: &mut Self) {
        Self::any().difference(self, output);
    }

    pub(crate) fn union(&self, other: &Self, output: &mut Self) {
        output
            .range_set
            .extend(self.range_set.union(&other.range_set));
    }

    pub(crate) fn difference(&self, other: &Self, output: &mut Self) {
        output
            .range_set
            .extend(self.range_set.difference(&other.range_set));
    }

    pub(crate) fn insert(&mut self, char_range: Range<char>) -> bool {
        let range = Range::new(char_range.start as u32, char_range.end as u32);
        if range.start <= 0xD7FF && range.end >= 0xE000 {
            let mut is_new = false;
            is_new |= self.range_set.insert(range.start..0xD800);
            is_new |= self.range_set.insert(0xE000..range.end + 1);
            is_new
        } else {
            self.range_set.insert(range.start..range.end + 1)
        }
    }

    pub(crate) fn clear(&mut self) {
        self.range_set.clear()
    }
}

impl<'a> IntoIterator for &'a CharClass {
    type IntoIter = Iter<'a>;
    type Item = Range<char>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Iter<'a> {
    iter: makepad_range_set::Iter<'a, u32>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = Range<char>;

    fn next(&mut self) -> Option<Self::Item> {
        let range = self.iter.next()?;
        Some(Range::new(
            char::from_u32(range.start).unwrap(),
            char::from_u32(range.end - 1).unwrap(),
        ))
    }
}

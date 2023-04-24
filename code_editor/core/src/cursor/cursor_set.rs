use {super::Cursor, std::slice};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct CursorSet {
    cursors: Vec<Cursor>,
}

impl CursorSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.cursors.is_empty()
    }

    pub fn len(&self) -> usize {
        self.cursors.len()
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter {
            iter: self.cursors.iter(),
        }
    }

    pub fn update<F>(&mut self, mut f: F)
    where
        F: FnMut(Cursor) -> Cursor,
    {
        for cursor in &mut self.cursors {
            *cursor = f(*cursor);
        }
        self.normalize();
    }

    pub fn insert(&mut self, cursor: Cursor) {
        let mut index = match self
            .cursors
            .binary_search_by_key(&cursor.start(), |cursor| cursor.start())
        {
            Ok(index) => index,
            Err(index) => index,
        };
        self.cursors.insert(index, cursor);
        if index > 0 && self.merge(index - 1) {
            index -= 1;
        }
        while index < self.cursors.len() - 1 {
            if !self.merge(index) {
                break;
            }
        }
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(Cursor) -> bool,
    {
        self.cursors.retain(|&cursor| f(cursor));
    }

    pub fn clear(&mut self) {
        self.cursors.clear()
    }

    fn normalize(&mut self) {
        if self.cursors.is_empty() {
            return;
        }
        self.cursors.sort_by_key(|cursor| cursor.start());
        let mut index = 0;
        while index < self.cursors.len() - 1 {
            if !self.merge(index) {
                index += 1;
            }
        }
    }

    fn merge(&mut self, index: usize) -> bool {
        if let Some(merged_cursor) = self.cursors[index].merge(self.cursors[index + 1]) {
            self.cursors[index] = merged_cursor;
            self.cursors.remove(index + 1);
            true
        } else {
            false
        }
    }
}

impl Extend<Cursor> for CursorSet {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Cursor>,
    {
        self.cursors.extend(iter);
        self.normalize();
    }
}

impl<const N: usize> From<[Cursor; N]> for CursorSet {
    fn from(array: [Cursor; N]) -> Self {
        array.into_iter().collect()
    }
}

impl FromIterator<Cursor> for CursorSet {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Cursor>,
    {
        let mut cursors = CursorSet::new();
        cursors.extend(iter);
        cursors
    }
}

impl<'a> IntoIterator for &'a CursorSet {
    type Item = Cursor;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Clone, Debug)]
pub struct Iter<'a> {
    iter: slice::Iter<'a, Cursor>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = Cursor;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().copied()
    }
}

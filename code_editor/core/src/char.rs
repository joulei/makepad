pub trait CharExt {
    fn column_count(self) -> usize;
}

impl CharExt for char {
    fn column_count(self) -> usize {
        1
    }
}
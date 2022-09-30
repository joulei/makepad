pub trait CharExt {
    fn is_ascii_word(&self) -> bool;
}

impl CharExt for char {
    fn is_ascii_word(&self) -> bool {
        match self {
            '0'..='9' | 'A'..='Z' | '_' | 'a'..='z' => true,
            _ => false,
        }
    }
}

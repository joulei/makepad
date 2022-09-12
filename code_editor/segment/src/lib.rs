pub mod cursor;
pub mod str;

pub use self::{cursor::Cursor, str::StrExt};

#[cfg(test)]
mod tests;
#[cfg(test)]
mod test_data;
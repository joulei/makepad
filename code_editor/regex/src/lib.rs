mod ast;
mod case_folder;
mod char;
mod char_class;
mod code_generator;
mod cursor;
mod dfa;
mod nfa;
mod parser;
mod program;
mod range;
mod regex;
mod sparse_set;
mod str_cursor;
mod unicode_tables;
mod utf8_encoder;

pub use self::regex::Regex;

use self::{
    ast::Ast, case_folder::CaseFolder, char_class::CharClass, code_generator::CodeGenerator,
    cursor::Cursor, dfa::Dfa, nfa::Nfa, parser::Parser, program::Program, range::Range,
    sparse_set::SparseSet, str_cursor::StrCursor, utf8_encoder::Utf8Encoder,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let regex = Regex::new("[[:^alpha:]]+");
        let mut slots = [None; 2];
        println!("{:?}", regex.run("xxxa123AaBcCcyyy", &mut slots));
        println!("{:?}", slots);
    }
}

mod ast;
mod compiler;
mod parser;
mod prog;

use self::{ast::Ast, prog::Prog};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let ast = parser::parse("ab*|(cd)?").unwrap();
        println!("{:#?}", ast);
        let prog = compiler::compile(&ast);
        println!("{:#?}", prog);
    }
}

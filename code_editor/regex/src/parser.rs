use crate::{ast::Quant, Ast};

pub(crate) fn parse(string: &str) -> Result<Ast, ParseError> {
    ParseContext {
        string,
        byte_position: 0,
    }
    .parse()
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct ParseError;

struct ParseContext<'a> {
    string: &'a str,
    byte_position: usize,
}

impl<'a> ParseContext<'a> {
    fn parse(&mut self) -> Result<Ast, ParseError> {
        match self.peek_char() {
            Some(')') | None => Ok(Ast::Empty),
            _ => self.parse_alt(),
        }
    }

    fn parse_alt(&mut self) -> Result<Ast, ParseError> {
        let mut ast = self.parse_cat()?;
        loop {
            match self.peek_char() {
                Some('|') => {
                    self.skip_char();
                    let ast_0 = ast;
                    let ast_1 = self.parse_cat()?;
                    ast = Ast::Alt(Box::new(ast_0), Box::new(ast_1));
                }
                _ => break,
            }
        }
        Ok(ast)
    }

    fn parse_cat(&mut self) -> Result<Ast, ParseError> {
        let mut ast = self.parse_rep()?;
        loop {
            match self.peek_char() {
                Some('|') | Some(')') | None => break,
                _ => {
                    let ast_0 = ast;
                    let ast_1 = self.parse_cat()?;
                    ast = Ast::Cat(Box::new(ast_0), Box::new(ast_1));
                }
            }
        }
        Ok(ast)
    }

    fn parse_rep(&mut self) -> Result<Ast, ParseError> {
        let mut ast = self.parse_atom()?;
        match self.peek_char().and_then(|ch| ch.to_quant()) {
            Some(quant) => {
                self.skip_char();
                let non_greedy = match self.peek_char() {
                    Some('?') => {
                        self.skip_char();
                        true
                    }
                    _ => false,
                };
                ast = Ast::Rep(Box::new(ast), quant, non_greedy);
            }
            _ => (),
        }
        Ok(ast)
    }

    fn parse_atom(&mut self) -> Result<Ast, ParseError> {
        Ok(match self.peek_char() {
            Some('(') => {
                self.skip_char();
                let ast = self.parse()?;
                match self.peek_char() {
                    Some(')') => {
                        self.skip_char();
                        ast
                    }
                    _ => return Err(ParseError),
                }
            }
            Some(ch) => {
                self.skip_char();
                Ast::Char(ch)
            }
            _ => panic!(),
        })
    }

    fn peek_char(&self) -> Option<char> {
        self.string[self.byte_position..].chars().next()
    }

    fn skip_char(&mut self) {
        self.byte_position += self.peek_char().unwrap().len_utf8();
    }
}

trait CharExt {
    fn to_quant(self) -> Option<Quant>;
}

impl CharExt for char {
    fn to_quant(self) -> Option<Quant> {
        match self {
            '?' => Some(Quant::Quest),
            '*' => Some(Quant::Star),
            '+' => Some(Quant::Plus),
            _ => None,
        }
    }
}

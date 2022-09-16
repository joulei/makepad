#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) enum Ast {
    Empty,
    Char(char),
    Rep(Box<Ast>, Quant, bool),
    Cat(Box<Ast>, Box<Ast>),
    Alt(Box<Ast>, Box<Ast>),
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) enum Quant {
    Quest,
    Star,
    Plus,
}

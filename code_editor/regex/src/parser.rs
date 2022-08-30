use {
    crate::{
        ast::{Pred, Quant},
        Ast, CaseFolder, CharClass, Range,
    },
    std::str::Chars,
};

#[derive(Clone, Debug, Default)]
pub(crate) struct Parser {
    asts: Vec<Ast>,
    groups: Vec<Group>,
    case_folder: CaseFolder,
    char_class: CharClass,
}

impl Parser {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn parse(&mut self, pattern: &str, options: Options) -> Ast {
        let mut chars = pattern.chars();
        ParseContext {
            asts: &mut self.asts,
            groups: &mut self.groups,
            folder: &mut self.case_folder,
            char_class: &mut self.char_class,
            ch_0: chars.next(),
            ch_1: chars.next(),
            chars,
            position: 0,
            options,
            cap_count: 1,
            group: Group::new(Some(0)),
        }
        .parse()
    }
}

#[derive(Debug)]
struct ParseContext<'a> {
    folder: &'a mut CaseFolder,
    char_class: &'a mut CharClass,
    asts: &'a mut Vec<Ast>,
    groups: &'a mut Vec<Group>,
    ch_0: Option<char>,
    ch_1: Option<char>,
    chars: Chars<'a>,
    position: usize,
    options: Options,
    cap_count: usize,
    group: Group,
}

impl<'a> ParseContext<'a> {
    fn parse(&mut self) -> Ast {
        loop {
            match self.peek_char() {
                Some('|') => {
                    self.skip_char();
                    self.maybe_push_cat();
                    self.pop_cats();
                    self.group.alt_count += 1;
                }
                Some('?') => {
                    self.skip_char();
                    let mut is_lazy = false;
                    if self.peek_char() == Some('?') {
                        self.skip_char();
                        is_lazy = true;
                    }
                    let ast = self.asts.pop().unwrap();
                    self.asts
                        .push(Ast::Rep(Box::new(ast), Quant::Quest(is_lazy)));
                }
                Some('*') => {
                    self.skip_char();
                    let mut is_lazy = false;
                    if self.peek_char() == Some('?') {
                        self.skip_char();
                        is_lazy = true;
                    }
                    let ast = self.asts.pop().unwrap();
                    self.asts
                        .push(Ast::Rep(Box::new(ast), Quant::Star(is_lazy)));
                }
                Some('+') => {
                    self.skip_char();
                    let mut is_lazy = false;
                    if self.peek_char() == Some('?') {
                        self.skip_char();
                        is_lazy = true;
                    }
                    let ast = self.asts.pop().unwrap();
                    self.asts
                        .push(Ast::Rep(Box::new(ast), Quant::Plus(is_lazy)));
                }
                Some('^') => {
                    self.skip_char();
                    self.maybe_push_cat();
                    self.asts.push(Ast::Assert(Pred::IsAtStartOfText));
                    self.group.ast_count += 1;
                }
                Some('$') => {
                    self.skip_char();
                    self.maybe_push_cat();
                    self.asts.push(Ast::Assert(Pred::IsAtEndOfText));
                    self.group.ast_count += 1;
                }
                Some('(') => {
                    self.skip_char();
                    let cap = match self.peek_two_chars() {
                        (Some('?'), Some(':')) => {
                            self.skip_two_chars();
                            false
                        }
                        _ => true,
                    };
                    self.push_group(cap);
                }
                Some(')') => {
                    self.skip_char();
                    self.pop_group();
                }
                Some('[') => {
                    self.maybe_push_cat();
                    let char_class = self.parse_char_class();
                    self.asts.push(Ast::CharClass(char_class));
                    self.group.ast_count += 1;
                }
                Some('.') => {
                    self.skip_char();
                    self.maybe_push_cat();
                    self.asts.push(Ast::CharClass(CharClass::any()));
                    self.group.ast_count += 1;
                }
                Some(ch) => {
                    self.skip_char();
                    self.maybe_push_cat();
                    self.asts.push(Ast::Char(ch));
                    self.group.ast_count += 1;
                }
                None => break,
            }
        }
        self.maybe_push_cat();
        self.pop_alts();
        self.asts.pop().unwrap()
    }

    fn parse_char_class(&mut self) -> CharClass {
        use std::mem;

        let mut char_class = CharClass::new();
        self.skip_char();
        let mut is_negated = false;
        if self.peek_char() == Some('^') {
            self.skip_char();
            is_negated = true;
        }
        let mut is_first = true;
        loop {
            match self.peek_char() {
                Some(']') if !is_first => {
                    self.skip_char();
                    break;
                }
                _ => {
                    let char_range = self.parse_char_range();
                    if self.options.ignore_case {
                        self.folder.fold(char_range, &mut char_class);
                    } else {
                        char_class.insert(char_range);
                    }
                }
            }
            is_first = false;
        }
        if is_negated {
            char_class.negate(&mut self.char_class);
            mem::swap(&mut char_class, &mut self.char_class);
            self.char_class.clear();
        }
        char_class
    }

    fn parse_char_range(&mut self) -> Range<char> {
        let start = self.parse_char();
        match self.peek_two_chars() {
            (Some('-'), ch) if ch != Some(']') => {
                self.skip_char();
                let end = self.parse_char();
                return Range::new(start, end);
            }
            _ => Range::new(start, start),
        }
    }

    fn parse_char(&mut self) -> char {
        let ch = self.peek_char().unwrap();
        self.skip_char();
        ch
    }

    fn peek_char(&self) -> Option<char> {
        self.ch_0
    }

    fn peek_two_chars(&self) -> (Option<char>, Option<char>) {
        (self.ch_0, self.ch_1)
    }

    fn skip_char(&mut self) {
        self.position += self.ch_0.unwrap().len_utf8();
        self.ch_0 = self.ch_1;
        self.ch_1 = self.chars.next();
    }

    fn skip_two_chars(&mut self) {
        self.position += self.ch_1.unwrap().len_utf8();
        self.position += self.ch_1.unwrap().len_utf8();
        self.ch_0 = self.chars.next();
        self.ch_1 = self.chars.next();
    }

    fn push_group(&mut self, cap: bool) {
        use std::mem;

        self.maybe_push_cat();
        self.pop_cats();
        let cap_index = if cap {
            let cap_index = self.cap_count;
            self.cap_count += 1;
            Some(cap_index)
        } else {
            None
        };
        let group = mem::replace(&mut self.group, Group::new(cap_index));
        self.groups.push(group);
    }

    fn pop_group(&mut self) {
        self.maybe_push_cat();
        self.pop_alts();
        if let Some(index) = self.group.cap {
            let ast = self.asts.pop().unwrap();
            self.asts.push(Ast::Cap(Box::new(ast), index));
        }
        self.group = self.groups.pop().unwrap();
        self.group.ast_count += 1;
    }

    fn maybe_push_cat(&mut self) {
        if self.group.ast_count - self.group.alt_count - self.group.cat_count == 2 {
            self.group.cat_count += 1;
        }
    }

    fn pop_alts(&mut self) {
        self.pop_cats();
        if self.group.alt_count == 0 {
            return;
        }
        let asts = self
            .asts
            .split_off(self.asts.len() - (self.group.alt_count + 1));
        self.asts.push(Ast::Alt(asts));
        self.group.ast_count -= self.group.alt_count;
        self.group.alt_count = 0;
    }

    fn pop_cats(&mut self) {
        if self.group.cat_count == 0 {
            return;
        }
        let asts = self
            .asts
            .split_off(self.asts.len() - (self.group.cat_count + 1));
        self.asts.push(Ast::Cat(asts));
        self.group.ast_count -= self.group.cat_count;
        self.group.cat_count = 0;
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Options {
    pub(crate) ignore_case: bool,
}

#[derive(Clone, Copy, Debug)]
struct Group {
    cap: Option<usize>,
    ast_count: usize,
    alt_count: usize,
    cat_count: usize,
}

impl Group {
    fn new(index: Option<usize>) -> Self {
        Self {
            cap: index,
            ast_count: 0,
            alt_count: 0,
            cat_count: 0,
        }
    }
}

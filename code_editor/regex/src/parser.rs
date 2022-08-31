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

    pub(crate) fn parse(&mut self, pattern: &str) -> Ast {
        let mut chars = pattern.chars();
        ParseContext {
            asts: &mut self.asts,
            groups: &mut self.groups,
            case_folder: &mut self.case_folder,
            char_class: &mut self.char_class,
            pattern,
            ch_0: chars.next(),
            ch_1: chars.next(),
            chars,
            byte_position: 0,
            cap_count: 1,
            group: Group::new(Some(0), Flags::default()),
        }
        .parse()
    }
}

#[derive(Debug)]
struct ParseContext<'a> {
    asts: &'a mut Vec<Ast>,
    groups: &'a mut Vec<Group>,
    case_folder: &'a mut CaseFolder,
    char_class: &'a mut CharClass,
    pattern: &'a str,
    ch_0: Option<char>,
    ch_1: Option<char>,
    chars: Chars<'a>,
    byte_position: usize,
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
                    let mut non_greedy = false;
                    if self.peek_char() == Some('?') {
                        self.skip_char();
                        non_greedy = true;
                    }
                    let ast = self.asts.pop().unwrap();
                    self.asts
                        .push(Ast::Rep(Box::new(ast), Quant::Quest(non_greedy)));
                }
                Some('*') => {
                    self.skip_char();
                    let mut non_greedy = false;
                    if self.peek_char() == Some('?') {
                        self.skip_char();
                        non_greedy = true;
                    }
                    let ast = self.asts.pop().unwrap();
                    self.asts
                        .push(Ast::Rep(Box::new(ast), Quant::Star(non_greedy)));
                }
                Some('+') => {
                    self.skip_char();
                    let mut non_greedy = false;
                    if self.peek_char() == Some('?') {
                        self.skip_char();
                        non_greedy = true;
                    }
                    let ast = self.asts.pop().unwrap();
                    self.asts
                        .push(Ast::Rep(Box::new(ast), Quant::Plus(non_greedy)));
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
                    match self.peek_char() {
                        Some('?') => {
                            self.skip_char();
                            let flags = self.parse_flags();
                            match self.peek_char() {
                                Some(':') => {
                                    self.skip_char();
                                    self.push_group(true, flags);
                                }
                                Some(')') => {
                                    self.skip_char();
                                    self.group.flags = flags;
                                }
                                _ => panic!(),
                            }
                        }
                        _ => self.push_group(false, Flags::default()),
                    };
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
                    self.asts.push(if self.group.flags.case_insensitive {
                        let mut char_class = CharClass::new();
                        self.case_folder.fold(Range::new(ch, ch), &mut char_class);
                        Ast::CharClass(char_class)
                    } else {
                        Ast::Char(ch)
                    });
                    self.group.ast_count += 1;
                }
                None => break,
            }
        }
        self.maybe_push_cat();
        self.pop_alts();
        self.asts.pop().unwrap()
    }

    fn parse_flags(&mut self) -> Flags {
        let mut flags = Flags::default();
        loop {
            match self.peek_char() {
                Some(':') | Some(')') => break,
                Some('i') => {
                    self.skip_char();
                    flags.case_insensitive = true;
                }
                _ => panic!(),
            }
        }
        flags
    }

    fn parse_char_class(&mut self) -> CharClass {
        use std::mem;

        let mut char_class = CharClass::new();
        self.skip_char();
        let mut negated = false;
        if self.peek_char() == Some('^') {
            self.skip_char();
            negated = true;
        }
        let mut first = true;
        loop {
            match self.peek_two_chars() {
                (Some('['), Some(':')) => {
                    char_class.union(&self.parse_named_char_class(), &mut self.char_class);
                    mem::swap(&mut char_class, &mut self.char_class);
                    self.char_class.clear();
                }
                (Some(']'), _) if !first => {
                    self.skip_char();
                    break;
                }
                _ => {
                    let char_range = self.parse_char_range();
                    if self.group.flags.case_insensitive {
                        self.case_folder.fold(char_range, &mut char_class);
                    } else {
                        char_class.insert(char_range);
                    }
                }
            }
            first = false;
        }
        if negated {
            char_class.negate(&mut self.char_class);
            mem::swap(&mut char_class, &mut self.char_class);
            self.char_class.clear();
        }
        char_class
    }

    fn parse_named_char_class(&mut self) -> CharClass {
        use {crate::unicode_tables::compatibility_properties, std::mem};

        self.skip_two_chars();
        let mut negated = false;
        if self.peek_char() == Some('^') {
            self.skip_char();
            negated = true;
        }
        let start = self.byte_position;
        let end;
        loop {
            match self.peek_two_chars() {
                (Some(':'), Some(']')) => {
                    end = self.byte_position;
                    self.skip_two_chars();
                    break;
                }
                (Some(_), _) => self.skip_char(),
                (None, _) => panic!(),
            }
        }
        let mut char_class = CharClass::from_sorted_iter(
            match &self.pattern[start..end] {
                "alnum" => compatibility_properties::ALNUM.as_slice(),
                "alpha" => compatibility_properties::ALPHA.as_slice(),
                "blank" => compatibility_properties::BLANK.as_slice(),
                "cntrl" => compatibility_properties::CNTRL.as_slice(),
                "digit" => compatibility_properties::DIGIT.as_slice(),
                "graph" => compatibility_properties::GRAPH.as_slice(),
                "lower" => compatibility_properties::LOWER.as_slice(),
                "print" => compatibility_properties::PRINT.as_slice(),
                "punct" => compatibility_properties::PUNCT.as_slice(),
                "space" => compatibility_properties::SPACE.as_slice(),
                "upper" => compatibility_properties::UPPER.as_slice(),
                "word" => compatibility_properties::WORD.as_slice(),
                "xdigit" => compatibility_properties::XDIGIT.as_slice(),
                _ => panic!(),
            }
            .iter()
            .cloned(),
        );
        if negated {
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
        self.byte_position += self.ch_0.unwrap().len_utf8();
        self.ch_0 = self.ch_1;
        self.ch_1 = self.chars.next();
    }

    fn skip_two_chars(&mut self) {
        self.byte_position += self.ch_1.unwrap().len_utf8();
        self.byte_position += self.ch_1.unwrap().len_utf8();
        self.ch_0 = self.chars.next();
        self.ch_1 = self.chars.next();
    }

    fn push_group(&mut self, cap: bool, flags: Flags) {
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
        let group = mem::replace(&mut self.group, Group::new(cap_index, flags));
        self.groups.push(group);
    }

    fn pop_group(&mut self) {
        self.maybe_push_cat();
        self.pop_alts();
        if let Some(cap_index) = self.group.cap_index {
            let ast = self.asts.pop().unwrap();
            self.asts.push(Ast::Capture(Box::new(ast), cap_index));
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

#[derive(Clone, Copy, Debug)]
struct Group {
    cap_index: Option<usize>,
    flags: Flags,
    ast_count: usize,
    alt_count: usize,
    cat_count: usize,
}

impl Group {
    fn new(cap_index: Option<usize>, flags: Flags) -> Self {
        Self {
            cap_index,
            flags,
            ast_count: 0,
            alt_count: 0,
            cat_count: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Flags {
    case_insensitive: bool,
}

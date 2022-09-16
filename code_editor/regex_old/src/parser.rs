use {
    crate::{
        ast::{Pred, Quant},
        Ast, CaseFolder, CharClass, Range,
    },
    std::{error, fmt},
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

    pub(crate) fn parse(&mut self, pattern: &str) -> Result<Ast, ParseError> {
        ParseContext {
            asts: &mut self.asts,
            groups: &mut self.groups,
            case_folder: &mut self.case_folder,
            char_class: &mut self.char_class,
            pattern,
            byte_position: 0,
            cap_count: 1,
            group: Group::new(GroupFlags::default(), Some(0), ),
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
    byte_position: usize,
    cap_count: usize,
    group: Group,
}

impl<'a> ParseContext<'a> {
    fn parse(&mut self) -> Result<Ast, ParseError> {
        loop {
            match self.peek_two_chars() {
                (Some('|'), _) => {
                    self.skip_char();
                    self.maybe_push_cat();
                    self.pop_cats();
                    if self.asts.is_empty() {
                        return Err(ParseError);
                    }
                    self.group.alt_count += 1;
                }
                (Some('?'), _) => {
                    self.skip_char();
                    let mut non_greedy = false;
                    if self.peek_char() == Some('?') {
                        self.skip_char();
                        non_greedy = true;
                    }
                    let ast = self.asts.pop().ok_or(ParseError)?;
                    self.asts
                        .push(Ast::Rep(Box::new(ast), Quant::Quest(non_greedy)));
                }
                (Some('*'), _) => {
                    self.skip_char();
                    if self.asts.is_empty() {
                        self.push_char('*');
                    } else {
                        let mut non_greedy = false;
                        if self.peek_char() == Some('?') {
                            self.skip_char();
                            non_greedy = true;
                        }
                        let ast = self.asts.pop().unwrap();
                        self.asts
                            .push(Ast::Rep(Box::new(ast), Quant::Star(non_greedy)));
                    }
                }
                (Some('+'), _) => {
                    self.skip_char();
                    let mut non_greedy = false;
                    if self.peek_char() == Some('?') {
                        self.skip_char();
                        non_greedy = true;
                    }
                    let ast = self.asts.pop().ok_or(ParseError)?;
                    self.asts
                        .push(Ast::Rep(Box::new(ast), Quant::Plus(non_greedy)));
                }
                (Some('{'), _) => match self.try_parse_counted() {
                    Some((min, max, non_greedy)) => {
                        let ast = self.asts.pop().ok_or(ParseError)?;
                        self.asts.push(Ast::Rep(
                            Box::new(ast),
                            Quant::Counted(min, max, non_greedy),
                        ));
                    }
                    None => {
                        self.skip_char();
                        self.push_char('{');
                    }
                },
                (Some('^'), _) => {
                    self.skip_char();
                    self.maybe_push_cat();
                    self.asts.push(Ast::Assert(if self.group.flags.multiline {
                        Pred::IsAtStartOfLine
                    } else {
                        Pred::IsAtStartOfText
                    }));
                    self.group.ast_count += 1;
                }
                (Some('$'), _) => {
                    self.skip_char();
                    self.maybe_push_cat();
                    self.asts.push(Ast::Assert(if self.group.flags.multiline {
                        Pred::IsAtEndOfLine
                    } else {
                        Pred::IsAtEndOfText
                    }));
                    self.group.ast_count += 1;
                }
                (Some('\\'), Some('b')) => {
                    self.skip_two_chars();
                    self.maybe_push_cat();
                    self.asts.push(Ast::Assert(Pred::IsAtWordBoundary));
                    self.group.ast_count += 1;
                }
                (Some('\\'), Some('B')) => {
                    self.skip_two_chars();
                    self.maybe_push_cat();
                    self.asts.push(Ast::Assert(Pred::IsNotAtWordBoundary));
                    self.group.ast_count += 1;
                }
                (Some('('), _) => {
                    self.skip_char();
                    let mut flags = GroupFlags::default();
                    let mut non_capturing = false;
                    if self.peek_char() == Some('?') {
                        flags = self.parse_group_flags()?;
                        if self.peek_char() == Some(':') {
                            self.skip_char();
                            non_capturing = true;
                        }
                    }
                    // TODO: Set flags for current group
                    self.push_group(flags, non_capturing);
                }
                (Some(')'), _) => {
                    self.skip_char();
                    self.pop_group()?;
                }
                (Some('['), _) => {
                    let char_class = self.parse_char_class()?;
                    self.push_char_class(char_class);
                }
                (Some('.'), _) => {
                    self.skip_char();
                    self.push_char_class(CharClass::any());
                }
                (Some('\\'), _) => match self.try_parse_escaped_char_class() {
                    Some(char_class) => {
                        self.push_char_class(char_class);
                    }
                    None => {
                        let ch = self.parse_escaped_char()?;
                        self.push_char(ch);
                    }
                },
                (Some(ch), _) => {
                    self.skip_char();
                    self.push_char(ch);
                }
                (None, _) => break,
            }
        }
        self.maybe_push_cat();
        self.pop_alts()?;
        Ok(self.asts.pop().unwrap_or(Ast::Empty))
    }

    fn try_parse_counted(&mut self) -> Option<(u32, Option<u32>, bool)> {
        let byte_position = self.byte_position;
        self.skip_char();
        let min = match self.parse_dec_int().ok() {
            Some(min) => min,
            None => {
                self.byte_position = byte_position;
                return None;
            }
        };
        let max = if self.peek_char() == Some(',') {
            self.skip_char();
            if self.peek_char() == Some('}') {
                None
            } else {
                match self.parse_dec_int().ok() {
                    Some(max) => Some(max),
                    None => {
                        self.byte_position = byte_position;
                        return None;
                    }
                }
            }
        } else {
            Some(min)
        };
        if self.peek_char() != Some('}') {
            self.byte_position = byte_position;
            return None;
        }
        self.skip_char();
        let mut non_greedy = false;
        if self.peek_char() == Some('?') {
            self.skip_char();
            non_greedy = true;
        }
        Some((min, max, non_greedy))
    }

    fn parse_group_flags(&mut self) -> Result<GroupFlags, ParseError> {
        self.skip_char();
        let mut flags = GroupFlags::default();
        loop {
            match self.peek_char() {
                Some(':') | Some(')') => break,
                Some('i') => {
                    self.skip_char();
                    flags.ignore_case = true
                },
                Some('m') => {
                    self.skip_char();
                    flags.multiline = true
                },
                _ => return Err(ParseError),
            }
        }
        Ok(flags)
    }

    fn parse_char_class(&mut self) -> Result<CharClass, ParseError> {
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
                (Some(']'), _) if !first => {
                    self.skip_char();
                    break;
                }
                (Some('['), Some(':')) => {
                    let other_char_class = self.parse_posix_char_class()?;
                    char_class.union(&other_char_class, &mut self.char_class);
                    mem::swap(&mut char_class, &mut self.char_class);
                    self.char_class.clear();
                }
                (Some(_), _) => match self.try_parse_escaped_char_class() {
                    Some(other_char_class) => {
                        char_class.union(&other_char_class, &mut self.char_class);
                        mem::swap(&mut char_class, &mut self.char_class);
                        self.char_class.clear();
                    }
                    None => {
                        char_class.insert(self.parse_char_range()?);
                    }
                },
                (None, _) => return Err(ParseError),
            }
            first = false;
        }
        if negated {
            char_class.negate(&mut self.char_class);
            mem::swap(&mut char_class, &mut self.char_class);
            self.char_class.clear();
        }
        Ok(char_class)
    }

    fn parse_posix_char_class(&mut self) -> Result<CharClass, ParseError> {
        use {crate::posix_char_classes::*, std::mem};

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
                (None, _) => return Err(ParseError),
            }
        }
        let mut char_class = CharClass::from_sorted_iter(
            match &self.pattern[start..end] {
                "alnum" => ALNUM.as_slice(),
                "alpha" => ALPHA.as_slice(),
                "blank" => BLANK.as_slice(),
                "cntrl" => CNTRL.as_slice(),
                "digit" => DIGIT.as_slice(),
                "graph" => GRAPH.as_slice(),
                "lower" => LOWER.as_slice(),
                "print" => PRINT.as_slice(),
                "punct" => PUNCT.as_slice(),
                "space" => SPACE.as_slice(),
                "upper" => UPPER.as_slice(),
                "word" => WORD.as_slice(),
                "xdigit" => XDIGIT.as_slice(),
                _ => return Err(ParseError),
            }
            .iter()
            .cloned(),
        );
        if negated {
            char_class.negate(&mut self.char_class);
            mem::swap(&mut char_class, &mut self.char_class);
            self.char_class.clear();
        }
        Ok(char_class)
    }

    fn parse_char_range(&mut self) -> Result<Range<char>, ParseError> {
        let start = self.parse_char()?;
        Ok(match self.peek_two_chars() {
            (Some('-'), ch) if ch != Some(']') => {
                self.skip_char();
                let end = self.parse_char()?;
                Range::new(start, end)
            }
            _ => Range::new(start, start),
        })
    }

    fn parse_char(&mut self) -> Result<char, ParseError> {
        let ch = self.peek_char().ok_or(ParseError)?;
        self.skip_char();
        Ok(ch)
    }

    fn try_parse_escaped_char_class(&mut self) -> Option<CharClass> {
        use {crate::posix_char_classes::*, std::mem};

        match self.peek_two_chars() {
            (Some('\\'), Some(ch)) => {
                let mut negated = false;
                if ch.is_ascii_uppercase() {
                    negated = true;
                }
                let ranges = match ch.to_ascii_lowercase() {
                    'd' => Some(DIGIT.as_slice()),
                    's' => Some(SPACE.as_slice()),
                    'w' => Some(WORD.as_slice()),
                    _ => None,
                };
                match ranges {
                    Some(ranges) => {
                        self.skip_two_chars();
                        let mut char_class = CharClass::from_sorted_iter(ranges.iter().cloned());
                        if negated {
                            char_class.negate(&mut self.char_class);
                            mem::swap(&mut char_class, &mut self.char_class);
                            self.char_class.clear();
                        }
                        Some(char_class)
                    }
                    None => None,
                }
            }
            _ => None,
        }
    }

    fn parse_escaped_char(&mut self) -> Result<char, ParseError> {
        use crate::char::CharExt;

        self.skip_char();
        let c = match self.peek_char() {
            Some('n') => '\n',
            Some('r') => '\r',
            Some('t') => '\t',
            Some(c) if !c.is_word() => c,
            _ => return Err(ParseError),
        };
        self.skip_char();
        Ok(c)
    }

    fn parse_dec_int(&mut self) -> Result<u32, ParseError> {
        let ch = match self.peek_char() {
            Some(ch) if ch.is_digit(10) => ch,
            _ => return Err(ParseError),
        };
        self.skip_char();
        let mut value = ch.to_digit(10).unwrap();
        loop {
            let ch = match self.peek_char() {
                Some(ch) if ch.is_digit(10) => ch,
                _ => break,
            };
            self.skip_char();
            value = value.checked_mul(10).ok_or(ParseError)? + ch.to_digit(10).unwrap();
        }
        Ok(value)
    }

    fn peek_char(&self) -> Option<char> {
        self.pattern[self.byte_position..].chars().next()
    }

    fn peek_two_chars(&self) -> (Option<char>, Option<char>) {
        let mut chars = self.pattern[self.byte_position..].chars();
        (chars.next(), chars.next())
    }

    fn skip_char(&mut self) {
        self.byte_position += self.peek_char().unwrap().len_utf8();
    }

    fn skip_two_chars(&mut self) {
        let (ch_0, ch_1) = self.peek_two_chars();
        self.byte_position += ch_0.unwrap().len_utf8() + ch_1.unwrap().len_utf8();
    }

    fn push_group(&mut self, flags: GroupFlags, non_capturing: bool) {
        use std::mem;

        self.maybe_push_cat();
        self.pop_cats();
        let cap_index = if non_capturing {
            None
        } else {
            let cap_index = self.cap_count;
            self.cap_count += 1;
            Some(cap_index)
        };
        let group = mem::replace(&mut self.group, Group::new(flags, cap_index));
        self.groups.push(group);
    }

    fn pop_group(&mut self) -> Result<(), ParseError> {
        self.maybe_push_cat();
        self.pop_alts()?;
        let mut ast = if self.group.ast_count == 0 {
            Ast::Empty
        } else {
            self.asts.pop().unwrap()
        };
        if let Some(cap_index) = self.group.cap_index {
            ast = Ast::Capture(Box::new(ast), cap_index);
        }
        self.asts.push(ast);
        self.group = self.groups.pop().ok_or(ParseError)?;
        self.group.ast_count += 1;
        Ok(())
    }

    fn push_char_class(&mut self, mut char_class: CharClass) {
        use std::mem;

        if self.group.flags.ignore_case {
            for char_range in &char_class {
                self.case_folder.fold(char_range, &mut self.char_class);
            }
            mem::swap(&mut char_class, &mut self.char_class);
            self.char_class.clear();
        }
        self.maybe_push_cat();
        self.asts.push(Ast::CharClass(char_class));
        self.group.ast_count += 1;
    }

    fn push_char(&mut self, ch: char) {
        let ast = if self.group.flags.ignore_case {
            let mut char_class = CharClass::new();
            self.case_folder.fold(Range::new(ch, ch), &mut char_class);
            Ast::CharClass(char_class)
        } else {
            Ast::Char(ch)
        };
        self.maybe_push_cat();
        self.asts.push(ast);
        self.group.ast_count += 1;
    }

    fn maybe_push_cat(&mut self) {
        if self.group.ast_count == self.group.alt_count + self.group.cat_count + 2 {
            self.group.cat_count += 1;
        }
    }

    fn pop_alts(&mut self) -> Result<(), ParseError> {
        self.pop_cats();
        if self.group.alt_count == 0 {
            return Ok(());
        }
        if self.group.ast_count < self.group.alt_count + 1 {
            return Err(ParseError);
        }
        let asts = self
            .asts
            .split_off(self.asts.len() - (self.group.alt_count + 1));
        self.asts.push(Ast::Alt(asts));
        self.group.ast_count -= self.group.alt_count;
        self.group.alt_count = 0;
        Ok(())
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

#[derive(Clone, Debug)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl error::Error for ParseError {}

#[derive(Clone, Copy, Debug)]
struct Group {
    flags: GroupFlags,
    cap_index: Option<usize>,
    ast_count: usize,
    alt_count: usize,
    cat_count: usize,
}

impl Group {
    fn new(flags: GroupFlags, cap_index: Option<usize>) -> Self {
        Self {
            flags,
            cap_index,
            ast_count: 0,
            alt_count: 0,
            cat_count: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct GroupFlags {
    ignore_case: bool,
    multiline: bool,
}

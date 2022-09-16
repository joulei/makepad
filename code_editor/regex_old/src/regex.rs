use {
    crate::{
        code_generator, dfa, nfa, parser, CodeGenerator, Cursor, Dfa, Nfa, Parser, Program,
        StrCursor,
    },
    std::{cell::RefCell, error, fmt, result::Result, sync::Arc},
};

#[derive(Clone, Debug)]
pub struct Regex {
    unique: Box<RefCell<Unique>>,
    shared: Arc<Shared>,
}

impl Regex {
    pub fn new(pattern: &str) -> Result<Self, Error> {
        let mut parser = Parser::new();
        let ast = parser.parse(pattern)?;
        let mut code_generator = CodeGenerator::new();
        let dfa_program = code_generator.generate(
            &ast,
            code_generator::Options {
                use_bytes: true,
                dot_star: true,
                ..code_generator::Options::default()
            },
        );
        let reverse_dfa_program = code_generator.generate(
            &ast,
            code_generator::Options {
                use_bytes: true,
                reverse: true,
                ..code_generator::Options::default()
            },
        );
        let nfa_program = code_generator.generate(&ast, code_generator::Options::default());
        Ok(Self {
            unique: Box::new(RefCell::new(Unique {
                dfa: Dfa::new(),
                reverse_dfa: Dfa::new(),
                nfa: Nfa::new(),
            })),
            shared: Arc::new(Shared {
                dfa_program,
                reverse_dfa_program,
                nfa_program,
            }),
        })
    }

    pub fn run(&self, haystack: &str, slots: &mut [Option<usize>]) -> bool {
        self.run_with_cursor(StrCursor::new(haystack), slots)
    }

    pub fn run_with_cursor<C: Cursor>(&self, mut cursor: C, slots: &mut [Option<usize>]) -> bool {
        let mut unique = self.unique.borrow_mut();
        match unique.dfa.run(
            &self.shared.dfa_program,
            &mut cursor,
            dfa::Options {
                stop_after_first_match: slots.is_empty(),
                ..dfa::Options::default()
            },
        ) {
            Ok(Some(end)) => {
                cursor.move_to(end);
                match unique.reverse_dfa.run(
                    &self.shared.reverse_dfa_program,
                    (&mut cursor).rev(),
                    dfa::Options {
                        continue_until_last_match: true,
                        ..dfa::Options::default()
                    },
                ) {
                    Ok(Some(start)) => {
                        cursor.move_to(start);
                        if slots.len() == 2 {
                            slots[0] = Some(start);
                            slots[1] = Some(end);
                        } else {
                            unique.nfa.run(
                                &self.shared.nfa_program,
                                cursor,
                                nfa::Options::default(),
                                slots,
                            );
                        }
                        return true;
                    }
                    Ok(None) => panic!(),
                    Err(_) => {}
                }
            }
            Ok(None) => return false,
            Err(_) => {}
        }
        let mut unique = self.unique.borrow_mut();
        unique.nfa.run(
            &self.shared.nfa_program,
            cursor,
            nfa::Options {
                stop_after_first_match: slots.is_empty(),
                ..nfa::Options::default()
            },
            slots,
        )
    }
}

#[derive(Clone, Debug)]
pub enum Error {
    Parse,
}

impl fmt::Display for Error {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl error::Error for Error {}

impl From<parser::ParseError> for Error {
    fn from(_error: parser::ParseError) -> Self {
        Error::Parse
    }
}

#[derive(Clone, Debug)]
struct Unique {
    dfa: Dfa,
    reverse_dfa: Dfa,
    nfa: Nfa,
}

#[derive(Debug)]
struct Shared {
    dfa_program: Program,
    reverse_dfa_program: Program,
    nfa_program: Program,
}

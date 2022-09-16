use crate::{
    ast::Quant,
    prog::{Instr, InstrPtr},
    Ast, Prog,
};

pub(crate) fn compile(ast: &Ast) -> Prog {
    let mut instrs = Vec::new();
    let mut context = CompileContext {
        instrs: &mut instrs,
    };
    let frag = context.compile(ast);
    let instr = context.emit_instr(Instr::Match);
    context.fill_holes(frag.ends, instr);
    Prog {
        instrs: instrs.into_iter().map(|instr| instr.into_instr()).collect(),
        start: frag.start,
    }
}

#[derive(Debug)]
struct CompileContext<'a> {
    instrs: &'a mut Vec<InstrMaybeHole>,
}

impl<'a> CompileContext<'a> {
    fn compile(&mut self, ast: &Ast) -> Frag {
        match *ast {
            Ast::Empty => self.compile_empty(),
            Ast::Char(ch) => self.compile_char(ch),
            Ast::Rep(ref ast, quant, non_greedy) => {
                let frag = self.compile(ast);
                self.compile_rep(frag, quant, non_greedy)
            }
            Ast::Cat(ref ast_0, ref ast_1) => {
                let frag_0 = self.compile(ast_0);
                let frag_1 = self.compile(ast_1);
                self.compile_cat(frag_0, frag_1)
            }
            Ast::Alt(ref ast_0, ref ast_1) => {
                let frag_0 = self.compile(ast_0);
                let frag_1 = self.compile(ast_1);
                self.compile_alt(frag_0, frag_1)
            }
        }
    }

    fn compile_empty(&mut self) -> Frag {
        let instr = self.emit_instr_hole(InstrHole::Empty);
        Frag {
            start: instr,
            ends: vec![instr],
        }
    }

    fn compile_char(&mut self, ch: char) -> Frag {
        let instr = self.emit_instr_hole(InstrHole::Char(ch));
        Frag {
            start: instr,
            ends: vec![instr],
        }
    }

    fn compile_rep(&mut self, mut frag: Frag, quant: Quant, non_greedy: bool) -> Frag {
        match quant {
            Quant::Quest => {
                let instr = self.emit_instr_hole(if non_greedy {
                    InstrHole::Split1(frag.start)
                } else {
                    InstrHole::Split0(frag.start)
                });
                frag.ends.push(instr);
                Frag {
                    start: instr,
                    ends: frag.ends,
                }
            }
            Quant::Star => {
                let instr = self.emit_instr_hole(if non_greedy {
                    InstrHole::Split1(frag.start)
                } else {
                    InstrHole::Split0(frag.start)
                });
                self.fill_holes(frag.ends, instr);
                Frag {
                    start: instr,
                    ends: vec![instr],
                }
            }
            Quant::Plus => {
                let instr = self.emit_instr_hole(if non_greedy {
                    InstrHole::Split1(frag.start)
                } else {
                    InstrHole::Split0(frag.start)
                });
                self.fill_holes(frag.ends, instr);
                Frag {
                    start: frag.start,
                    ends: vec![instr],
                }
            }
        }
    }

    fn compile_cat(&mut self, frag_0: Frag, frag_1: Frag) -> Frag {
        self.fill_holes(frag_0.ends, frag_1.start);
        Frag {
            start: frag_0.start,
            ends: frag_1.ends,
        }
    }

    fn compile_alt(&mut self, mut frag_0: Frag, frag_1: Frag) -> Frag {
        let instr = self.emit_instr(Instr::Split(frag_0.start, frag_1.start));
        frag_0.ends.extend(frag_1.ends);
        Frag {
            start: instr,
            ends: frag_0.ends,
        }
    }

    fn fill_holes(&mut self, instrs: Vec<InstrPtr>, out: InstrPtr) {
        for instr in instrs {
            self.instrs[instr].fill_hole(out);
        }
    }

    fn emit_instr(&mut self, instr: Instr) -> InstrPtr {
        let instr_ptr = self.instrs.len();
        self.instrs.push(InstrMaybeHole::Instr(instr));
        instr_ptr
    }

    fn emit_instr_hole(&mut self, instr: InstrHole) -> InstrPtr {
        let instr_ptr = self.instrs.len();
        self.instrs.push(InstrMaybeHole::InstrHole(instr));
        instr_ptr
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Frag {
    start: InstrPtr,
    ends: Vec<InstrPtr>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum InstrMaybeHole {
    Instr(Instr),
    InstrHole(InstrHole),
}

impl InstrMaybeHole {
    fn into_instr(self) -> Instr {
        match self {
            Self::Instr(instr) => instr,
            _ => panic!(),
        }
    }

    fn fill_hole(&mut self, out: InstrPtr) {
        match self {
            Self::InstrHole(hole) => *self = Self::Instr(hole.fill_hole(out)),
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum InstrHole {
    Empty,
    Char(char),
    Split0(InstrPtr),
    Split1(InstrPtr),
}

impl InstrHole {
    fn fill_hole(self, out: InstrPtr) -> Instr {
        match self {
            InstrHole::Empty => Instr::Empty(out),
            InstrHole::Char(ch) => Instr::Char(ch, out),
            InstrHole::Split0(out_0) => Instr::Split(out_0, out),
            InstrHole::Split1(out_1) => Instr::Split(out, out_1),
        }
    }
}

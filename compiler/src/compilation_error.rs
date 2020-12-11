use crate::Line;
use mexprp::EvalError;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum CompilationError<'a> {
    MathEval(Line<'a>, EvalError),
    NegativeRegisters {
        line: Line<'a>,
        expr: &'a str,
        value: isize,
    },
    ResgrNoOperand(Line<'a>),
    NoOperand {
        line: Line<'a>,
        opcode: &'a str,
    },
    Incomprehensible(Line<'a>, EvalError),
    NoCompilation,
}

impl CompilationError<'_> {
    pub fn get_line(&self) -> Option<&Line> {
        match self {
            CompilationError::MathEval(l, ..) => Some(l),
            CompilationError::NegativeRegisters { line, .. } => Some(line),
            CompilationError::ResgrNoOperand(line) => Some(line),
            CompilationError::NoOperand { line, .. } => Some(line),
            CompilationError::Incomprehensible(line, ..) => Some(line),
            CompilationError::NoCompilation => None
        }
    }
}

impl std::error::Error for CompilationError<'_> {}

impl std::fmt::Display for CompilationError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilationError::MathEval(_, e) => std::fmt::Display::fmt(e, f),
            CompilationError::NegativeRegisters { .. } => write!(f, "RESGR expects a non-negative operand"),
            CompilationError::ResgrNoOperand(_) => write!(f, "RESGR must have one operand"),
            CompilationError::NoCompilation => write!(f, "No compilation happened"),
            CompilationError::NoOperand { opcode, .. } => write!(f, "Instruction `{}` expects an operand, but you provided none", opcode),
            CompilationError::Incomprehensible(..) => write!(f, "Not a valid instruction or integer expression"),
        }
    }
}
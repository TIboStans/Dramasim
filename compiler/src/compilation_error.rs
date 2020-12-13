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
    NoOperand {
        line: Line<'a>,
        opcode: &'a str,
    },
    Incomprehensible(Line<'a>, EvalError),
    NotARegister {
        line: Line<'a>,
        malformed_operand: &'a str
    },
    UnexpectedInterpretation(Line<'a>, String),
    NoCompilation,
}

impl CompilationError<'_> {
    pub fn get_line(&self) -> Option<&Line> {
        match self {
            CompilationError::MathEval(l, ..) => Some(l),
            CompilationError::NegativeRegisters { line, .. } => Some(line),
            CompilationError::NoOperand { line, .. } => Some(line),
            CompilationError::Incomprehensible(line, ..) => Some(line),
            CompilationError::NotARegister { line, .. } => Some(line),
            CompilationError::UnexpectedInterpretation(line, ..) => Some(line),
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
            CompilationError::NoCompilation => write!(f, "No compilation happened"),
            CompilationError::NoOperand { opcode, .. } => write!(f, "Instruction `{}` expects an operand, but you provided none", opcode),
            CompilationError::NotARegister { malformed_operand, .. } => write!(f, "`{}` is not in the form of Rx, where 0 <= x <= 9.", malformed_operand),
            CompilationError::UnexpectedInterpretation(_, op) => write!(f, "Instruction `{}` does not expect an interpretation", op),
            CompilationError::Incomprehensible(..) => write!(f, "Not a valid instruction or integer expression"),
        }
    }
}
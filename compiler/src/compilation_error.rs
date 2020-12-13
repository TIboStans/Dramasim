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
        malformed_operand: &'a str,
    },
    UnexpectedInterpretation(Line<'a>, String),
    UnsupportedInterpretation(Line<'a>, String, Vec<char>),
    TooLongInterpretation(Line<'a>, String),
    NoSecondOperand(Line<'a>, String),
    RegRegUnsupported(Line<'a>, String),
    RegRegInterpretation(Line<'a>, String),
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
            CompilationError::UnexpectedInterpretation(line, _) => Some(line),
            CompilationError::UnsupportedInterpretation(line, ..) => Some(line),
            CompilationError::TooLongInterpretation(line, ..) => Some(line),
            CompilationError::NoSecondOperand(line, ..) => Some(line),
            CompilationError::RegRegUnsupported(line, ..) => Some(line),
            CompilationError::RegRegInterpretation(line, ..) => Some(line),
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
            CompilationError::UnexpectedInterpretation(_, opcode) => write!(f, "Instruction `{}` does not expect an interpretation", opcode),
            CompilationError::UnsupportedInterpretation(_, op, provides) => write!(f, "Instruction `{}` supports only interpretations {:?}", op, provides),
            CompilationError::TooLongInterpretation(_, int) => write!(f, "Interpretations consist of exactly one character, thus `{}` is invalid.", int),
            CompilationError::NoSecondOperand(_, opcode) => write!(f, "Instruction `{}` expects two operands, but you provided only one", opcode),
            CompilationError::RegRegUnsupported(_, opcode) => write!(f, "Register-register operations are not supported for `{}`", opcode),
            CompilationError::RegRegInterpretation(_, opcode) => write!(f, "Register-register operations using `{}` don't support interpretations", opcode),
            CompilationError::Incomprehensible(..) => write!(f, "Not a valid instruction or integer expression"),
        }
    }
}
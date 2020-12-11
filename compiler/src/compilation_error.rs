use crate::Line;
use mexprp::EvalError;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum CompilationError<'a> {
    MathEvalError(Line<'a>, EvalError),
    NegativeResgrError {
        line: Line<'a>,
        expr: &'a str,
        value: isize,
    },
    ResgrNoOperandError(Line<'a>),
}

impl std::error::Error for CompilationError<'_> {}

impl std::fmt::Display for CompilationError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilationError::MathEvalError(_, e) => std::fmt::Display::fmt(e, f),
            CompilationError::NegativeResgrError { .. } => write!(f, "RESGR expects a non-negative operand"),
            CompilationError::ResgrNoOperandError(_) => write!(f, "RESGR must have one operand")
        }
    }
}
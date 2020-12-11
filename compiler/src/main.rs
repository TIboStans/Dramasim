#![feature(try_trait)]
#![feature(or_patterns)]
#![feature(arbitrary_enum_discriminant)]

use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::Debug;
use mexprp::{Answer, EvalError};
use crate::compilation_error::*;

mod compilation_error;

#[derive(Debug, Clone)]
pub struct Line<'a> {
    address: usize,
    line_number: usize,
    line: &'a str,
}

fn main() {
    const INPUT: &str = include_str!("test_resgr");
    println!("{}", compile(INPUT).unwrap_err());
}

fn compile(source_code: &str) -> Result<&[(usize, usize)], CompilationError> {
    let filter = as_filtered_lines(source_code);
    let expanded = as_numbered_lines(&filter)?;
    let labels = map_labels(&expanded);
    for line in expanded {
        println!("{:?}", line);
    }
    println!("{:#?}", labels);

    Ok(&[(0, 0)])
}

/// Returns a vec of lines with comments, trailing whitespace, and leading whitespace removed.
/// Takes everything until EOF or EINDPR
fn as_filtered_lines(input: &str) -> Vec<&str> {
    let mut lines = Vec::new();
    for line in input.lines() {
        // remove comments
        let without_comment = line.splitn(2, '|').next().unwrap();
        // trim whitespace
        let x = without_comment.trim();

        if x == "EINDPR" {
            break;
        }

        if !x.is_empty() {
            lines.push(x);
        }
    }

    lines
}

/// Parses filtered code, expanding RESGR where needed.
fn as_numbered_lines<'a>(input: &Vec<&'a str>) -> Result<Vec<Line<'a>>, CompilationError<'a>> {
    let mut address_counter = 0usize;
    let mut lines = Vec::new();
    for line_number in 0..input.len() {
        let line = input[line_number];

        let line_struct = Line {
            address: address_counter,
            line_number: line_number + 1, // line numbers start at 1
            line,
        };

        let (_, line_without_label) = omit_label(line);
        let (insn, operand) = trimmed_split_space(line_without_label);
        if insn == "RESGR" {
            if let Some(operand) = operand {
                let value = calculate_expression(operand)
                    .map_err(|e| CompilationError::MathEvalError(line_struct.clone(), e))?;
                let value: usize = usize::try_from(value)
                    .map_err(|_| CompilationError::NegativeResgrError { line: line_struct, expr: operand, value })?;
                address_counter += value;
            } else {
                return Err(CompilationError::ResgrNoOperandError(line_struct))
            }
        } else {
            lines.push(line_struct);
            address_counter += 1;
        }
    }

    Ok(lines)
}

/// Inspects numbered lines, returning a map of all labels and their corresponding memory address.
fn map_labels<'a>(numbered_lines: &Vec<Line<'a>>) -> HashMap<&'a str, usize> {
    numbered_lines.iter()
        .filter_map(|line| {
            let (label, _) = omit_label(line.line);

            label.map(|label| (label, line.line_number))
        }).collect()
}

/// Removes the label from a string without any other operations such as trimming. Label may be `None` if there is none present.
fn omit_label(line: &str) -> (Option<&str>, &str) {
    // TODO: omit string literals from labels:
    /*
    HIA.a R0, 4
    DRS
    STP
    "look: I can confuse the compiler!"   | this is not a label
    */

    let mut split = line.rsplitn(2, ':');
    let rhs = split.next().unwrap();
    let lhs = split.next();
    (lhs, rhs)
}

fn trimmed_split_space(line: &str) -> (&str, Option<&str>) {
    let mut splitn = line.trim().splitn(2, ' ');
    (splitn.next().unwrap().trim(), splitn.next().map(|s| s.trim()))
}

/// Calculate an integer expression.
/// If multiple answers are possible, arbitrarily return the first one found.
/// Answers are calculated in f64 and converted to isize.
fn calculate_expression(expr: &str) -> Result<isize, EvalError> {
    match mexprp::eval::<f64>(expr) {
        Ok(Answer::Single(answer)) => Ok(answer as isize),
        Ok(Answer::Multiple(v)) => Ok(*v.first().unwrap() as isize),
        Err(e) => Err(e)
    }
}

#[inline]
#[allow(unused)]
fn insn(op: isize, m1: isize, m2: isize, acc: isize, ind: isize, operand: isize) -> isize {
    let mut o = operand % 10_000;
    if o < 0 { o += 10_000 }
    return op * 1_00_0_0_0000 + m1 * 10_0_0_0000 + m2 * 10_0_0000 + acc * 1_0_0000 + ind * 1_0000 + o;
}
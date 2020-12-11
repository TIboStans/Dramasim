#![feature(try_trait)]
#![feature(or_patterns)]
#![feature(arbitrary_enum_discriminant)]
#![feature(pattern)]

use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::Debug;
use mexprp::{Answer, EvalError};
use crate::compilation_error::*;
use std::str::pattern::Pattern;

mod compilation_error;

#[derive(Debug, Clone)]
pub struct Line<'a> {
    address: usize,
    line_number: usize,
    line: &'a str,
}

fn main() {
    const INPUT: &str = include_str!("test_resgr");
    match compile(INPUT) {
        Ok(c) => {
            println!("Compilation successful!");
            println!();
            for (address, value) in c.iter() {
                println!("{:04}: {:010}", address, value)
            }
        }
        Err(e) => {
            println!("Compilation error:");
            if let Some(line) = e.get_line() {
                let line_str = line.line;
                println!("\nOn line {} \t[address {}]", line.line_number, line.address);
                println!("\t{}", line_str);
                println!("\t{} {}", (0..line_str.len()).map(|_| '^').collect::<String>(), e)
            } else {
                println!("{}", e);
            }
        }
    }
}

fn compile(source_code: &str) -> Result<Box<[(usize, isize)]>, CompilationError> {
    let filter = as_filtered_lines(source_code);
    let expanded = as_numbered_lines(&filter)?;
    let _labels = map_labels(&expanded);
    let numerical = to_numerical_representation(expanded)?;

    Ok(numerical.into_boxed_slice())
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
        let (insn, operand) = trimmed_split(line_without_label, ' ');
        if insn == "RESGR" {
            if let Some(operand) = operand {
                let value = calculate_expression(operand)
                    .map_err(|e| CompilationError::MathEval(line_struct.clone(), e))?;
                let value: usize = usize::try_from(value)
                    .map_err(|_| CompilationError::NegativeRegisters { line: line_struct, expr: operand, value })?;
                address_counter += value;
            } else {
                return Err(CompilationError::NoOperand { line: line_struct, opcode: "RESGR" });
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

fn to_numerical_representation(lines: Vec<Line>) -> Result<Vec<(usize, isize)>, CompilationError> {
    let mut out = Vec::new();
    for line in lines {
        let str = line.line;

        let (_, line_without_label) = omit_label(str);
        let line_without_label = line_without_label.trim();
        let numerical = match insn_to_numerical(line_without_label, &line) {
            Ok(insn) => insn,
            Err(CompilationError::NoCompilation) => calculate_expression(line_without_label)
                .map_err(|e| CompilationError::Incomprehensible(line.clone(), e))?,
            e => e?
        };

        out.push((line.address, numerical))
    }

    Ok(out)
}

fn insn_to_numerical<'a>(insn: &'a str, line: &Line<'a>) -> Result<isize, CompilationError<'a>> {
    let (original_opcode, rhs) = trimmed_split(insn, ' ');
    let opcode = original_opcode.to_uppercase();
    let opcode = opcode.as_str();

    match opcode {
        "KTG" => return Ok(self::insn(42, 0, 0, 0, 0, 0)),
        "LEZ" => return Ok(self::insn(71, 0, 0, 0, 0, 0)),
        "DRU" => return Ok(self::insn(72, 0, 0, 0, 0, 0)),
        "NWL" => return Ok(self::insn(73, 0, 0, 0, 0, 0)),
        "DRS" => return Ok(self::insn(74, 0, 0, 0, 0, 0)),
        "STP" => return Ok(self::insn(99, 0, 0, 0, 0, 0)),
        "NOP" => return Ok(0),
        _ => {}
    }

    let (opcode, _int) = trimmed_split(opcode, '.');

    const LEFTOVER_INSNS: [&str; 13] = ["HIA", "BIG", "OPT", "AFT", "VER", "DEL", "MOD", "VGL", "SPR", "VSP", "SBR", "BST", "HST"];
    if !LEFTOVER_INSNS.contains(&opcode) {
        return Err(CompilationError::NoCompilation)
    }

    // All instructions without operands have been parsed at this point,
    // and any invalid instructions have already thrown a NoCompilation error.
    // - let's toss an error if there is no right hand side at this point.
    let _rhs = match rhs {
        Some(s) => s,
        None => return Err(CompilationError::NoOperand { line: line.clone(), opcode: original_opcode })
    };

    Err(CompilationError::NoCompilation)
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

fn trimmed_split<'a, P: Pattern<'a>>(string: &'a str, pattern: P) -> (&'a str, Option<&'a str>) {
    let mut splitn = string.trim().splitn(2, pattern);
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
fn insn(op: isize, m1: isize, m2: isize, acc: isize, ind: isize, operand: isize) -> isize {
    let mut o = operand % 10_000;
    if o < 0 { o += 10_000 }
    return op * 1_00_0_0_0000 + m1 * 10_0_0_0000 + m2 * 10_0_0000 + acc * 1_0_0000 + ind * 1_0000 + o;
}
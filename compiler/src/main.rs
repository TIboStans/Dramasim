#![feature(try_trait)]
#![feature(or_patterns)]
#![feature(arbitrary_enum_discriminant)]
#![feature(pattern)]

use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::Debug;
use mexprp::{Answer, EvalError, Context, Term};
use crate::compilation_error::*;
use std::str::pattern::Pattern;
use std::ops::Try;

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
                println!("\t{} {}", (0..line_str.len())
                    .map(|i| if &line_str[i..=i] == "\t" { '\t' } else { '^' })
                    .collect::<String>(), e)
            } else {
                println!("{}", e);
            }
        }
    }
}

fn compile(source_code: &str) -> Result<Box<[(usize, isize)]>, CompilationError> {
    let filter = as_filtered_lines(source_code);
    let expanded = as_numbered_lines(&filter)?;
    let labels = map_labels(&expanded);
    let evaluation_context = {
        let mut context = Context::new();
        for (key, value) in labels {
            context.vars.insert(key.to_string(), Term::Num(Answer::Single(value as f64)));
        }
        context
    };
    let numerical = to_numerical_representation(expanded, evaluation_context)?;

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
    let empty_context = Context::new();
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
                let value = calculate_expression(operand, &empty_context)
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

            label.map(|label| (label, line.address))
        }).collect()
}

fn to_numerical_representation(lines: Vec<Line>, evaluation_context: Context<f64>) -> Result<Vec<(usize, isize)>, CompilationError> {
    let mut out = Vec::new();
    for line in lines {
        let str = line.line;

        let (_, line_without_label) = omit_label(str);
        let line_without_label = line_without_label.trim();
        let numerical = match insn_to_numerical(line_without_label, &line, &evaluation_context) {
            Ok(insn) => insn,
            Err(CompilationError::NoCompilation) => calculate_expression(line_without_label, &evaluation_context)
                .map_err(|e| CompilationError::Incomprehensible(line.clone(), e))?,
            e => e?
        };

        out.push((line.address, numerical))
    }

    Ok(out)
}

fn insn_to_numerical<'a>(insn: &'a str, line: &Line<'a>, evaluation_context: &Context<f64>) -> Result<isize, CompilationError<'a>> {
    let (original_opcode, rhs) = trimmed_split(insn, ' ');
    let opcode = original_opcode.to_uppercase();
    let opcode = opcode.as_str();

    if let Some(insn) = parse_no_operand(opcode) {
        return Ok(insn)
    }

    let (opcode, _int) = trimmed_split(opcode, '.');

    const LEFTOVER_INSNS: [&str; 13] = ["HIA", "BIG", "OPT", "AFT", "VER", "DEL", "MOD", "VGL", "SPR", "VSP", "SBR", "BST", "HST"];
    if !LEFTOVER_INSNS.contains(&opcode) {
        return Err(CompilationError::NoCompilation)
    }

    // All instructions without operands have been parsed at this point,
    // and any invalid instructions have already thrown a NoCompilation error.
    // - let's toss an error if there is no right hand side at this point.
    let rhs = match rhs {
        Some(s) => s,
        None => return Err(CompilationError::NoOperand {
            line: line.clone(),
            opcode: original_opcode
        })
    };

    fn operand_to_reg(op: &str) -> Option<usize> {
        if op.len() != 2 || &op[0..1] != "R" {
            return None
        }

        let r = &op[1..2];
        let r = r.parse().ok()?;
        // It is not necessary to check that 0 <= R <= 9,
        // as all of those values exclusively fit the condition that op.len() == 2.
        Some(r)
    }

    // Single-operand instructions:
    match opcode {
        "HST" => {
            // HST becomes HIA <reg>, 0(R8+)
            let r = operand_to_reg(rhs).into_result().map_err(|_| CompilationError::NotARegister {
                line: line.clone(),
                malformed_operand: rhs
            })?;
            return Ok(self::insn(11 /*HIA*/, 1 /*value*/, 4 /*indexation post-inc*/, r as isize, 8, 0));
        },
        "BST" => {
            // BST becomes BIG <reg>, 0(-R8)
            let r = operand_to_reg(rhs).into_result().map_err(|_| CompilationError::NotARegister {
                line: line.clone(),
                malformed_operand: rhs
            })?;
            return Ok(self::insn(12 /*BIG*/, 1 /*value*/, 5 /*indexation pre-dec*/, r as isize, 8, 0));
        },
        "SBR" => {
            let address = calculate_expression(rhs, evaluation_context)
                .map_err(|e| CompilationError::MathEval(line.clone(), e))?;
            // TODO: .i
            return Ok(self::insn(41 /*SBR*/, 9, 9, 9, 9, address))
        },
        _ => {}
    }

    Err(CompilationError::NoCompilation)
}

fn parse_no_operand(opcode: &str) -> Option<isize> {
    match opcode {
        "KTG" => return Some(insn(42, 0, 0, 0, 0, 0)),
        "LEZ" => return Some(insn(71, 0, 0, 0, 0, 0)),
        "DRU" => return Some(insn(72, 0, 0, 0, 0, 0)),
        "NWL" => return Some(insn(73, 0, 0, 0, 0, 0)),
        "DRS" => return Some(insn(74, 0, 0, 0, 0, 0)),
        "STP" => return Some(insn(99, 0, 0, 0, 0, 0)),
        "NOP" => return Some(0), // TODO: HIA R0, R0
        _ => None
    }
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
fn calculate_expression(expr: &str, ctx: &Context<f64>) -> Result<isize, EvalError> {
    match mexprp::eval_ctx::<f64>(expr, ctx) {
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
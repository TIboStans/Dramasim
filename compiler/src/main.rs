#![feature(try_trait)]
#![feature(or_patterns)]
#![feature(arbitrary_enum_discriminant)]

#[macro_use]
extern crate scan_fmt;

use std::collections::HashMap;
use std::convert::{TryInto, TryFrom};
use std::num::ParseIntError;

type NumberedLine<'a> = (usize, &'a str);

fn main() {
    const INPUT: &str = include_str!("test");
    let filter = to_filtered_lines(INPUT);
    let expanded = to_numbered_lines(&filter);
    for line in expanded {
        println!("{:?}", line);
    }
}

/// Returns a vec of lines with comments, trailing whitespace, and leading whitespace removed.
fn to_filtered_lines(input: &str) -> Vec<&str> {
    input.lines().filter_map(|line| {
        // remove comments
        let without_comment = line.splitn(2, '|').next().unwrap();
        // trim whitespace
        let x = without_comment.trim();

        if x.is_empty() {
            None
        } else {
            Some(x)
        }
    }).collect()
}

/// Parses filtered code, expanding RESGR where needed.
fn to_numbered_lines<'a>(input: &Vec<&'a str>) -> Vec<NumberedLine<'a>> {
    let mut counter = 0usize;
    let mut lines = Vec::new();
    for line in input {
        let (_, mut line_without_label) = omit_label(line);
        let (insn, operand) = trimmed_split_space(line_without_label);
        if insn == "RESGR" {
            if let Some(operand) = operand {
                let operand = calculate_expression(operand).unwrap(); // TODO: don't unwrap, expression must be valid
                let operand: usize = operand.try_into().unwrap(); // TODO: don't unwrap: operand must be in bounds of an usize
                counter += operand;
            } else {
                panic!("RESGR without operand"); // TODO: don't
            }
        }

        lines.push((counter, *line));
        counter += 1;
    }

    lines
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

fn calculate_expression(expr: &str) -> Result<isize, ParseIntError> {
    // TODO: actually calculate expressions
    expr.parse()
}

#[inline]
fn insn(op: isize, m1: isize, m2: isize, acc: isize, ind: isize, operand: isize) -> isize {
    let mut o = operand % 10_000;
    if o < 0 { o += 10_000 }
    return op * 1_00_0_0_0000 + m1 * 10_0_0_0000 + m2 * 10_0_0000 + acc * 1_0_0000 + ind * 1_0000 + o;
}
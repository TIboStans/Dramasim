#![feature(try_trait)]
#![feature(or_patterns)]
#![feature(arbitrary_enum_discriminant)]

use std::collections::HashMap;
use std::convert::TryInto;
use std::num::ParseIntError;

#[derive(Debug)]
struct Line<'a> {
    address: usize,
    line_number: usize,
    line: &'a str
}

fn main() {
    const INPUT: &str = include_str!("test_resgr");
    let filter = as_filtered_lines(INPUT);
    let expanded = as_numbered_lines(&filter);
    let labels = map_labels(&expanded);
    for line in expanded {
        println!("{:?}", line);
    }
    println!("{:#?}", labels);
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
fn as_numbered_lines<'a>(input: &Vec<&'a str>) -> Vec<Line<'a>> {
    let mut address_counter = 0usize;
    let mut lines = Vec::new();
    for line_number in 0..input.len() {
        let line = input[line_number];

        let (_, line_without_label) = omit_label(line);
        let (insn, operand) = trimmed_split_space(line_without_label);
        if insn == "RESGR" {
            if let Some(operand) = operand {
                let operand = calculate_expression(operand).unwrap(); // TODO: don't unwrap, expression must be valid
                let operand: usize = operand.try_into().unwrap(); // TODO: don't unwrap: operand must be in bounds of an usize
                address_counter += operand;
            } else {
                panic!("RESGR without operand"); // TODO: don't
            }
        } else {
            lines.push(Line {
                address: address_counter,
                line_number: line_number + 1, // line numbers start at 1
                line
            });
            address_counter += 1;
        }
    }

    lines
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
#![feature(try_trait)]
#![feature(or_patterns)]
#![feature(arbitrary_enum_discriminant)]

use std::collections::HashMap;
use std::convert::TryInto;
use std::num::ParseIntError;

type NumberedLine<'a> = (usize, &'a str);

fn main() {
    const INPUT: &str = include_str!("test_resgr");
    let filter = to_filtered_lines(INPUT);
    let expanded = to_numbered_lines(&filter);
    let labels = map_labels(&expanded);
    for line in expanded {
        println!("{:?}", line);
    }
    println!("{:#?}", labels);
}

/// Returns a vec of lines with comments, trailing whitespace, and leading whitespace removed.
/// Takes everything until EOF or EINDPR
fn to_filtered_lines(input: &str) -> Vec<&str> {
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
fn to_numbered_lines<'a>(input: &Vec<&'a str>) -> Vec<NumberedLine<'a>> {
    let mut counter = 0usize;
    let mut lines = Vec::new();
    for line in input {
        let (_, line_without_label) = omit_label(line);
        let (insn, operand) = trimmed_split_space(line_without_label);
        if insn == "RESGR" {
            if let Some(operand) = operand {
                let operand = calculate_expression(operand).unwrap(); // TODO: don't unwrap, expression must be valid
                let operand: usize = operand.try_into().unwrap(); // TODO: don't unwrap: operand must be in bounds of an usize
                counter += operand;
            } else {
                panic!("RESGR without operand"); // TODO: don't
            }
        } else {
            lines.push((counter, *line));
            counter += 1;
        }
    }

    lines
}

/// Inspects numbered lines, returning a map of all labels and their corresponding memory address.
fn map_labels<'a>(numbered_lines: &Vec<NumberedLine<'a>>) -> HashMap<&'a str, usize> {
    numbered_lines.iter()
        .filter_map(|(line_number, line)| {
            let (label, _) = omit_label(line);

            label.map(|label| (label, *line_number))
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
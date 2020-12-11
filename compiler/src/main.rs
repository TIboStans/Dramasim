#![feature(try_trait)]
#![feature(or_patterns)]
#![feature(arbitrary_enum_discriminant)]

#[macro_use]
extern crate scan_fmt;

use std::collections::HashMap;
use std::convert::{TryInto, TryFrom};

type NumberedLine<'a> = (usize, &'a str);

fn main() {
    const INPUT: &str = include_str!("test");
    for line in to_filtered_lines(INPUT) {
        println!("{}", line);
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

#[inline]
fn insn(op: isize, m1: isize, m2: isize, acc: isize, ind: isize, operand: isize) -> isize {
    let mut o = operand % 10_000;
    if o < 0 { o += 10_000 }
    return op * 1_00_0_0_0000 + m1 * 10_0_0_0000 + m2 * 10_0_0000 + acc * 1_0_0000 + ind * 1_0000 + o;
}
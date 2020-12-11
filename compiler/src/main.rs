#![feature(try_trait)]
#![feature(or_patterns)]
#![feature(arbitrary_enum_discriminant)]

#[macro_use]
extern crate scan_fmt;

use std::collections::HashMap;
use std::ops::Try;
use std::option::NoneError;
use std::convert::{TryInto, TryFrom};

fn main() {
    if let Err(error) = compile(include_str!("test2")) {
        eprintln!("{}", error);
    }
}

fn compile(source_code: &str) -> Result<Vec<isize>, String> {
    let mut label_map: HashMap<String, usize> = HashMap::new();

    let code: Vec<String> = {
        let mut counter = 0;
        source_code.lines().filter_map(|line| {
            let line = line.trim().replace('\t', " ");
            if line.starts_with('|') || line.is_empty() {
                None
            } else {
                // remove comments
                let line = match line.find('|') {
                    Some(index) => line[0..index].to_string(),
                    None => line
                };

                counter += 1;

                let r = scan_fmt!(line.as_str(), "{}: {}", String, String);

                let submit = if let Ok((label, code)) = r {
                    let is_empty_label_line = code.trim().is_empty();
                    if is_empty_label_line {
                        counter -= 1;
                    }
                    label_map.insert(label, counter);

                    !is_empty_label_line
                } else {
                    true
                };

                if submit { Some(line) } else { None }
            }
        }).collect()
    };

    let mut output: Vec<isize> = Vec::new();

    for line_number in 0..code.len() {
        let line = &code[line_number];

        // remove `label: ...`
        let line = match line.find(':') {
            Some(colon_index) => line[colon_index + 1..].to_string(),
            None => line.to_owned()
        };
        let line = line.trim();

        let mut splitn = line.splitn(2, ' ');
        let (op, rhs) = (splitn.next().into_result().map_err(|_| {
            pretty_error("Empty instruction", line, line_number)
        })?, splitn.next());

        let op: Opcode = op.try_into().map_err(|_| {
            pretty_error(format!("Nonexistent instruction {}", op).as_str(), line, line_number)
        })?;

        let single_reg_operand = |rhs: Option<&str>| -> Result<isize, String> {
            let s = rhs.into_result().map_err(|_| pretty_error("Missing register operand", line, line_number))?;
            Ok(scan_fmt!(s, "R{}", u8).map_err(|_| pretty_error(
                "Register number must be numerical", line, line_number,
            ))? as isize)
        };

        let address_operand = |rhs: Option<&str>| -> Result<(isize, isize, isize), String> {
            let rhs = rhs.into_result().map_err(|_| pretty_error("Missing address operand", line, line_number))?;
            // address
            // getal
            // address(R0)
            // getal(R0-)
            // address(+R0)

            let mut splitn = rhs.splitn(2, "(");
            let (address, register) = (
                splitn.next().into_result().map_err(|_| pretty_error("Missing address operand", line, line_number))?,
                splitn.next()
            );

            let address = match scan_fmt!(address, "{}", usize) {
                Ok(a) => a,
                Err(_) => *label_map.get(address).into_result().map_err(|_|
                    pretty_error("Address is not a number, nor a symbolic label", line, line_number)
                )?
            } as isize;

            match register {
                None => Ok((address, 0isize, 1isize)),
                Some(rhs) => {
                    let rhs = rhs.trim_end_matches(")");


                    Ok((0isize, 0isize, 0isize))
                }
            }
        };

        let reg_and_cmpx_operand = |rhs: Option<&str>| -> Result<(isize, isize), String> {
            let rhs = rhs.into_result().map_err(|_| pretty_error("Missing operands", line, line_number))?;
            let mut splitn = rhs.splitn(2, ", ");
            let (reg_op, cmpx_op) = (
                splitn.next().into_result().map_err(|_| pretty_error("Missing left operand", line, line_number))?,
                splitn.next().into_result().map_err(|_| pretty_error("Missing right operand", line, line_number))?
            );
            let reg_op = single_reg_operand(Some(reg_op))?;

            //

            // Rx = 0(Rx)

            // HIA R0, R1
            // -> HIA.w R0, 0(R1)

            // HIA R0, 500
            // -> HIA.d R0, 500

            // getal(Rx)
            // expressie
            // expressie(Rx)

            /*
            Rx
            (string | number)(\(([+-]?Rx|Rx[+-])\))?
             */

            Ok((reg_op, 0isize))
        };

        let numerical_repr = match op {
            Opcode::HIA(m) => insn(11, m as isize, 0, single_reg_operand(rhs)?, 0, 0),
            Opcode::BIG(m) => insn(12, m as isize, 0, single_reg_operand(rhs)?, 0, 0),

            Opcode::OPT(m) => insn(21, m as isize, 0, single_reg_operand(rhs)?, 0, 0),
            Opcode::AFT(m) => insn(22, m as isize, 0, single_reg_operand(rhs)?, 0, 0),
            Opcode::VER(m) => insn(23, m as isize, 0, single_reg_operand(rhs)?, 0, 0),
            Opcode::DEL(m) => insn(24, m as isize, 0, single_reg_operand(rhs)?, 0, 0),
            Opcode::MOD(m) => insn(25, m as isize, 0, single_reg_operand(rhs)?, 0, 0),

            Opcode::VGL(m) => insn(31, m as isize, 0, single_reg_operand(rhs)?, 0, 0),

            Opcode::SPR(m) => insn(32, m as isize & 6, 0, single_reg_operand(rhs)?, 0, 0),
            Opcode::VSP(m) => insn(33, m as isize & 6, 0, single_reg_operand(rhs)?, 0, 0),

            Opcode::SBR(m) => insn(41, m as isize & 6, 0, 0, 0, 0),
            Opcode::KTG => insn(42, 0, 0, 0, 0, 0),

            Opcode::LEZ => insn(71, 0, 0, 0, 0, 0),
            Opcode::DRU => insn(72, 0, 0, 0, 0, 0),
            Opcode::NWL => insn(73, 0, 0, 0, 0, 0),
            Opcode::DRS => insn(74, 0, 0, 0, 0, 0),

            Opcode::STP => insn(99, 0, 0, 0, 0, 0),
            Opcode::BST => insn(12, 3, 5, single_reg_operand(rhs)?, 8, 0), // BIG <acc> 0(-R8) (BST R1 -> BIG R1 0(-R8))
            Opcode::HST => insn(11, 3, 4, single_reg_operand(rhs)?, 8, 0), // HIA <acc> 0(R8+)
        };

        output.push(numerical_repr);
    }

    println!("{:#?}\n{:#?}", output, label_map);

    Err("no compilation yet".to_string())
}

fn pretty_error(kind: &str, line: &str, line_number: usize) -> String {
    format!("error: {}\n\
        {} | {}",
            kind, line_number + 1, line)
}

#[repr(u8)]
enum Opcode {
    HIA(Interpretation),
    // .d .i
    BIG(Interpretation),
    OPT(Interpretation),
    AFT(Interpretation),
    VER(Interpretation),
    DEL(Interpretation),
    MOD(Interpretation),
    VGL(Interpretation),
    // .d .i
    SPR(Interpretation),
    // .d .i
    VSP(Interpretation),
    // .d .i
    SBR(Interpretation),
    KTG,
    LEZ,
    DRU,
    NWL,
    DRS,
    STP,
    BST,
    HST,
}

enum Interpretation {
    Direct = 3,
    Indirect = 4,
    Value = 1,
    Address = 2,
}

impl TryFrom<&str> for Opcode {
    type Error = NoneError;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let mut splitn = line.splitn(2, '.');
        let (insn, int) = (splitn.next().into_result().map_err(|_| NoneError)?, splitn.next());
        let int = match int {
            None => Interpretation::Direct,
            Some(i) => match i {
                "d" => Interpretation::Direct,
                "i" => Interpretation::Indirect,
                "w" => Interpretation::Value,
                "a" => Interpretation::Address,
                _ => return Err(NoneError)
            }
        };

        let insn = match insn.to_lowercase().as_str() {
            "hia" => Opcode::HIA(int),
            "big" => Opcode::BIG(int), // .d -> .a
            "opt" => Opcode::OPT(int),
            "aft" => Opcode::AFT(int),
            "ver" => Opcode::VER(int),
            "del" => Opcode::DEL(int),
            "mod" => Opcode::MOD(int),
            "vgl" => Opcode::VGL(int),
            "spr" => Opcode::SPR(int), // .d -> .a
            "vsp" => Opcode::VSP(int), // .d -> .a
            "sbr" => Opcode::SBR(int),
            "ktg" => Opcode::KTG,
            "lez" => Opcode::LEZ,
            "dru" => Opcode::DRU,
            "nwl" => Opcode::NWL,
            "drs" => Opcode::DRS,
            "stp" => Opcode::STP,
            "bst" => Opcode::BST,
            "hst" => Opcode::HST,
            _ => return Err(NoneError),
        };

        if let Err(err) = match insn {
            Opcode::BIG(Interpretation::Value | Interpretation::Address) => Err(NoneError),
            Opcode::SPR(Interpretation::Value | Interpretation::Address) => Err(NoneError),
            Opcode::VSP(Interpretation::Value | Interpretation::Address) => Err(NoneError),
            Opcode::SBR(Interpretation::Value | Interpretation::Address) => Err(NoneError),
            _ => Ok(())
        } {
            return Err(err);
        }

        Ok(insn)
    }
}

#[inline]
fn insn(op: isize, m1: isize, m2: isize, acc: isize, ind: isize, operand: isize) -> isize {
    let mut o = operand % 10_000;
    if o < 0 { o += 10_000 }
    return op * 1_00_0_0_0000 + m1 * 10_0_0_0000 + m2 * 10_0_0000 + acc * 1_0_0000 + ind * 1_0000 + o;
}
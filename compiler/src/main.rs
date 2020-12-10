#![feature(try_trait)]
#![feature(or_patterns)]

#[macro_use]
extern crate scan_fmt;

use std::collections::HashMap;
use std::ops::Try;
use std::str::FromStr;
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

    for line_number in 0..code.len() {
        let line = &code[line_number];

        // remove `label: ...`
        let line = match line.find(':') {
            Some(colon_index) => line[colon_index + 1..].to_string(),
            None => line.to_owned()
        };
        let line = line.trim();

        let mut splitn = line.splitn(2, ' ');
        let (insn, rhs) = (splitn.next().into_result().map_err(|_| {
            pretty_error("Empty instruction", line, line_number)
        })?, splitn.next());

        let insn: Opcode = insn.try_into().map_err(|_| {
            pretty_error(format!("Nonexistent instruction {}", insn).as_str(), line, line_number)
        })?;
    }

    println!("{:#?}", label_map);

    Err("no compilation yet".to_string())
}

fn pretty_error(kind: &str, line: &str, line_number: usize) -> String {
    format!("error: {}\n\
        {} | {}",
            kind, line_number + 1, line)
}

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
    Direct,
    Indirect,
    Value,
    Address,
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
            "big" => Opcode::BIG(int),
            "opt" => Opcode::OPT(int),
            "aft" => Opcode::AFT(int),
            "ver" => Opcode::VER(int),
            "del" => Opcode::DEL(int),
            "mod" => Opcode::MOD(int),
            "vgl" => Opcode::VGL(int),
            "spr" => Opcode::SPR(int),
            "vsp" => Opcode::VSP(int),
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
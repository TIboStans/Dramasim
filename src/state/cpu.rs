use crate::state::ram::{self, RAM};

macro_rules! num_range {
    // `()` indicates that the macro takes no argument.
    ($c:expr, $s:expr; $e:expr) => {
        {
            const DIV: usize = 10usize.pow(10 - $e);
            const MOD: usize = 10usize.pow($e - $s);
            ($c / DIV) % MOD
        }
    };
}

pub struct CPU {
    pub instruction_pointer: usize,
    pub instruction_register: isize,
    pub condition_code: ConditionCode,
    pub accumulators: [isize; 10],
    pub stopped: bool,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            instruction_pointer: 0,
            instruction_register: 0,
            condition_code: ConditionCode::Eql,
            accumulators: [0; 10],
            stopped: false,
        }
    }

    pub fn run(&mut self, mut ram: RAM) {
        while !self.stopped {
            // Get instructions

            let register = ram[self.instruction_pointer];
            self.instruction_register = register;
            self.instruction_pointer += 1;

            // Analyse Instruction
            // 01_23_4_5_6789
            // fc_mo_a_i_operand
            let command = if self.instruction_register < 0 {(self.instruction_register + 100_000_000_000) as usize} else {self.instruction_register as usize};

            let fc = num_range!(command, 0; 2);
            let modus = num_range!(command, 2; 4);
            let modus1 = modus / 10;
            let modus2 = modus % 10;
            let acc = num_range!(command, 4; 5);
            let ind = num_range!(command, 5; 6);
            let mut raw_operand: isize = num_range!(command, 6; 10) as isize; // TODO: why s this i8?
            if raw_operand >= 5_000 { raw_operand -= 10_000 }

            let raw_operand2: isize = match modus2 {
                1 => raw_operand,//nop
                2 => raw_operand + self.accumulators[ind],
                3 => {
                    self.accumulators[ind] += 1;
                    raw_operand + self.accumulators[ind]
                }
                4 => {
                    let p = self.accumulators[ind];
                    self.accumulators[ind] += 1;
                    raw_operand + p
                }
                5 => {
                    self.accumulators[ind] -= 1;
                    raw_operand + self.accumulators[ind]
                }
                6 => {
                    let p = self.accumulators[ind];
                    self.accumulators[ind] += 1;
                    raw_operand + p
                }
                _ => unreachable!()
            };

            let operand: isize = match modus1 {
                1 => raw_operand2,
                2 => ram::address(raw_operand2) as isize,
                3 => ram[raw_operand2],
                4 => ram[ram[raw_operand2]],
                _ => unreachable!() // TODO: don't panic
            };

            // instruction sets

            match fc.into() {
                Insn::HIA => {
                    self.accumulators[acc] = operand;
                    self.condition_code = ConditionCode::from_number(operand);
                }
                Insn::BIG => {
                    let p = self.accumulators[acc];
                    ram[operand] = p;
                    self.condition_code = ConditionCode::from_number(p);
                }
                Insn::OPT => {
                    self.accumulators[acc] += operand;
                    self.condition_code = ConditionCode::from_number(operand);
                }
                Insn::AFT => {
                    self.accumulators[acc] -= operand;
                    self.condition_code = ConditionCode::from_number(operand);
                }
                Insn::VER => {
                    self.accumulators[acc] *= operand;
                    self.condition_code = ConditionCode::from_number(operand);
                }
                Insn::DEL => {
                    self.accumulators[acc] /= operand;
                    self.condition_code = ConditionCode::from_number(operand);
                }
                Insn::MOD => {
                    self.accumulators[acc] %= operand;
                    self.condition_code = ConditionCode::from_number(operand);
                }
                Insn::VGL => {
                    self.condition_code = ConditionCode::from_number(operand);
                }
                Insn::SPR => {
                    self.instruction_pointer = ram::address(operand);
                }
                Insn::VSP => {
                    if match acc {
                        1 => self.condition_code == ConditionCode::Eql, /* NUL */
                        2 => self.condition_code != ConditionCode::Neg, /* NNEG */
                        3 => self.condition_code != ConditionCode::Pos, /* NPOS */
                        6 => self.condition_code == ConditionCode::Pos, /* POS */
                        7 => self.condition_code == ConditionCode::Neg, /* NEG */
                        8 => self.condition_code != ConditionCode::Eql, /* NNUL */
                        _ => unreachable!(),
                    } {
                        self.instruction_pointer = ram::address(operand);
                    }
                }
                Insn::SBR => {
                    self.accumulators[9] -= 1;
                    ram[self.accumulators[9]] = ram::expand(self.instruction_pointer);
                    self.instruction_pointer = ram::address(operand);
                }
                Insn::KTG => {
                    let ret = self.accumulators[9];
                    self.accumulators[9] += 1;
                    self.instruction_pointer = ram::address(ram[ret]);
                }
                Insn::LEZ => {}
                Insn::DRU => {
                    let variabele = self.accumulators[0];
                    println!("{:?}", variabele);
                    self.condition_code = ConditionCode::from_number(self.accumulators[0])
                }
                Insn::NWL => {
                    println!("\n")
                }
                Insn::DRS => {
                    println!("unimplemented");
                }
                Insn::STP => {
                    self.stop();
                }
            }
        }
    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }
}

enum Insn { HIA, BIG, OPT, AFT, VER, DEL, MOD, VGL, SPR, VSP, SBR, KTG, LEZ, DRU, NWL, DRS, STP }

impl From<usize> for Insn {
    fn from(n: usize) -> Self {
        match n {
            11 => Insn::HIA,
            12 => Insn::BIG,
            21 => Insn::OPT,
            22 => Insn::AFT,
            23 => Insn::VER,
            24 => Insn::DEL,
            25 => Insn::MOD,
            31 => Insn::VGL,
            32 => Insn::SPR,
            33 => Insn::VSP,
            41 => Insn::SBR,
            42 => Insn::KTG,
            71 => Insn::LEZ,
            72 => Insn::DRU,
            73 => Insn::NWL,
            74 => Insn::DRS,
            99 => Insn::STP,
            _ => unreachable!()
        }
    }
}

#[derive(PartialEq)]
pub enum ConditionCode {
    Pos,
    Eql,
    Neg,
}

impl ConditionCode {
    pub fn from_number(number: isize) -> Self {
        if number < 0 {
            ConditionCode::Neg
        } else if number == 0 {
            ConditionCode::Eql
        } else {
            ConditionCode::Pos
        }
    }
}
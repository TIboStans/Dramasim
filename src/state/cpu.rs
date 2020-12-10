pub struct CPU {
    pub instruction_counter: usize,
    pub instruction_register: usize,
    pub condition_code: ConditionCode,
    pub accumulators: [usize; 10],
    pub stopped: bool,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            instruction_counter: 0,
            instruction_register: 0,
            condition_code: ConditionCode::Eql,
            accumulators: [0; 10],
            stopped: false,
        }
    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }
}

pub enum ConditionCode {
    Pos = 1,
    Eql = 0,
    Neg = 2,
}

impl ConditionCode {
    pub fn from_discriminant(discriminant: usize) -> Self {
        discriminant as ConditionCode
    }

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
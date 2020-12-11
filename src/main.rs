use crate::state::cpu::{CPU, Insn};
use crate::state::ram::RAM;

mod state {
    pub mod cpu;
    pub mod ram;
}

mod ui {
    pub mod interface;
}

fn main() {
    ui::interface::gui();
    let mut cpu = CPU::new();
    let mut ram = RAM::new();

    ram[0usize] = insn(Insn::HIA, 1, 1, 1, 0, 0001);

    ram[1usize] = insn(Insn::DRU, 1, 1, 0, 0, 0000);
    ram[2usize] = insn(Insn::HIA, 1, 2, 2, 0, 0000);
    ram[3usize] = insn(Insn::OPT, 1, 2, 2, 1, 0000);
    ram[4usize] = insn(Insn::HIA, 1, 2, 0, 1, 0000);
    ram[5usize] = insn(Insn::HIA, 1, 2, 1, 2, 0000);

    ram[6usize] = insn(Insn::OPT, 1, 1, 3, 0, 0001);
    ram[7usize] = insn(Insn::VGL, 1, 1, 3, 0, 0050);
    ram[8usize] = insn(Insn::VSP, 1, 1, 7, 0, 0001);

    ram[9usize] = insn(Insn::STP, 1, 1, 0, 0, 0000);

    cpu.run(ram);
}

#[inline]
fn insn(op: Insn, m1: isize, m2: isize, acc: isize, ind: isize, operand: isize) -> isize {
    let mut o = operand % 10_000;
    if o < 0 { o += 10_000 }
    return (op as isize) * 1_00_0_0_0000 + m1 * 10_0_0_0000 + m2 * 10_0_0000 + acc * 1_0_0000 + ind * 1_0000 + o;
}

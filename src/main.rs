use crate::state::cpu::CPU;
use crate::state::ram::RAM;

mod state {
    pub mod cpu;
    pub mod ram;
}

fn main() {
    let mut cpu = CPU::new();
    let mut ram = RAM::new();
    ram[0usize] = 99_11_0_0_0000;

    cpu.run(ram);
}

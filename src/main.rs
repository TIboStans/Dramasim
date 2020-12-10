use crate::state::cpu::CPU;
use crate::state::ram::RAM;

mod state {
    pub mod cpu;
    pub mod ram;
}

fn main() {
    let mut cpu = CPU::new();
    let mut ram = RAM::new();
    let stop: isize = 99_11_0_0_0000 - 100_000_000_000;
    ram[0usize] = stop;

    cpu.run(ram);
}

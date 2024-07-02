use crate::cpu::CPU;

mod memory;
mod register;
mod opcode;
mod flag;
mod utils;
mod trap;
mod cpu;


fn main() {
    let mut cpu = CPU::new();
    cpu.run()
}

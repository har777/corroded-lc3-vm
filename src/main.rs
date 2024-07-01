use crate::flag::ConditionFlag;
use crate::memory::Memory;
use crate::opcode::Opcode;
use crate::register::{Register, Registers};
use crate::utils::sign_extend;

mod memory;
mod register;
mod opcode;
mod flag;
mod utils;


fn main() {
    let memory = Memory::new();
    let mut registers = Registers::new();
    registers.write(Register::COND, ConditionFlag::ZRO as u16);
    registers.write(Register::PC, 0x3000u16);

    loop {
        let instruction_memory_index = registers.read(Register::PC);
        registers.write(Register::PC, instruction_memory_index + 1);
        registers.update_flags(Register::R0);
        let instruction = memory.read(instruction_memory_index);
        let raw_opcode = instruction >> 12;
        let opcode = Opcode::from_u16(raw_opcode).unwrap();
        match opcode {
            Opcode::BR => {
                let pc_offset = sign_extend(instruction & 0x1FF, 9);
                let cond_flag = (instruction >> 9) & 0x7;
                if cond_flag & registers.read(Register::COND) == 1 {
                    registers.write(
                        Register::PC,
                        registers.read(Register::PC) + pc_offset
                    )
                }
            },
            Opcode::ADD => {
                let raw_dr = (instruction >> 9) & 0x7;
                let raw_sr1 = (instruction >> 6) & 0x7;
                let imm_flag = (instruction >> 5) & 0x1;
                let dr = Register::from_u16(raw_dr).unwrap();
                let sr1 = Register::from_u16(raw_sr1).unwrap();

                if imm_flag == 1 {
                    let imm5 = sign_extend(instruction & 0x1F, 5);
                    registers.write(
                        dr,
                        registers.read(sr1) + imm5
                    )
                } else {
                    let raw_sr2 = instruction & 0x7;
                    let sr2 = Register::from_u16(raw_sr2).unwrap();
                    registers.write(
                        dr,
                        registers.read(sr1) + registers.read(sr2)
                    )
                }
                registers.update_flags(dr)
            },
            Opcode::LD => {},
            Opcode::ST => {},
            Opcode::JSR => {},
            Opcode::AND => {
                let raw_dr = (instruction >> 9) & 0x7;
                let raw_sr1 = (instruction >> 6) & 0x7;
                let imm_flag = (instruction >> 5) & 0x1;
                let dr = Register::from_u16(raw_dr).unwrap();
                let sr1 = Register::from_u16(raw_sr1).unwrap();

                if imm_flag == 1 {
                    let imm5 = sign_extend(instruction & 0x1F, 5);
                    registers.write(
                        dr,
                        registers.read(sr1) & imm5
                    )
                } else {
                    let raw_sr2 = instruction & 0x7;
                    let sr2 = Register::from_u16(raw_sr2).unwrap();
                    registers.write(
                        dr,
                        registers.read(sr1) & registers.read(sr2)
                    )
                }
                registers.update_flags(dr);
            },
            Opcode::LDR => {},
            Opcode::STR => {},
            Opcode::RTI => {},
            Opcode::NOT => {
                let raw_dr = (instruction >> 9) & 0x7;
                let raw_sr = (instruction >> 6) & 0x7;
                let dr = Register::from_u16(raw_dr).unwrap();
                let sr = Register::from_u16(raw_sr).unwrap();

                registers.write(
                    dr,
                    !registers.read(sr)
                );
                registers.update_flags(dr)
            },
            Opcode::LDI => {
                let raw_dr = (instruction >> 9) & 0x7;
                let dr = Register::from_u16(raw_dr).unwrap();
                let pc_offset = sign_extend(instruction_memory_index & 0x1FF, 9);
                registers.write(
                    dr,
                    memory.read(memory.read(registers.read(Register::PC) + pc_offset)),
                );
                registers.update_flags(dr)
            },
            Opcode::STI => {},
            Opcode::JMP => {},
            Opcode::RES => {},
            Opcode::LEA => {},
            Opcode::TRAP => {},
        }
    }
}

use std::io;
use std::io::{Read, Write};
use crate::flag::ConditionFlag;
use crate::memory::Memory;
use crate::opcode::Opcode;
use crate::register::{Register, Registers};
use crate::trap::TrapCode;
use crate::utils::sign_extend;

pub struct CPU {
    memory: Memory,
    registers: Registers
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            memory: Memory::new(),
            registers: Registers::new(),
        }
    }
    
    pub fn run(&mut self) {
        self.registers.write(Register::COND, ConditionFlag::ZRO as u16);
        self.registers.write(Register::PC, 0x3000u16);
        loop {
            let instruction_memory_index = self.registers.read(Register::PC);
            self.registers.write(Register::PC, instruction_memory_index + 1);
            self.registers.update_flags(Register::R0);
            let instruction = self.memory.read(instruction_memory_index);
            let raw_opcode = instruction >> 12;
            let opcode = Opcode::from_u16(raw_opcode).unwrap();
            match opcode {
                Opcode::BR => {
                    let pc_offset = sign_extend(instruction & 0x1FF, 9);
                    let cond_flag = (instruction >> 9) & 0x7;
                    if cond_flag & self.registers.read(Register::COND) == 1 {
                        self.registers.write(
                            Register::PC,
                            self.registers.read(Register::PC) + pc_offset
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
                        self.registers.write(
                            dr,
                            self.registers.read(sr1) + imm5
                        )
                    } else {
                        let raw_sr2 = instruction & 0x7;
                        let sr2 = Register::from_u16(raw_sr2).unwrap();
                        self.registers.write(
                            dr,
                            self.registers.read(sr1) + self.registers.read(sr2)
                        )
                    }
                    self.registers.update_flags(dr)
                },
                Opcode::LD => {
                    let raw_dr = (instruction >> 9) & 0x7;
                    let pc_offset = sign_extend(instruction & 0x1FF, 9);
                    let dr = Register::from_u16(raw_dr).unwrap();
                    self.registers.write(
                        dr,
                        self.memory.read(self.registers.read(Register::PC) + pc_offset)
                    );
                    self.registers.update_flags(dr)
                },
                Opcode::ST => {
                    let raw_sr = (instruction >> 9) & 0x7;
                    let pc_offset = sign_extend(instruction & 0x1FF, 9);
                    let sr = Register::from_u16(raw_sr).unwrap();
                    self.memory.write(
                        self.registers.read(Register::PC) + pc_offset,
                        self.registers.read(sr)
                    )
                },
                Opcode::JSR => {
                    let long_flag = (instruction >> 11) & 1;
                    self.registers.write(Register::R7, self.registers.read(Register::PC));
                    if long_flag == 1 { /* JSR */
                        let long_pc_offset = sign_extend(instruction & 0x7FF, 11);
                        self.registers.write(
                            Register::PC,
                            self.registers.read(Register::PC) + long_pc_offset
                        )
                    } else { /* JSRR */
                        let raw_base_r = (instruction >> 6) & 0x7;
                        let base_r = Register::from_u16(raw_base_r).unwrap();
                        self.registers.write(
                            Register::PC,
                            self.registers.read(base_r)
                        )
                    }
                },
                Opcode::AND => {
                    let raw_dr = (instruction >> 9) & 0x7;
                    let raw_sr1 = (instruction >> 6) & 0x7;
                    let imm_flag = (instruction >> 5) & 0x1;
                    let dr = Register::from_u16(raw_dr).unwrap();
                    let sr1 = Register::from_u16(raw_sr1).unwrap();
                    if imm_flag == 1 {
                        let imm5 = sign_extend(instruction & 0x1F, 5);
                        self.registers.write(
                            dr,
                            self.registers.read(sr1) & imm5
                        )
                    } else {
                        let raw_sr2 = instruction & 0x7;
                        let sr2 = Register::from_u16(raw_sr2).unwrap();
                        self.registers.write(
                            dr,
                            self.registers.read(sr1) & self.registers.read(sr2)
                        )
                    }
                    self.registers.update_flags(dr);
                },
                Opcode::LDR => {
                    let raw_dr = (instruction >> 9) & 0x7;
                    let raw_base_r = (instruction >> 6) & 0x7;
                    let offset = sign_extend(instruction & 0x3F, 6);
                    let dr = Register::from_u16(raw_dr).unwrap();
                    let base_r = Register::from_u16(raw_base_r).unwrap();
                    self.registers.write(
                        dr,
                        self.memory.read(self.registers.read(base_r) + offset)
                    );
                    self.registers.update_flags(dr);
                },
                Opcode::STR => {
                    let raw_sr = (instruction >> 9) & 0x7;
                    let raw_base_r = (instruction >> 6) & 0x7;
                    let offset = sign_extend(instruction & 0x3F, 6);
                    let sr = Register::from_u16(raw_sr).unwrap();
                    let base_r = Register::from_u16(raw_base_r).unwrap();
                    self.memory.write(
                        self.registers.read(base_r) + offset,
                        self.registers.read(sr)
                    )
                },
                Opcode::RTI => {
                    panic!("RTI is an unused OPCODE")
                },
                Opcode::NOT => {
                    let raw_dr = (instruction >> 9) & 0x7;
                    let raw_sr = (instruction >> 6) & 0x7;
                    let dr = Register::from_u16(raw_dr).unwrap();
                    let sr = Register::from_u16(raw_sr).unwrap();
                    self.registers.write(
                        dr,
                        !self.registers.read(sr)
                    );
                    self.registers.update_flags(dr)
                },
                Opcode::LDI => {
                    let raw_dr = (instruction >> 9) & 0x7;
                    let pc_offset = sign_extend(instruction & 0x1FF, 9);
                    let dr = Register::from_u16(raw_dr).unwrap();
                    self.registers.write(
                        dr,
                        self.memory.read(self.memory.read(self.registers.read(Register::PC) + pc_offset)),
                    );
                    self.registers.update_flags(dr)
                },
                Opcode::STI => {
                    let raw_sr = (instruction >> 9) & 0x7;
                    let pc_offset = sign_extend(instruction & 0x1FF, 9);
                    let sr = Register::from_u16(raw_sr).unwrap();
                    self.memory.write(
                        self.memory.read(self.registers.read(Register::PC) + pc_offset),
                        self.registers.read(sr)
                    )
                },
                Opcode::JMP => {
                    let raw_base_r = (instruction >> 6) & 0x7;
                    let base_r = Register::from_u16(raw_base_r).unwrap();
                    self.registers.write(
                        Register::PC,
                        self.registers.read(base_r)
                    )
                },
                Opcode::RES => {
                    panic!("RES is an unused OPCODE")
                },
                Opcode::LEA => {
                    let raw_dr = (instruction >> 9) & 0x7;
                    let pc_offset = sign_extend(instruction & 0x1FF, 9);
                    let dr = Register::from_u16(raw_dr).unwrap();
                    self.registers.write(
                        dr,
                        self.registers.read(Register::PC) + pc_offset
                    );
                    self.registers.update_flags(dr)
                },
                Opcode::TRAP => {
                    self.registers.write(
                        Register::R7,
                        self.registers.read(Register::PC)
                    );
                    let raw_trap_code = instruction & 0xFF;
                    let trap_code = TrapCode::from_u16(raw_trap_code).unwrap();
                    match trap_code {
                        TrapCode::GETC => {
                            let mut input = [0; 1];
                            io::stdin().read_exact(&mut input).unwrap();
                            self.registers.write(Register::R0, input[0] as u16);
                            self.registers.update_flags(Register::R0)
                        },
                        TrapCode::OUT => {
                            let char_integer = self.registers.read(Register::R0);
                            print!("{}", char_integer as u8 as char);
                            io::stdout().flush().unwrap()
                        },
                        TrapCode::PUTS => {
                            let mut char_mem_idx = self.registers.read(Register::R0);
                            loop {
                                let char_integer = self.memory.read(char_mem_idx);
                                if char_integer == 0 {
                                    break
                                }
                                print!("{}", char_integer as u8 as char);
                                char_mem_idx += 1;
                            }
                            io::stdout().flush().unwrap()
                        },
                        TrapCode::IN => {
                            print!("Enter a character: ");
                            io::stdout().flush().unwrap();
                            let mut input = [0; 1];
                            io::stdin().read_exact(&mut input).unwrap();
                            print!("{}", input[0] as char);
                            io::stdout().flush().unwrap();
                            self.registers.write(Register::R0, input[0] as u16);
                            self.registers.update_flags(Register::R0)
                        },
                        TrapCode::PUTSP => {
                            let mut char_mem_idx = self.registers.read(Register::R0);
                            loop {
                                let char_integer = self.memory.read(char_mem_idx);
                                if char_integer == 0 {
                                    break
                                }
                                let char_1_integer = char_integer & 0xFF;
                                print!("{}", char_1_integer as u8 as char);
                                let char_2_integer = char_integer >> 8;
                                if char_2_integer != 0 {
                                    print!("{}", char_2_integer as u8 as char);
                                }
                                char_mem_idx += 1;
                            }
                            io::stdout().flush().unwrap()
                        },
                        TrapCode::HALT => {
                            print!("HALT");
                            io::stdout().flush().unwrap();
                            break
                        },
                    }
                },
            }
        }
    }
}

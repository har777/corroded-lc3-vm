use crate::utils::{check_key, get_char_byte};

const MEMORY_MAX: usize = 65536;

enum MemoryMappedRegister {
    KBSR = 0xFE00, /* keyboard status */
    KBDR = 0xFE02  /* keyboard data */
}

pub struct Memory {
    data: [u16; MEMORY_MAX],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: [0; MEMORY_MAX],
        }
    }

    pub fn read(&mut self, index: u16) -> u16 {
        if index == MemoryMappedRegister::KBSR as u16 {
            if check_key() {
                self.data[MemoryMappedRegister::KBSR as usize] = 1 << 15;
                self.data[MemoryMappedRegister::KBDR as usize] = get_char_byte().unwrap() as u16;
            } else {
                self.data[MemoryMappedRegister::KBSR as usize] = 0;
            }
        }
        self.data[index as usize]
    }

    pub fn write(&mut self, index: u16, value: u16) {
        self.data[index as usize] = value
    }
}

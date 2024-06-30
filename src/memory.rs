pub struct Memory {
    data: [u16; 65_536],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: [0; 65_536],
        }
    }

    pub fn read(&self, index: u16) -> u16 {
        self.data[index as usize]
    }

    pub fn write(&mut self, index: u16, value: u16) {
        self.data[index as usize] = value
    }
}
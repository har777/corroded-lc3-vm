use crate::flag::ConditionFlag;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,
    COND,
    COUNT
}

#[derive(Debug, PartialEq)]
pub enum RegisterError {
    UnknownRegister(u16),
}


impl Register {
    pub fn from_u16(raw_register: u16) -> Result<Self, RegisterError> {
        match raw_register {
            raw_reg if raw_reg == Register::R0 as u16 => Ok(Register::R0),
            raw_reg if raw_reg == Register::R1 as u16 => Ok(Register::R1),
            raw_reg if raw_reg == Register::R2 as u16 => Ok(Register::R2),
            raw_reg if raw_reg == Register::R3 as u16 => Ok(Register::R3),
            raw_reg if raw_reg == Register::R4 as u16 => Ok(Register::R4),
            raw_reg if raw_reg == Register::R5 as u16 => Ok(Register::R5),
            raw_reg if raw_reg == Register::R6 as u16 => Ok(Register::R6),
            raw_reg if raw_reg == Register::R7 as u16 => Ok(Register::R7),
            raw_reg if raw_reg == Register::PC as u16 => Ok(Register::PC),
            raw_reg if raw_reg == Register::COND as u16 => Ok(Register::COND),
            raw_reg if raw_reg == Register::COUNT as u16 => Ok(Register::COUNT),
            _ => Err(RegisterError::UnknownRegister(raw_register)),
        }
    }
}

pub struct Registers {
    data: [u16; 10],
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            data: [0; 10],
        }
    }

    pub fn read(&self, register: Register) -> u16 {
        self.data[register as usize]
    }

    pub fn write(&mut self, register: Register, value: u16) {
        self.data[register as usize] = value
    }

    pub fn update_flags(&mut self, register: Register) {
        let value = self.data[register as usize];
        self.data[Register::COND as usize] = match value {
            0 => ConditionFlag::ZRO as u16,
            v if v >> 15 == 1 => ConditionFlag::NEG as u16,
            _ => ConditionFlag::POS as u16,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_from_u16_valid() {
        assert_eq!(Register::from_u16(0), Ok(Register::R0));
        assert_eq!(Register::from_u16(1), Ok(Register::R1));
        assert_eq!(Register::from_u16(2), Ok(Register::R2));
        assert_eq!(Register::from_u16(3), Ok(Register::R3));
        assert_eq!(Register::from_u16(4), Ok(Register::R4));
        assert_eq!(Register::from_u16(5), Ok(Register::R5));
        assert_eq!(Register::from_u16(6), Ok(Register::R6));
        assert_eq!(Register::from_u16(7), Ok(Register::R7));
        assert_eq!(Register::from_u16(8), Ok(Register::PC));
        assert_eq!(Register::from_u16(9), Ok(Register::COND));
        assert_eq!(Register::from_u16(10), Ok(Register::COUNT));
    }

    #[test]
    fn test_register_from_u16_invalid() {
        assert_eq!(Register::from_u16(16), Err(RegisterError::UnknownRegister(16)));
    }
}

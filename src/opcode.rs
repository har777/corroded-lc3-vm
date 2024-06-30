#[derive(Debug, PartialEq)]
pub enum Opcode {
    BR,     /* branch */
    ADD,    /* add  */
    LD,     /* load */
    ST,     /* store */
    JSR,    /* jump register */
    AND,    /* bitwise and */
    LDR,    /* load register */
    STR,    /* store register */
    RTI,    /* unused */
    NOT,    /* bitwise not */
    LDI,    /* load indirect */
    STI,    /* store indirect */
    JMP,    /* jump */
    RES,    /* reserved (unused) */
    LEA,    /* load effective address */
    TRAP    /* execute trap */
}

#[derive(Debug, PartialEq)]
pub enum OpcodeError {
    UnknownOpcode(u16),
}

impl Opcode {
    pub fn from_u16(raw_opcode: u16) -> Result<Self, OpcodeError> {
        match raw_opcode {
            raw_op if raw_op == Opcode::BR as u16 => Ok(Opcode::BR),
            raw_op if raw_op == Opcode::ADD as u16 => Ok(Opcode::ADD),
            raw_op if raw_op == Opcode::LD as u16 => Ok(Opcode::LD),
            raw_op if raw_op == Opcode::ST as u16 => Ok(Opcode::ST),
            raw_op if raw_op == Opcode::JSR as u16 => Ok(Opcode::JSR),
            raw_op if raw_op == Opcode::AND as u16 => Ok(Opcode::AND),
            raw_op if raw_op == Opcode::LDR as u16 => Ok(Opcode::LDR),
            raw_op if raw_op == Opcode::STR as u16 => Ok(Opcode::STR),
            raw_op if raw_op == Opcode::RTI as u16 => Ok(Opcode::RTI),
            raw_op if raw_op == Opcode::NOT as u16 => Ok(Opcode::NOT),
            raw_op if raw_op == Opcode::LDI as u16 => Ok(Opcode::LDI),
            raw_op if raw_op == Opcode::STI as u16 => Ok(Opcode::STI),
            raw_op if raw_op == Opcode::JMP as u16 => Ok(Opcode::JMP),
            raw_op if raw_op == Opcode::RES as u16 => Ok(Opcode::RES),
            raw_op if raw_op == Opcode::LEA as u16 => Ok(Opcode::LEA),
            raw_op if raw_op == Opcode::TRAP as u16 => Ok(Opcode::TRAP),
            _ => Err(OpcodeError::UnknownOpcode(raw_opcode)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_from_u16_valid() {
        assert_eq!(Opcode::from_u16(0), Ok(Opcode::BR));
        assert_eq!(Opcode::from_u16(1), Ok(Opcode::ADD));
        assert_eq!(Opcode::from_u16(2), Ok(Opcode::LD));
        assert_eq!(Opcode::from_u16(3), Ok(Opcode::ST));
        assert_eq!(Opcode::from_u16(4), Ok(Opcode::JSR));
        assert_eq!(Opcode::from_u16(5), Ok(Opcode::AND));
        assert_eq!(Opcode::from_u16(6), Ok(Opcode::LDR));
        assert_eq!(Opcode::from_u16(7), Ok(Opcode::STR));
        assert_eq!(Opcode::from_u16(8), Ok(Opcode::RTI));
        assert_eq!(Opcode::from_u16(9), Ok(Opcode::NOT));
        assert_eq!(Opcode::from_u16(10), Ok(Opcode::LDI));
        assert_eq!(Opcode::from_u16(11), Ok(Opcode::STI));
        assert_eq!(Opcode::from_u16(12), Ok(Opcode::JMP));
        assert_eq!(Opcode::from_u16(13), Ok(Opcode::RES));
        assert_eq!(Opcode::from_u16(14), Ok(Opcode::LEA));
        assert_eq!(Opcode::from_u16(15), Ok(Opcode::TRAP));
    }

    #[test]
    fn test_opcode_from_u16_invalid() {
        assert_eq!(Opcode::from_u16(16), Err(OpcodeError::UnknownOpcode(16)));
    }
}

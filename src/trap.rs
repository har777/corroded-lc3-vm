#[derive(Debug, PartialEq)]
pub enum TrapCode {
    GETC = 0x20,  /* get character from keyboard, not echoed onto the terminal */
    OUT = 0x21,   /* output a character */
    PUTS = 0x22,  /* output a word string */
    IN = 0x23,    /* get character from keyboard, echoed onto the terminal */
    PUTSP = 0x24, /* output a byte string */
    HALT = 0x25   /* halt the program */
}

#[derive(Debug, PartialEq)]
pub enum TrapCodeError {
    UnknownTrapCode(u16),
}

impl TrapCode {
    pub fn from_u16(raw_trap_code: u16) -> Result<Self, TrapCodeError> {
        match raw_trap_code {
            raw_tc if raw_tc == TrapCode::GETC as u16 => Ok(TrapCode::GETC),
            raw_tc if raw_tc == TrapCode::OUT as u16 => Ok(TrapCode::OUT),
            raw_tc if raw_tc == TrapCode::PUTS as u16 => Ok(TrapCode::PUTS),
            raw_tc if raw_tc == TrapCode::IN as u16 => Ok(TrapCode::IN),
            raw_tc if raw_tc == TrapCode::PUTSP as u16 => Ok(TrapCode::PUTSP),
            raw_tc if raw_tc == TrapCode::HALT as u16 => Ok(TrapCode::HALT),
            _ => Err(TrapCodeError::UnknownTrapCode(raw_trap_code)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trap_code_from_u16_valid() {
        assert_eq!(TrapCode::from_u16(0x20), Ok(TrapCode::GETC));
        assert_eq!(TrapCode::from_u16(0x21), Ok(TrapCode::OUT));
        assert_eq!(TrapCode::from_u16(0x22), Ok(TrapCode::PUTS));
        assert_eq!(TrapCode::from_u16(0x23), Ok(TrapCode::IN));
        assert_eq!(TrapCode::from_u16(0x24), Ok(TrapCode::PUTSP));
        assert_eq!(TrapCode::from_u16(0x25), Ok(TrapCode::HALT));
    }

    #[test]
    fn test_raw_trap_code_from_u16_invalid() {
        assert_eq!(TrapCode::from_u16(1), Err(TrapCodeError::UnknownTrapCode(1)));
    }
}

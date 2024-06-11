use std::convert::From;

#[derive(Debug, PartialEq)]
pub enum OpcodeError {
    InvalidAddress(u16),
    InvalidOpcode(u16),
}

#[derive(Debug, PartialEq)]
pub enum Opcode {
    // System Commands
    ClearDisplay, // 00E0
    Return,       // 00EE

    // Jump & Call Commands
    SysAddr(u16), // 0NNN
    Jump(u16),    // 1NNN
    Call(u16),    // 2NNN

    // Conditional Commands
    SkipIfEqual(u8, u8),       // 3XNN
    SkipIfNotEqual(u8, u8),    // 4XNN
    SkipIfRegEqual(u8, u8),    // 5XY0
    SkipIfRegNotEqual(u8, u8), // 9XY0

    // Register Commands
    LoadByte(u8, u8), // 6XNN
    AddByte(u8, u8),  // 7XNN
    LoadReg(u8, u8),  // 8XY0
    Or(u8, u8),       // 8XY1
    And(u8, u8),      // 8XY2
    Xor(u8, u8),      // 8XY3
    AddReg(u8, u8),   // 8XY4
    Sub(u8, u8),      // 8XY5
    ShiftRight(u8),   // 8XY6
    SubN(u8, u8),     // 8XY7
    ShiftLeft(u8),    // 8XYE

    // Memory Commands
    SetIndex(u16),  // ANNN
    JumpV0(u16),    // BNNN
    Random(u8, u8), // CXNN

    // Display Commands
    Draw(u8, u8, u8), // DXYN

    // KeyOp Commands
    SkipIfKeyPressed(u8),    // EX9E
    SkipIfKeyNotPressed(u8), // EXA1

    // Timer & Sound Commands
    LoadDelayTimer(u8), // FX07
    WaitForKey(u8),     // FX0A
    SetDelayTimer(u8),  // FX15
    SetSoundTimer(u8),  // FX18

    // I Register Commands
    AddI(u8),           // FX1E
    LoadSpriteAddr(u8), // FX29
    StoreBCD(u8),       // FX33

    // Memory & Register Commands
    RegDump(u8), // FX55
    RegLoad(u8), // FX65

    // Undefined or unknown opcode
    Undefined(u16), // For any opcode that doesn't match the above
}

struct Instruction {
    msb: u8,
    lsb: u8,
}

impl From<(u8, u8)> for Instruction {
    fn from(bytes: (u8, u8)) -> Self {
        Instruction {
            msb: bytes.0,
            lsb: bytes.1,
        }
    }
}
impl Instruction {
    fn get_address(&self) -> Result<u16, OpcodeError> {
        let addr = ((self.msb as u16 & 0x0F) << 8) | self.lsb as u16;
        // Return Err if the address is out of bounds (only first 12 bits are used for address space).
        if 0xC00 & addr != 0 {
            Err(OpcodeError::InvalidAddress(addr))
        } else {
            Ok(addr)
        }
    }

    fn get_code(&self) -> u8 {
        (self.msb & 0xF0) >> 4
    }

    fn get_n(&self) -> u8 {
        self.lsb & 0x0F
    }

    fn get_x(&self) -> u8 {
        self.msb & 0x0F
    }

    fn get_y(&self) -> u8 {
        self.lsb >> 4
    }

    fn get_kk(&self) -> u8 {
        self.lsb
    }

    fn get_byte(&self) -> u8 {
        self.lsb
    }
}

impl Opcode {
    pub fn from_bytes(msb: u8, lsb: u8) -> Result<Opcode, OpcodeError> {
        let instruction = Instruction::from((msb, lsb));
        let (code, x, y, n) = (
            instruction.get_code(),
            instruction.get_x(),
            instruction.get_y(),
            instruction.get_n(),
        );

        match (code, x, y, n) {
            (0, 0, 0xE, 0) => Ok(Opcode::ClearDisplay),
            (0, 0, 0xE, 0xE) => Ok(Opcode::Return),
            (0, _, _, _) => Ok(Opcode::SysAddr(instruction.get_address()?)),
            (0x1, _, _, _) => Ok(Opcode::Jump(instruction.get_address()?)),
            (0x2, _, _, _) => Ok(Opcode::Call(instruction.get_address()?)),
            (0x3, _, _, _) => Ok(Opcode::SkipIfEqual(
                instruction.get_x(),
                instruction.get_kk(),
            )),
            (0x4, _, _, _) => Ok(Opcode::SkipIfNotEqual(
                instruction.get_x(),
                instruction.get_kk(),
            )),
            (0x5, _, _, _) => Ok(Opcode::SkipIfRegEqual(
                instruction.get_x(),
                instruction.get_y(),
            )),
            (0x6, _, _, _) => Ok(Opcode::LoadByte(
                instruction.get_x(),
                instruction.get_byte(),
            )),
            (0x7, _, _, _) => Ok(Opcode::AddByte(instruction.get_x(), instruction.get_byte())),
            (0x8, _, _, 0x0) => Ok(Opcode::LoadReg(instruction.get_x(), instruction.get_y())),
            (0x8, _, _, 0x1) => Ok(Opcode::Or(instruction.get_x(), instruction.get_y())),
            (0x8, _, _, 0x2) => Ok(Opcode::And(instruction.get_x(), instruction.get_y())),
            (0x8, _, _, 0x3) => Ok(Opcode::Xor(instruction.get_x(), instruction.get_y())),
            (0x8, _, _, 0x4) => Ok(Opcode::AddReg(instruction.get_x(), instruction.get_y())),
            (0x8, _, _, 0x5) => Ok(Opcode::Sub(instruction.get_x(), instruction.get_y())),
            (0x8, _, _, 0x6) => Ok(Opcode::ShiftRight(instruction.get_x())),
            (0x8, _, _, 0x7) => Ok(Opcode::SubN(instruction.get_x(), instruction.get_y())),
            (0x8, _, _, 0xE) => Ok(Opcode::ShiftLeft(instruction.get_x())),
            (0x9, _, _, 0x0) => Ok(Opcode::SkipIfRegNotEqual(
                instruction.get_x(),
                instruction.get_y(),
            )),
            (0xA, _, _, _) => Ok(Opcode::SetIndex(instruction.get_address()?)),
            (0xB, _, _, _) => Ok(Opcode::JumpV0(instruction.get_address()?)),
            (0xC, _, _, _) => Ok(Opcode::Random(instruction.get_x(), instruction.get_byte())),
            (0xD, _, _, _) => Ok(Opcode::Draw(
                instruction.get_x(),
                instruction.get_y(),
                instruction.get_n(),
            )),
            (0xE, _, 0x9, 0xE) => Ok(Opcode::SkipIfKeyPressed(instruction.get_x())),
            (0xE, _, 0xA, 0x1) => Ok(Opcode::SkipIfKeyNotPressed(instruction.get_x())),
            (0xF, _, 0x0, 0x7) => Ok(Opcode::LoadDelayTimer(instruction.get_x())),
            (0xF, _, 0x0, 0xA) => Ok(Opcode::WaitForKey(instruction.get_x())),
            (0xF, _, 0x1, 0x5) => Ok(Opcode::SetDelayTimer(instruction.get_x())),
            (0xF, _, 0x1, 0x8) => Ok(Opcode::SetSoundTimer(instruction.get_x())),
            (0xF, _, 0x1, 0xE) => Ok(Opcode::AddI(instruction.get_x())),
            (0xF, _, 0x2, 0x9) => Ok(Opcode::LoadSpriteAddr(instruction.get_x())),
            (0xF, _, 0x3, 0x3) => Ok(Opcode::StoreBCD(instruction.get_x())),
            (0xF, _, 0x5, 0x5) => Ok(Opcode::RegDump(instruction.get_x())),
            (0xF, _, 0x6, 0x5) => Ok(Opcode::RegLoad(instruction.get_x())),

            _ => Err(OpcodeError::InvalidOpcode((msb as u16) << 8 | lsb as u16)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_from_bytes_clear_display() {
        let opcode = Opcode::from_bytes(0x00, 0xE0);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::ClearDisplay);
    }

    #[test]
    fn test_opcode_from_bytes_return() {
        let opcode = Opcode::from_bytes(0x00, 0xEE);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::Return);
    }

    #[test]
    fn test_opcode_from_bytes_sys_addr() {
        let opcode = Opcode::from_bytes(0x01, 0x23);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::SysAddr(0x123));
    }

    #[test]
    fn test_opcode_from_bytes_jump() {
        let opcode = Opcode::from_bytes(0x12, 0x34);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::Jump(0x234));
    }

    #[test]
    fn test_opcode_from_bytes_call() {
        let opcode = Opcode::from_bytes(0x23, 0x45);
        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::Call(0x345));
    }

    #[test]
    fn test_opcode_from_bytes_skip_if_equal() {
        let opcode = Opcode::from_bytes(0x30, 0x12);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::SkipIfEqual(0x0, 0x12));
    }

    #[test]
    fn test_opcode_from_bytes_skip_if_not_equal() {
        let opcode = Opcode::from_bytes(0x40, 0x34);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::SkipIfNotEqual(0x0, 0x34));
    }

    #[test]
    fn test_opcode_from_bytes_skip_if_reg_equal() {
        let opcode = Opcode::from_bytes(0x50, 0x40);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::SkipIfRegEqual(0x0, 0x4));
    }

    #[test]
    fn test_opcode_from_bytes_load_byte() {
        let opcode = Opcode::from_bytes(0x60, 0x23);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::LoadByte(0x0, 0x23));
    }

    #[test]
    fn test_opcode_from_bytes_add_byte() {
        let opcode = Opcode::from_bytes(0x70, 0x45);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::AddByte(0x0, 0x45));
    }

    #[test]
    fn test_opcode_from_bytes_load_reg() {
        let opcode = Opcode::from_bytes(0x80, 0x10);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::LoadReg(0x0, 0x1));
    }

    #[test]
    fn test_opcode_from_bytes_or() {
        let opcode = Opcode::from_bytes(0x80, 0x21);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::Or(0x0, 0x2));
    }

    #[test]
    fn test_opcode_from_str_and() {
        let opcode = Opcode::from_bytes(0x81, 0x32);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::And(0x1, 0x3));
    }

    #[test]
    fn test_opcode_from_bytes_xor() {
        let opcode = Opcode::from_bytes(0x80, 0x23);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::Xor(0x0, 0x2));
    }

    #[test]
    fn test_opcode_from_bytes_add_reg() {
        let opcode = Opcode::from_bytes(0x80, 0x34);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::AddReg(0x0, 0x3));
    }

    #[test]
    fn test_opcode_from_bytes_add_sub() {
        let opcode = Opcode::from_bytes(0x80, 0x45);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::Sub(0x0, 0x4));
    }

    #[test]
    fn test_opcode_from_bytes_shr() {
        let opcode = Opcode::from_bytes(0x80, 0x56);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::ShiftRight(0x0));
    }

    #[test]
    fn test_opcode_from_bytes_subn() {
        let opcode = Opcode::from_bytes(0x80, 0x67);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::SubN(0x0, 0x6));
    }

    #[test]
    fn test_opcode_from_bytes_shift_left() {
        let opcode = Opcode::from_bytes(0x80, 0x8E);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::ShiftLeft(0x0));
    }

    #[test]
    fn test_opcode_from_bytes_skip_if_reg_not_equal() {
        let opcode = Opcode::from_bytes(0x90, 0x50);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::SkipIfRegNotEqual(0x0, 0x5));
    }

    #[test]
    fn test_opcode_from_bytes_set_index() {
        let opcode = Opcode::from_bytes(0xA0, 0x12);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::SetIndex(0x012));
    }

    #[test]
    fn test_opcode_from_bytes_jump_v0() {
        let opcode = Opcode::from_bytes(0xB0, 0x12);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::JumpV0(0x012));
    }

    #[test]
    fn test_opcode_from_bytes_random() {
        let opcode = Opcode::from_bytes(0xC0, 0x12);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::Random(0x0, 0x12));
    }

    #[test]
    fn test_opcode_from_bytes_draw() {
        let opcode = Opcode::from_bytes(0xD0, 0x12);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::Draw(0x0, 0x1, 0x2));
    }

    #[test]
    fn test_opcode_from_bytes_skip_if_key_pressed() {
        let opcode = Opcode::from_bytes(0xE0, 0x9E);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::SkipIfKeyPressed(0x0));
    }

    #[test]
    fn test_opcode_from_bytes_skip_if_key_not_pressed() {
        let opcode = Opcode::from_bytes(0xE0, 0xA1);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::SkipIfKeyNotPressed(0x0));
    }

    #[test]
    fn test_opcode_from_bytes_load_delay_timer() {
        let opcode = Opcode::from_bytes(0xF1, 0x07);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::LoadDelayTimer(0x1));
    }

    #[test]
    fn test_opcode_from_bytes_wait_for_key() {
        let opcode = Opcode::from_bytes(0xF1, 0x0A);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::WaitForKey(0x1));
    }

    #[test]
    fn test_opcode_from_bytes_set_delay_timer() {
        let opcode = Opcode::from_bytes(0xF0, 0x15);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::SetDelayTimer(0x0));
    }

    #[test]
    fn test_opcode_from_bytes_set_sound_timer() {
        let opcode = Opcode::from_bytes(0xF0, 0x18);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::SetSoundTimer(0x0));
    }

    #[test]
    fn test_opcode_from_bytes_add_i() {
        let opcode = Opcode::from_bytes(0xF0, 0x1E);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::AddI(0x0));
    }

    #[test]
    fn test_opcode_from_bytes_load_sprite_addr() {
        let opcode = Opcode::from_bytes(0xF0, 0x29);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::LoadSpriteAddr(0x0));
    }

    #[test]
    fn test_opcode_from_bytes_store_bcd() {
        let opcode = Opcode::from_bytes(0xF0, 0x33);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::StoreBCD(0x0));
    }

    #[test]
    fn test_opcode_from_bytes_reg_dump() {
        let opcode = Opcode::from_bytes(0xF0, 0x55);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::RegDump(0x0));
    }

    #[test]
    fn test_opcode_from_bytes_reg_load() {
        let opcode = Opcode::from_bytes(0xF0, 0x65);

        assert!(opcode.is_ok(), "{:?}", opcode);
        assert_eq!(opcode.unwrap(), Opcode::RegLoad(0x0));
    }

    #[test]
    fn test_opcode_from_bytes_with_invalid_opcode() {
        let opcode = Opcode::from_bytes(0xFA, 0xBC);

        assert!(opcode.is_err(), "{:?}", opcode);
        assert_eq!(opcode.unwrap_err(), OpcodeError::InvalidOpcode(0xFABC));
    }

    #[test]
    fn test_opcode_from_bytes_with_invalid_address() {
        let opcode = Opcode::from_bytes(0x2F, 0xFF);

        assert!(opcode.is_err(), "{:?}", opcode);
        assert_eq!(opcode.unwrap_err(), OpcodeError::InvalidAddress(0xFFF));
    }
}

use opcodes::{Opcode, OpcodeError};
use rand::random;
use std::{convert::From};

mod display;
mod input;
mod memory;
mod opcodes;
mod registers;
mod stack;
mod timers;

const PROGRAM_START_ADDRESS: usize = 0x200;

#[derive(Debug, PartialEq)]
pub enum Chip8Error {
    StackError(stack::StackError),
    MemoryError(memory::MemoryError),
    OpcodeError(OpcodeError),
    BootError
}

impl From<stack::StackError> for Chip8Error {
    fn from(err: stack::StackError) -> Chip8Error {
        Chip8Error::StackError(err)
    }
}

impl From<memory::MemoryError> for Chip8Error {
    fn from(err: memory::MemoryError) -> Chip8Error {
        Chip8Error::MemoryError(err)
    }
}

impl From<OpcodeError> for Chip8Error {
    fn from(err: OpcodeError) -> Chip8Error {
        Chip8Error::OpcodeError(err)
    }
}

#[derive(Debug)]
pub struct Chip8 {
    memory: memory::Memory,
    registers: registers::Registers,
    stack: stack::Stack,
    timers: timers::Timers,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            memory: memory::Memory::new(),
            registers: registers::Registers::new(),
            stack: stack::Stack::new(),
            timers: timers::Timers::new(),
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        for (i, &byte) in rom.iter().enumerate() {
            let addr = PROGRAM_START_ADDRESS + i;
            self.memory.write_byte(addr, byte).unwrap();
        }
    }

    pub fn load_rom_from_file(&mut self, path: &str) -> Result<(), std::io::Error> {
        let rom = std::fs::read(path)?;
        self.load_rom(&rom);
        Ok(())
    }

    pub fn boot(&mut self) -> Result<(), Chip8Error> {
        self.registers.pc = PROGRAM_START_ADDRESS as u16;
        self.load_sprites()
    }

    fn load_sprites(&mut self) -> Result<(), Chip8Error> {
        let sprite_size = display::SPRITES[0].len() as usize;
        for (sprite_idx, sprite) in display::SPRITES.iter().enumerate() {
            for (byte_idx, &byte) in sprite.iter().enumerate() {
                let write_addr = display::SPRITE_START_ADDRESS + sprite_idx * sprite_size + byte_idx;
                self.memory
                    .write_byte(write_addr, byte)?;
            }
        }
        Ok(())
    }

    fn execute(&mut self, op: Opcode) -> Result<(), Chip8Error> {
        match op {
            Opcode::AddByte(vx, val) => self.add_vx_byte(vx, val),
            Opcode::AddI(vx) => self.add_i_vx(vx),
            Opcode::AddReg(vx, vy) => self.add_reg(vx, vy),
            Opcode::And(vx, vy) => self.and(vx, vy),
            Opcode::Call(addr) => self.call(addr)?,
            Opcode::Jump(addr) => self.jump(addr),
            Opcode::JumpV0(addr) => self.jump_v0(addr),
            Opcode::LoadByte(vx, byte) => self.load_byte(vx, byte),
            Opcode::LoadDelayTimer(vx) => self.load_delay_timer(vx),
            Opcode::LoadReg(vx, vy) => self.load_register(vx, vy),
            Opcode::Or(vx, vy) => self.or(vx, vy),
            Opcode::Random(vx, byte) => self.random(vx, byte),
            Opcode::RegDump(vx) => self.reg_dump(vx)?,
            Opcode::RegLoad(vx) => self.reg_load(vx)?,
            Opcode::Return => self.return_from()?,
            Opcode::SetDelayTimer(vx) => self.set_delay_timer(vx),
            Opcode::SetIndex(addr) => self.set_index(addr),
            Opcode::SetSoundTimer(vx) => self.set_sound_timer(vx),
            Opcode::ShiftLeft(vx) => self.shift_left(vx),
            Opcode::ShiftRight(vx) => self.shift_right(vx),
            Opcode::SkipIfEqual(vx, byte) => self.skip_if_equal(vx, byte),
            Opcode::SkipIfNotEqual(vx, byte) => self.skip_if_not_equal(vx, byte),
            Opcode::SkipIfRegEqual(vx, vy) => self.skip_if_reg_equal(vx, vy),
            Opcode::SkipIfRegNotEqual(vx, vy) => self.skip_if_reg_not_equal(vx, vy),
            Opcode::Sub(vx, vy) => self.sub(vx, vy),
            Opcode::SubN(vx, vy) => self.subn(vx, vy),
            Opcode::Xor(vx, vy) => self.xor(vx, vy),
            Opcode::StoreBCD(vx) => self.store_bcd(vx),
            Opcode::SysAddr(addr) => {}
            Opcode::LoadSpriteAddr(vx) => unimplemented!(),
            Opcode::Draw(vx, vy, n) => unimplemented!(),
            Opcode::SkipIfKeyNotPressed(vx) => unimplemented!(),
            Opcode::SkipIfKeyPressed(vx) => unimplemented!(),
            Opcode::ClearDisplay => unimplemented!(),
            Opcode::WaitForKey(vx) => unimplemented!(),
            Opcode::Undefined(opcode) => {
                return Err(Chip8Error::OpcodeError(OpcodeError::InvalidOpcode(opcode)))
            }
        }
        Ok(())
    }

    fn subn(&mut self, vx: u8, vy: u8) {
        let vx_val = self.registers.read_v(vx);
        let vy_val = self.registers.read_v(vy);

        let (result, borrow) = vy_val.overflowing_sub(vx_val);

        self.registers.write_v(vx, result);
        self.registers.write_v(0xF, if borrow { 0 } else { 1 });
    }

    fn skip_if_reg_not_equal(&mut self, vx: u8, vy: u8) {
        let vx_val = self.registers.read_v(vx);
        let vy_val = self.registers.read_v(vy);

        if vx_val != vy_val {
            self.registers.pc += 2;
        }
    }

    fn xor(&mut self, vx: u8, vy: u8) {
        let vx_val = self.registers.read_v(vx);
        let vy_val = self.registers.read_v(vy);

        let result = vx_val ^ vy_val;

        self.registers.write_v(vx, result);
    }

    fn store_bcd(&mut self, vx: u8) {
        let vx_val = self.registers.read_v(vx);
        let i = self.registers.i as usize;

        self.memory.write_byte(i, vx_val / 100).unwrap();
        self.memory.write_byte(i + 1, (vx_val / 10) % 10).unwrap();
        self.memory.write_byte(i + 2, vx_val % 10).unwrap();
    }

    fn sub(&mut self, vx: u8, vy: u8) {
        let vx_val = self.registers.read_v(vx);
        let vy_val = self.registers.read_v(vy);

        let (result, borrow) = vx_val.overflowing_sub(vy_val);

        self.registers.write_v(vx, result);
        self.registers.write_v(0xF, if borrow { 0 } else { 1 });
    }

    fn skip_if_not_equal(&mut self, vx: u8, byte: u8) {
        let vx_val = self.registers.read_v(vx);
        if vx_val != byte {
            self.registers.pc += 2;
        }
    }

    fn skip_if_reg_equal(&mut self, vx: u8, vy: u8) {
        let vx_val = self.registers.read_v(vx);
        let vy_val = self.registers.read_v(vy);

        if vx_val == vy_val {
            self.registers.pc += 2;
        }
    }

    fn skip_if_equal(&mut self, vx: u8, byte: u8) {
        let vx_val = self.registers.read_v(vx);
        if vx_val == byte {
            self.registers.pc += 2;
        }
    }

    fn shift_right(&mut self, vx: u8) {
        let vx_val = self.registers.read_v(vx);
        let lsb = vx_val & 0b00000001;

        self.registers.write_v(vx, vx_val >> 1);
        self.registers.write_v(0xF, lsb);
    }

    fn shift_left(&mut self, vx: u8) {
        let vx_val = self.registers.read_v(vx);
        let overflow = vx_val & 0b10000000 != 0;

        self.registers.write_v(vx, vx_val << 1);
        self.registers.write_v(0xF, if overflow { 1 } else { 0 });
    }

    fn set_sound_timer(&mut self, vx: u8) {
        let vx_val = self.registers.read_v(vx);
        self.timers.set_sound_timer(vx_val);
    }

    fn set_index(&mut self, addr: u16) {
        self.registers.i = addr;
    }

    fn set_delay_timer(&mut self, vx: u8) {
        let vx_val = self.registers.read_v(vx);
        self.timers.set_delay_timer(vx_val);
    }

    fn return_from(&mut self) -> Result<(), Chip8Error> {
        self.registers.pc = self.stack.pop()?;
        Ok(())
    }

    fn reg_load(&mut self, vx: u8) -> Result<(), Chip8Error> {
        for reg in 0..=vx {
            let reg_val = self
                .memory
                .read_byte(self.registers.i as usize + reg as usize)?;
            self.registers.write_v(reg, reg_val);
        }
        Ok(())
    }
    fn reg_dump(&mut self, vx: u8) -> Result<(), Chip8Error> {
        for reg in 0..=vx {
            let reg_val = self.registers.read_v(reg);
            self.memory
                .write_byte(self.registers.i as usize + reg as usize, reg_val)?;
        }
        Ok(())
    }
    fn random(&mut self, vx: u8, byte: u8) {
        let random_byte = random::<u8>();
        self.registers.write_v(vx, random_byte & byte);
    }

    fn or(&mut self, vx: u8, vy: u8) {
        let vx_val = self.registers.read_v(vx);
        let vy_val = self.registers.read_v(vy);

        let result = vx_val | vy_val;

        self.registers.write_v(vx, result);
    }

    fn load_register(&mut self, vx: u8, vy: u8) {
        let vy_val = self.registers.read_v(vy);
        self.registers.write_v(vx, vy_val);
    }

    fn load_delay_timer(&mut self, vx: u8) {
        let delay_timer = self.timers.get_delay_timer();
        self.registers.write_v(vx, delay_timer);
    }

    fn load_byte(&mut self, vx: u8, byte: u8) {
        self.registers.write_v(vx, byte);
    }

    fn jump_v0(&mut self, addr: u16) {
        self.jump(self.registers.read_v(0) as u16 + addr);
    }

    fn jump(&mut self, addr: u16) {
        self.registers.pc = addr;
    }

    fn call(&mut self, addr: u16) -> Result<(), Chip8Error> {
        self.stack.push(self.registers.pc)?;
        self.registers.pc = addr;
        Ok(())
    }

    fn and(&mut self, vx: u8, vy: u8) {
        let vx_val = self.registers.read_v(vx);
        let vy_val = self.registers.read_v(vy);

        let result = vx_val & vy_val;

        self.registers.write_v(vx, result);
    }

    fn add_reg(&mut self, vx: u8, vy: u8) {
        let vx_val = self.registers.read_v(vx);
        let vy_val = self.registers.read_v(vy);

        let (result, overflow) = vx_val.overflowing_add(vy_val);

        self.registers.write_v(vx, result);
        self.registers.write_v(0xF, if overflow { 1 } else { 0 });
    }

    fn add_vx_byte(&mut self, vx: u8, val: u8) {
        let result = self.registers.read_v(vx).wrapping_add(val);
        self.registers.write_v(vx, result);
    }

    fn add_i_vx(&mut self, vx: u8) {
        let vx_val = self.registers.read_v(vx) as u16;
        self.registers.i = vx_val.wrapping_add(self.registers.i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chip8_new() {
        Chip8::new();
    }

    #[test]
    fn test_chip8_load_rom() {
        let mut chip8 = Chip8::new();
        let rom = [0x12, 0x34, 0x56, 0x78];
        chip8.load_rom(&rom);

        // Verify that the ROM is loaded correctly into memory
        assert_eq!(chip8.memory.read_byte(PROGRAM_START_ADDRESS), Ok(0x12));
        assert_eq!(chip8.memory.read_byte(PROGRAM_START_ADDRESS + 1), Ok(0x34));
        assert_eq!(chip8.memory.read_byte(PROGRAM_START_ADDRESS + 2), Ok(0x56));
        assert_eq!(chip8.memory.read_byte(PROGRAM_START_ADDRESS + 3), Ok(0x78));
    }

    #[test]
    fn test_chip8_load_rom_from_non_existing_file() {
        let mut chip8 = Chip8::new();
        let result = chip8.load_rom_from_file("nonexistent_file.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_chip8_load_rom_from_file() {
        let mut chip8 = Chip8::new();

        // Create a temporary file with some bytes
        let temp_file_path = "./test_rom.ch8";
        std::fs::write(temp_file_path, &[0xAB, 0xCD, 0xEF]).unwrap();

        // Load the ROM from the temporary file
        let result = chip8.load_rom_from_file(temp_file_path);
        assert!(result.is_ok());

        // Verify that the ROM is loaded correctly into memory
        assert_eq!(chip8.memory.read_byte(PROGRAM_START_ADDRESS), Ok(0xAB));
        assert_eq!(chip8.memory.read_byte(PROGRAM_START_ADDRESS + 1), Ok(0xCD));
        assert_eq!(chip8.memory.read_byte(PROGRAM_START_ADDRESS + 2), Ok(0xEF));

        // Delete the temporary file
        std::fs::remove_file(temp_file_path).unwrap();
    }

    #[test]
    fn test_chip8_execute_add_byte() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x0, 0x10);

        chip8.execute(Opcode::AddByte(0x0, 0x20)).unwrap();

        assert_eq!(chip8.registers.read_v(0x0), 0x30);
    }

    #[test]
    fn test_chip8_execute_add_i() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x0, 0x10);
        chip8.registers.i = 0x100;

        chip8.execute(Opcode::AddI(0x0)).unwrap();

        assert_eq!(chip8.registers.i, 0x110);
    }

    #[test]
    fn test_chip8_execute_add_reg_no_overflow() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x0, 0x10);
        chip8.registers.write_v(0x1, 0x20);

        chip8.execute(Opcode::AddReg(0x0, 0x1)).unwrap();

        assert_eq!(chip8.registers.read_v(0x0), 0x30);
        assert_eq!(chip8.registers.read_v(0xF), 0x0);
    }

    #[test]
    fn test_chip8_execute_add_reg_with_overflow() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x0, 0xFF);
        chip8.registers.write_v(0x1, 0x01);

        chip8.execute(Opcode::AddReg(0x0, 0x1)).unwrap();

        assert_eq!(chip8.registers.read_v(0x0), 0x00);
        assert_eq!(chip8.registers.read_v(0xF), 0x1);
    }

    #[test]
    fn test_chip8_execute_and() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x0, 0b10101010);
        chip8.registers.write_v(0x1, 0b11001100);

        chip8.execute(Opcode::And(0x0, 0x1)).unwrap();

        assert_eq!(chip8.registers.read_v(0x0), 0b10001000);
    }
    #[test]
    fn test_chip8_execute_call() {
        let mut chip8 = Chip8::new();
        chip8.registers.pc = 0x200;

        chip8.execute(Opcode::Call(0x300)).unwrap();

        assert_eq!(chip8.stack.pop(), Ok(0x200));
        assert_eq!(chip8.registers.pc, 0x300);
    }

    #[test]
    fn test_chip8_execute_call_stack_overflow() {
        let mut chip8 = Chip8::new();
        chip8.registers.pc = 0x200;
        for _ in 0..16 {
            chip8.stack.push(0x200).unwrap();
        }

        let result = chip8.execute(Opcode::Call(0x300));

        assert_eq!(
            result.unwrap_err(),
            Chip8Error::StackError(stack::StackError::StackOverflow)
        );
        assert_eq!(chip8.registers.pc, 0x200);
    }
    #[test]
    fn test_chip8_execute_jump() {
        let mut chip8 = Chip8::new();

        chip8.execute(Opcode::Jump(0x300)).unwrap();

        assert_eq!(chip8.registers.pc, 0x300);
    }

    #[test]
    fn test_chip8_execute_jump_v0() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x0, 0x01);

        chip8.execute(Opcode::JumpV0(0x300)).unwrap();

        assert_eq!(chip8.registers.pc, 0x301);
    }

    #[test]
    fn test_chip8_execute_load_byte() {
        let mut chip8 = Chip8::new();

        chip8.execute(Opcode::LoadByte(0x0, 0xFF)).unwrap();

        assert_eq!(chip8.registers.read_v(0x0), 0xFF);
    }
    #[test]
    fn test_chip8_execute_load_delay_timer() {
        let mut chip8 = Chip8::new();
        chip8.timers.set_delay_timer(0x10);

        chip8.execute(Opcode::LoadDelayTimer(0x0)).unwrap();

        assert_eq!(chip8.registers.read_v(0x0), 0x10);
    }
    #[test]
    fn test_chip8_execute_load_register() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x1, 0x42);

        chip8.execute(Opcode::LoadReg(0x0, 0x1)).unwrap();

        assert_eq!(chip8.registers.read_v(0x0), 0x42);
    }
    #[test]
    fn test_chip8_execute_or() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x0, 0b10101010);
        chip8.registers.write_v(0x1, 0b11001100);

        chip8.execute(Opcode::Or(0x0, 0x1)).unwrap();

        assert_eq!(chip8.registers.read_v(0x0), 0b11101110);
    }
    #[test]
    fn test_chip8_execute_random() {
        let mut chip8 = Chip8::new();

        chip8.execute(Opcode::Random(0x0, 0b11001100)).unwrap();

        let result = chip8.registers.read_v(0x0);
        assert_eq!(result & 0b11001100, result);
    }
    #[test]
    fn test_chip8_execute_reg_dump() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x0, 0x01);
        chip8.registers.write_v(0x1, 0x02);
        chip8.registers.write_v(0x2, 0x03);
        chip8.registers.write_v(0x3, 0x04);
        chip8.registers.i = 0x100;

        chip8.execute(Opcode::RegDump(0x3)).unwrap();

        assert_eq!(chip8.memory.read_byte(0x100), Ok(0x01));
        assert_eq!(chip8.memory.read_byte(0x101), Ok(0x02));
        assert_eq!(chip8.memory.read_byte(0x102), Ok(0x03));
        assert_eq!(chip8.memory.read_byte(0x103), Ok(0x04));
    }

    #[test]
    fn test_chip8_execute_reg_dump_memory_error() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x0, 0x01);
        chip8.registers.write_v(0x1, 0x02);
        chip8.registers.write_v(0x2, 0x03);
        chip8.registers.write_v(0x3, 0x04);
        chip8.registers.i = 0xFFFF;

        let result = chip8.execute(Opcode::RegDump(0x3));

        assert_eq!(
            result.unwrap_err(),
            Chip8Error::MemoryError(memory::MemoryError::AddressOutOfBounds)
        );
    }
    #[test]
    fn test_chip8_execute_reg_load() {
        let mut chip8 = Chip8::new();
        chip8.registers.i = 0x100;
        chip8.memory.write_byte(0x100, 0x42).unwrap();
        chip8.memory.write_byte(0x101, 0x43).unwrap();

        chip8.execute(Opcode::RegLoad(0x01)).unwrap();

        assert_eq!(chip8.registers.read_v(0x0), 0x42);
        assert_eq!(chip8.registers.read_v(0x1), 0x43);
    }

    #[test]
    fn test_chip8_execute_reg_load_memory_error() {
        let mut chip8 = Chip8::new();
        chip8.registers.i = 0xFFFF;

        let result = chip8.execute(Opcode::RegLoad(0x0));

        assert_eq!(
            result.unwrap_err(),
            Chip8Error::MemoryError(memory::MemoryError::AddressOutOfBounds)
        );
    }
    #[test]
    fn test_chip8_execute_return() {
        let mut chip8 = Chip8::new();
        chip8.stack.push(0x300).unwrap();

        chip8.execute(Opcode::Return).unwrap();

        assert_eq!(chip8.registers.pc, 0x300);
    }

    #[test]
    fn test_chip8_execute_return_empty_stack() {
        let mut chip8 = Chip8::new();
        chip8.registers.pc = 0x200;

        let result = chip8.execute(Opcode::Return);

        assert_eq!(
            result.unwrap_err(),
            Chip8Error::StackError(stack::StackError::StackUnderflow)
        );
        assert_eq!(chip8.registers.pc, 0x200);
    }
    #[test]
    fn test_chip8_execute_set_delay_timer() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x0, 0x10);

        chip8.execute(Opcode::SetDelayTimer(0x0)).unwrap();

        assert_eq!(chip8.timers.get_delay_timer(), 0x10);
    }

    #[test]
    fn test_chip8_execute_set_index() {
        let mut chip8 = Chip8::new();

        chip8.execute(Opcode::SetIndex(0x300)).unwrap();

        assert_eq!(chip8.registers.i, 0x300);
    }

    #[test]
    fn test_chip8_execute_set_sound_timer() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x0, 0x10);

        chip8.execute(Opcode::SetSoundTimer(0x0)).unwrap();

        assert_eq!(chip8.timers.get_sound_timer(), 0x10);
    }
    #[test]
    fn test_chip8_execute_shift_left() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x0, 0b00101010);

        chip8.execute(Opcode::ShiftLeft(0x0)).unwrap();

        assert_eq!(chip8.registers.read_v(0x0), 0b01010100);
        assert_eq!(chip8.registers.read_v(0xF), 0x0);
    }

    #[test]
    fn test_chip8_execute_shift_left_overflow() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x0, 0b10000000);

        chip8.execute(Opcode::ShiftLeft(0x0)).unwrap();

        assert_eq!(chip8.registers.read_v(0x0), 0b00000000);
        assert_eq!(chip8.registers.read_v(0xF), 0x1);
    }
    #[test]
    fn test_chip8_execute_shift_right() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x0, 0b10101010);

        chip8.execute(Opcode::ShiftRight(0x0)).unwrap();

        assert_eq!(chip8.registers.read_v(0x0), 0b01010101);
        assert_eq!(chip8.registers.read_v(0xF), 0x0);
    }

    #[test]
    fn test_chip8_execute_shift_right_overflow() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x0, 0b00000001);

        chip8.execute(Opcode::ShiftRight(0x0)).unwrap();

        assert_eq!(chip8.registers.read_v(0x0), 0b00000000);
        assert_eq!(chip8.registers.read_v(0xF), 0x1);
    }
    #[test]
    fn test_chip8_execute_skip_if_equal_skips() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x0, 0x10);
        chip8.registers.pc = 0x200;

        chip8.execute(Opcode::SkipIfEqual(0x0, 0x10)).unwrap();

        assert_eq!(chip8.registers.pc, 0x202);
    }

    #[test]
    fn test_chip8_execute_skip_if_equal_not_skips() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x0, 0x10);
        chip8.registers.pc = 0x200;

        chip8.execute(Opcode::SkipIfEqual(0x0, 0x1)).unwrap();

        assert_eq!(chip8.registers.pc, 0x200);
    }
    #[test]
    fn test_chip8_execute_skip_if_reg_equal_skips() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x0, 0x10);
        chip8.registers.write_v(0x1, 0x10);
        chip8.registers.pc = 0x200;

        chip8.execute(Opcode::SkipIfRegEqual(0x0, 0x1)).unwrap();

        assert_eq!(chip8.registers.pc, 0x202);
    }

    #[test]
    fn test_chip8_execute_skip_if_reg_equal_not_skips() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x0, 0x10);
        chip8.registers.write_v(0x1, 0x20);
        chip8.registers.pc = 0x200;

        chip8.execute(Opcode::SkipIfRegEqual(0x0, 0x1)).unwrap();

        assert_eq!(chip8.registers.pc, 0x200);
    }
    #[test]
    fn test_chip8_execute_skip_if_not_equal_skips() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x0, 0x10);
        chip8.registers.pc = 0x200;

        chip8.execute(Opcode::SkipIfNotEqual(0x0, 0x1)).unwrap();

        assert_eq!(chip8.registers.pc, 0x202);
    }

    #[test]
    fn test_chip8_execute_skip_if_not_equal_not_skips() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x0, 0x10);
        chip8.registers.pc = 0x200;

        chip8.execute(Opcode::SkipIfNotEqual(0x0, 0x10)).unwrap();

        assert_eq!(chip8.registers.pc, 0x200);
    }

    #[test]
    fn test_chip8_execute_sub() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x0, 0x10);
        chip8.registers.write_v(0x1, 0x05);

        chip8.execute(Opcode::Sub(0x0, 0x1)).unwrap();

        assert_eq!(chip8.registers.read_v(0x0), 0x0B);
        assert_eq!(chip8.registers.read_v(0xF), 0x1);
    }

    #[test]
    fn test_chip8_execute_sub_no_borrow() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x0, 0x10);
        chip8.registers.write_v(0x1, 0x0F);

        chip8.execute(Opcode::Sub(0x0, 0x1)).unwrap();

        assert_eq!(chip8.registers.read_v(0x0), 0x01);
        assert_eq!(chip8.registers.read_v(0xF), 0x1);
    }

    #[test]
    fn test_chip8_execute_sub_borrow() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x0, 0x0F);
        chip8.registers.write_v(0x1, 0x10);
        chip8.execute(Opcode::Sub(0x0, 0x1)).unwrap();
        assert_eq!(chip8.registers.read_v(0x0), 0xFF);
        assert_eq!(chip8.registers.read_v(0xF), 0x0);
    }
    #[test]
    fn test_chip8_execute_store_bcd() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x0, 123);
        chip8.registers.i = 0x200;

        chip8.execute(Opcode::StoreBCD(0x0)).unwrap();

        assert_eq!(chip8.memory.read_byte(0x200), Ok(1));
        assert_eq!(chip8.memory.read_byte(0x201), Ok(2));
        assert_eq!(chip8.memory.read_byte(0x202), Ok(3));
    }
    #[test]
    fn test_chip8_execute_xor() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x0, 0b10101010);
        chip8.registers.write_v(0x1, 0b11001100);

        chip8.execute(Opcode::Xor(0x0, 0x1)).unwrap();

        assert_eq!(chip8.registers.read_v(0x0), 0b01100110);
    }

    #[test]
    fn test_chip8_execute_skip_if_reg_not_equal_skips() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x0, 0x10);
        chip8.registers.write_v(0x1, 0x20);
        chip8.registers.pc = 0x200;

        chip8.execute(Opcode::SkipIfRegNotEqual(0x0, 0x1)).unwrap();

        assert_eq!(chip8.registers.pc, 0x202);
    }

    #[test]
    fn test_chip8_execute_skip_if_reg_not_equal_not_skips() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x0, 0x10);
        chip8.registers.write_v(0x1, 0x10);
        chip8.registers.pc = 0x200;

        chip8.execute(Opcode::SkipIfRegNotEqual(0x0, 0x1)).unwrap();

        assert_eq!(chip8.registers.pc, 0x200);
    }
    #[test]
    fn test_chip8_execute_subn() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x0, 0x5);
        chip8.registers.write_v(0x1, 0x10);

        chip8.execute(Opcode::SubN(0x0, 0x1)).unwrap();

        assert_eq!(chip8.registers.read_v(0x0), 0xB);
        assert_eq!(chip8.registers.read_v(0xF), 0x1);
    }

    #[test]
    fn test_chip8_execute_subn_no_borrow() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x0, 0x0F);
        chip8.registers.write_v(0x1, 0x10);

        chip8.execute(Opcode::SubN(0x0, 0x1)).unwrap();

        assert_eq!(chip8.registers.read_v(0x0), 0x01);
        assert_eq!(chip8.registers.read_v(0xF), 0x1);
    }

    #[test]
    fn test_chip8_execute_subn_borrow() {
        let mut chip8 = Chip8::new();
        chip8.registers.write_v(0x0, 0x10);
        chip8.registers.write_v(0x1, 0x0F);

        chip8.execute(Opcode::SubN(0x0, 0x1)).unwrap();

        assert_eq!(chip8.registers.read_v(0x0), 0xFF);
        assert_eq!(chip8.registers.read_v(0xF), 0x0);
    }

    #[test]
    fn test_chip8_load_sprites() {
        let mut chip8 = Chip8::new();
        chip8.load_sprites();
        for (sprite_idx, sprite) in display::SPRITES.iter().enumerate() {
            for (byte_idx, &byte) in sprite.iter().enumerate() {
                let read_addr = display::SPRITE_START_ADDRESS + sprite_idx * sprite.len() + byte_idx;
                assert_eq!(chip8.memory.read_byte(read_addr), Ok(byte));
            }
        }
    }
}

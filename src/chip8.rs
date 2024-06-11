mod display;
mod input;
mod memory;
mod opcodes;
mod registers;
mod stack;
mod timers;

const PROGRAM_START_ADDRESS: usize = 0x200;

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
}

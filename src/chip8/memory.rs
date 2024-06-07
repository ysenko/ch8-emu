const MEMORY_SIZE: usize = 4096;

#[derive(Debug, PartialEq)]
pub enum MemoryError {
    AddressOutOfBounds,
}

pub struct Memory {
    memory: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: [0; MEMORY_SIZE],
        }
    }

    pub fn read_byte(&self, address: usize) -> Result<u8, MemoryError> {
        if address >= MEMORY_SIZE {
            Err(MemoryError::AddressOutOfBounds)
        } else {
            Ok(self.memory[address])
        }
    }


    pub fn write_byte(&mut self, address: usize, value: u8) -> Result<(), MemoryError> {
        if address >= MEMORY_SIZE {
            Err(MemoryError::AddressOutOfBounds)
        } else {
            self.memory[address] = value;
            Ok(())
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_byte() {
        let value = 0xAB;
        let address = 0x200;
        let mut memory = Memory::new();

        memory.write_byte(address, value);

        assert_eq!(memory.read_byte(address).unwrap(), value);
    }

    #[test]
    fn test_write_byte() {
        let address = 0x300;
        let value = 0xCD;
        let mut memory = Memory::new();

        let res = memory.write_byte(address, value);

        assert!(res.is_ok());
        assert_eq!(memory.read_byte(address), Ok(value));
    }

    #[test]
    fn test_read_byte_out_of_bounds() {
        let memory = Memory::new();
        let address = 0x5000;

        assert_eq!(memory.read_byte(address), Err(MemoryError::AddressOutOfBounds));
    }

    #[test]
    fn test_write_byte_out_of_bounds() {
        let address = 0x5000;
        let value = 0xEF;
        let mut memory = Memory::new();

        let result = memory.write_byte(address, value);

        assert_eq!(result, Err(MemoryError::AddressOutOfBounds));   
    }
}
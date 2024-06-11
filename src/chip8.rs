mod display;
mod input;
mod memory;
mod opcodes;
mod registers;
mod stack;
mod timers;

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chip8_new() {
        Chip8::new();
    }

    // Add more test cases here...
}

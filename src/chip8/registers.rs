use std::convert::From;

const DATA_REGISTER_COUNT: usize = 0xF;

pub struct Registers {
    v: [u8; DATA_REGISTER_COUNT],
    i: u16,
    pc: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            v: [0; DATA_REGISTER_COUNT],
            i: 0,
            pc: 0,
        }
    }

    pub fn read_v(&self, register: u8) -> u8 {
        self.v[register as usize]
    }

    pub fn write_v(&mut self, register: u8, value: u8) {
        self.v[register as usize] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registers_new() {
        let registers = Registers::new();

        assert_eq!(registers.v, [0; DATA_REGISTER_COUNT]);
        assert_eq!(registers.i, 0);
        assert_eq!(registers.pc, 0);
    }

    #[test]
    fn test_write_and_read_data_register() {
        let mut registers = Registers {
            v: [0; DATA_REGISTER_COUNT],
            i: 0,
            pc: 0,
        };

        for register_idx in 0..DATA_REGISTER_COUNT as u8 {
            let register_value = register_idx;

            registers.write_v(register_idx, register_value);

            assert_eq!(registers.read_v(register_idx), register_value);
        }
    }

    #[test]
    fn test_write_and_read_i_register() {
        let mut registers = Registers {
            v: [0; DATA_REGISTER_COUNT],
            i: 0,
            pc: 0,
        };
        let value = 0xABCD;

        registers.i = value;

        assert_eq!(registers.i, value);
    }

    #[test]
    fn test_write_and_read_pc_register() {
        let mut registers = Registers {
            v: [0; DATA_REGISTER_COUNT],
            i: 0,
            pc: 0,
        };
        let value = 0x1234;

        registers.pc = value;

        assert_eq!(registers.pc, value);
    }
}

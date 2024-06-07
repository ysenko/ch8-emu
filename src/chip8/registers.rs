const DATA_REGISTER_COUNT: usize = 16;

/// Represents the 16 data registers (V0 through VF) in Chip8.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataRegister {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VA,
    VB,
    VC,
    VD,
    VE,
    VF,
}

impl DataRegister {
    /// Converts the enum to its corresponding index in the data registers array.
    pub fn to_index(self) -> usize {
        self as usize
    }
    /// Converts the given index to its corresponding data register enum.
    pub fn from_index(index: usize) -> Option<DataRegister> {
        match index {
            0 => Some(DataRegister::V0),
            1 => Some(DataRegister::V1),
            2 => Some(DataRegister::V2),
            3 => Some(DataRegister::V3),
            4 => Some(DataRegister::V4),
            5 => Some(DataRegister::V5),
            6 => Some(DataRegister::V6),
            7 => Some(DataRegister::V7),
            8 => Some(DataRegister::V8),
            9 => Some(DataRegister::V9),
            10 => Some(DataRegister::VA),
            11 => Some(DataRegister::VB),
            12 => Some(DataRegister::VC),
            13 => Some(DataRegister::VD),
            14 => Some(DataRegister::VE),
            15 => Some(DataRegister::VF),
            _ => None,
        }
    }
}

pub struct Registers {
    v: [u8; DATA_REGISTER_COUNT],
    i: u16,
    pc: u16,
}

impl Registers {
    pub fn read_data_register(&self, register: DataRegister) -> u8 {
        self.v[register.to_index()]
    }

    pub fn write_data_register(&mut self, register: DataRegister, value: u8) {
        self.v[register.to_index()] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_register_to_index() {
        assert_eq!(DataRegister::V0.to_index(), 0);
        assert_eq!(DataRegister::V1.to_index(), 1);
        assert_eq!(DataRegister::V2.to_index(), 2);
        assert_eq!(DataRegister::V3.to_index(), 3);
        assert_eq!(DataRegister::V4.to_index(), 4);
        assert_eq!(DataRegister::V5.to_index(), 5);
        assert_eq!(DataRegister::V6.to_index(), 6);
        assert_eq!(DataRegister::V7.to_index(), 7);
        assert_eq!(DataRegister::V8.to_index(), 8);
        assert_eq!(DataRegister::V9.to_index(), 9);
        assert_eq!(DataRegister::VA.to_index(), 10);
        assert_eq!(DataRegister::VB.to_index(), 11);
        assert_eq!(DataRegister::VC.to_index(), 12);
        assert_eq!(DataRegister::VD.to_index(), 13);
        assert_eq!(DataRegister::VE.to_index(), 14);
        assert_eq!(DataRegister::VF.to_index(), 15);
    }

    #[test]
    fn test_data_register_from_index() {
        assert_eq!(DataRegister::from_index(0), Some(DataRegister::V0));
        assert_eq!(DataRegister::from_index(1), Some(DataRegister::V1));
        assert_eq!(DataRegister::from_index(2), Some(DataRegister::V2));
        assert_eq!(DataRegister::from_index(3), Some(DataRegister::V3));
        assert_eq!(DataRegister::from_index(4), Some(DataRegister::V4));
        assert_eq!(DataRegister::from_index(5), Some(DataRegister::V5));
        assert_eq!(DataRegister::from_index(6), Some(DataRegister::V6));
        assert_eq!(DataRegister::from_index(7), Some(DataRegister::V7));
        assert_eq!(DataRegister::from_index(8), Some(DataRegister::V8));
        assert_eq!(DataRegister::from_index(9), Some(DataRegister::V9));
        assert_eq!(DataRegister::from_index(10), Some(DataRegister::VA));
        assert_eq!(DataRegister::from_index(11), Some(DataRegister::VB));
        assert_eq!(DataRegister::from_index(12), Some(DataRegister::VC));
        assert_eq!(DataRegister::from_index(13), Some(DataRegister::VD));
        assert_eq!(DataRegister::from_index(14), Some(DataRegister::VE));
        assert_eq!(DataRegister::from_index(15), Some(DataRegister::VF));
        assert_eq!(DataRegister::from_index(16), None);
        assert_eq!(DataRegister::from_index(100), None);
    }

    #[test]
    fn test_write_and_read_data_register() {
        let mut registers = Registers {
            v: [0; DATA_REGISTER_COUNT],
            i: 0,
            pc: 0,
        };

        for index in 0..DATA_REGISTER_COUNT {
            let register = DataRegister::from_index(index).unwrap();

            registers.write_data_register(register, index as u8);

            assert_eq!(registers.read_data_register(register), index as u8);
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

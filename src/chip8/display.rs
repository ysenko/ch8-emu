pub const SPRITE_LEN: usize = 5;

pub type SpriteValue = Vec<u8>;

// Constants with predefined sprites for digits 0x0-0xF that will be loaded into RAM during the system boot.
pub const BUILT_IN_SPRITES: [[u8; 5]; 0xF] = [
    [0xF0, 0x90, 0x90, 0x90, 0xF0],
    [0x20, 0x60, 0x20, 0x20, 0x70],
    [0xF0, 0x10, 0xF0, 0x80, 0xF0],
    [0xF0, 0x10, 0xF0, 0x10, 0xF0],
    [0x90, 0x90, 0xF0, 0x10, 0x10],
    [0xF0, 0x80, 0xF0, 0x10, 0xF0],
    [0xF0, 0x80, 0xF0, 0x90, 0xF0],
    [0xF0, 0x10, 0x20, 0x40, 0x40],
    [0xF0, 0x90, 0xF0, 0x90, 0xF0],
    [0xF0, 0x90, 0xF0, 0x10, 0xF0],
    [0xE0, 0x90, 0xE0, 0x90, 0xE0],
    [0xF0, 0x80, 0x80, 0x80, 0xF0],
    [0xE0, 0x90, 0x90, 0x90, 0xE0],
    [0xF0, 0x80, 0xF0, 0x80, 0xF0],
    [0xF0, 0x80, 0xF0, 0x80, 0x80],
];

pub const SPRITE_START_ADDRESS: usize = 0x0;
pub const DISPLAY_WIDTH: usize = 128;
pub const DISPLAY_HEIGHT: usize = 64;
const BUFFER_WIDTH: usize = DISPLAY_WIDTH / 8;
const BUFFER_HEIGHT: usize = DISPLAY_HEIGHT;

#[derive(Debug, PartialEq)]
pub enum DisplayError {
    InvalidSprite(u8),
    InvalidDrawPosition(usize, usize),
}

#[derive(Debug)]
pub struct Display {
    pub width: usize,
    pub height: usize,

    buffer: [[u8; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl Display {
    pub fn new() -> Self {
        Display {
            width: DISPLAY_WIDTH,
            height: DISPLAY_HEIGHT,
            buffer: [[0; BUFFER_WIDTH]; BUFFER_HEIGHT],
        }
    }

    pub fn get_sprite_address(sprite: u8) -> Result<usize, DisplayError> {
        if sprite as usize > BUILT_IN_SPRITES.len() {
            Err(DisplayError::InvalidSprite(sprite))
        } else {
            Ok(SPRITE_START_ADDRESS + (sprite as usize * 5))
        }
    }

    pub fn clear(&mut self) {
        for row in self.buffer.iter_mut() {
            for pixel in row.iter_mut() {
                *pixel = 0;
            }
        }
    }

    pub fn draw_sprite(&mut self, row: usize, col: usize, sprite_value: &SpriteValue) -> bool {
        let mut erased = false;
        for (row_delta, &sprite_row) in sprite_value.iter().enumerate() {
            erased |= self.draw_sprite_row((row + row_delta) % BUFFER_HEIGHT, col, sprite_row);
        }
        erased
    }

    fn draw_sprite_row(&mut self, row: usize, col: usize, value: u8) -> bool {
        let row_idx = row % BUFFER_HEIGHT;
        let col_idx = col / 8;
        let start_bit_idx = col % 8;

        let (mask, _) = (0b1111_1111 as u8).overflowing_shl(start_bit_idx as u32);
        let (masked_value, _) = (value & mask).overflowing_shr(start_bit_idx as u32);

        let original_value = self.buffer[row_idx][col_idx];
        self.buffer[row_idx][col_idx] ^= masked_value;

        let mut erased = bit_erased(original_value, self.buffer[row_idx][col_idx]);

        if start_bit_idx != 0 {
            let (mask, _) = (0b1111_1111 as u8).overflowing_shr(start_bit_idx as u32);
            let (masked_value, _) = (value & mask).overflowing_shl((8 - start_bit_idx) as u32);
            let original_value = self.buffer[row_idx][(col_idx + 1) % BUFFER_WIDTH];

            self.buffer[row_idx][(col_idx + 1) % BUFFER_WIDTH] ^= masked_value;
            if !erased {
                erased = bit_erased(
                    original_value,
                    self.buffer[row_idx][(col_idx + 1) % BUFFER_WIDTH],
                );
            }
        }
        erased
    }
}

fn bit_erased(original: u8, current: u8) -> bool {
    for bit_idx in 0..8 {
        let was = original & 0x1 << bit_idx != 0;
        let now = current & 0x1 << bit_idx != 0;
        if was && !now {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let display = Display::new();

        assert_eq!(display.width, DISPLAY_WIDTH);
        assert_eq!(display.height, DISPLAY_HEIGHT);
    }

    #[test]
    fn test_buffer_initialized_with_zeroes() {
        let display = Display::new();
        for row in display.buffer.iter() {
            for &pixels in row.iter() {
                assert_eq!(pixels, 0);
            }
        }
    }

    #[test]
    fn test_get_sprite_address_valid_sprite() {
        let sprite = 0x0A;

        let expected_address = SPRITE_START_ADDRESS + (sprite as usize * 5);

        assert_eq!(Display::get_sprite_address(sprite), Ok(expected_address));
    }

    #[test]
    fn test_get_sprite_address_invalid_sprite() {
        let sprite = 0x10;

        assert_eq!(
            Display::get_sprite_address(sprite),
            Err(DisplayError::InvalidSprite(sprite))
        );
    }

    #[test]
    fn test_render_sprite_row() {
        let mut display = Display::new();
        let row = 0;
        let col = 0;
        let value = 0b1010_1010;

        let erased = display.draw_sprite_row(row, col, value);

        assert!(!erased);
        assert_eq!(display.buffer[row][col], value);
    }

    #[test]
    fn test_render_sprite_row_multiple_buffer_elements() {
        let mut display = Display::new();
        let row = 0;
        let col = 4;
        let value = 0b1010_0101;
        let expected_byte_1 = 0b0000_1010;
        let expected_byte_2 = 0b0101_0000;

        let erased = display.draw_sprite_row(row, col, value);

        assert!(!erased);
        assert_eq!(display.buffer[row][col / 8], expected_byte_1);
        assert_eq!(display.buffer[row][col / 8 + 1], expected_byte_2);
    }

    #[test]
    fn test_render_sprite_row_wraps_around_right_edge() {
        let mut display = Display::new();
        let row = 0;
        let col = DISPLAY_WIDTH - 4;
        let value = 0b1010_0101;
        let expected_byte_1 = 0b0000_1010;
        let expected_byte_2 = 0b0101_0000;

        let erased = display.draw_sprite_row(row, col, value);

        assert!(!erased);
        assert_eq!(display.buffer[row][col / 8], expected_byte_1);
        assert_eq!(display.buffer[row][0], expected_byte_2);
    }

    #[test]
    fn draw_sprite() {
        let mut display = Display::new();
        let sprite: SpriteValue = vec![
            0b1000_0000,
            0b1000_0000,
            0b1000_0000,
            0b1000_0000,
            0b1000_0000,
        ];
        let row = 0;
        let col = 0;

        let erased = display.draw_sprite(row, col, &sprite);

        assert!(!erased);
        for (row_idx, &sprite_row) in sprite.iter().enumerate() {
            assert_eq!(display.buffer[row_idx][0], sprite_row);
        }
    }

    #[test]
    fn draw_sprite_reports_erased_bit() {
        let mut display = Display::new();
        let initial_sprite: SpriteValue = vec![
            0b1000_0000,
            0b1000_0000,
            0b1000_0000,
            0b1000_0000,
            0b1000_0000,
        ];
        let new_sprite: Vec<u8> = vec![
            0b0000_0000,
            0b0000_0000,
            0b1000_0000,
            0b0000_0000,
            0b0000_0000,
        ];
        let row = 0;
        let col = 0;
        display.draw_sprite(row, col, &initial_sprite);

        let erased = display.draw_sprite(row, col, &new_sprite);

        assert!(erased);
        assert_eq!(display.buffer[row][col], 0b1000_0000);
        assert_eq!(display.buffer[row + 1][col], 0b1000_0000);
        assert_eq!(display.buffer[row + 2][col], 0b0000_0000); // Pixel erased
        assert_eq!(display.buffer[row + 3][col], 0b1000_0000);
        assert_eq!(display.buffer[row + 4][col], 0b1000_0000);
    }

    #[test]
    fn draw_sprite_wraps_around_bottom_edge() {
        let mut display = Display::new();
        let sprite: SpriteValue = vec![
            0b1000_0000,
            0b1000_0000,
            0b1000_0000,
            0b1000_0000,
            0b1000_0000,
        ];
        let row = BUFFER_HEIGHT - 3;
        let col = 0;

        let erased = display.draw_sprite(row, col, &sprite);

        assert_eq!(erased, false);
        assert_eq!(display.buffer[row][0], sprite[0]);
        assert_eq!(display.buffer[row + 1][0], sprite[1]);
        assert_eq!(display.buffer[row + 2][0], sprite[2]);
        assert_eq!(display.buffer[0][0], sprite[3]);
        assert_eq!(display.buffer[1][0], sprite[4]);
    }

    #[test]
    fn test_bit_erased_no_erased_bits() {
        let original = 0b1010_1010;
        let current = 0b1010_1010;
        assert_eq!(bit_erased(original, current), false);
    }

    #[test]
    fn test_bit_erased_some_erased_bits() {
        let original = 0b1010_1010;
        let current = 0b1010_0010;
        assert_eq!(bit_erased(original, current), true);
    }

    #[test]
    fn test_bit_erased_all_erased_bits() {
        let original = 0b1010_1010;
        let current = 0b0000_0000;
        assert_eq!(bit_erased(original, current), true);
    }

    #[test]
    fn test_clear() {
        let mut display = Display::new();
        display.draw_sprite(0, 0, &BUILT_IN_SPRITES[0].to_vec());

        display.clear();

        for row in display.buffer.iter() {
            for &pixel in row.iter() {
                assert_eq!(pixel, 0);
            }
        }
    }
}

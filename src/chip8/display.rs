// Constants with predefined sprites for digits 0x0-0xF that will be loaded into RAM during the system boot.
pub const SPRITES: [[u8; 5]; 0xF] = [
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

#[derive(Debug, PartialEq)]
pub enum DisplayError {
    InvalidSprite(u8),
}

pub struct Display;

impl Display {
    pub fn new() -> Self {
        Display {}
    }

    pub fn get_sprite_address(sprite: u8) -> Result<usize, DisplayError> {
        if sprite as usize > SPRITES.len() {
            Err(DisplayError::InvalidSprite(sprite))
        } else {
            Ok(SPRITE_START_ADDRESS + (sprite as usize * 5))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

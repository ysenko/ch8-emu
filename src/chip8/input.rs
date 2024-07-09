use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidKey(String),
}
#[derive(Debug)]
pub struct Input {
    key: Option<String>,
}

impl Input {
    pub fn new() -> Self {
        Input { key: None }
    }

    pub fn set_key(&mut self, key: &str) {
        self.key = Some(key.to_string());
    }

    pub fn get_key(&self) -> Option<&str> {
        if self.key.is_some() {
            Some(self.key.as_ref().unwrap())
        } else {
            None
        }
    }

    pub fn clear_key(&mut self) {
        self.key = None;
    }

    pub fn get_key_u8(&self) -> Result<Option<u8>, Error> {
        if self.key.is_none() {
            return Ok(None);
        }

        let key_str = self.key.as_ref().unwrap();
        match u8::from_str_radix(key_str.as_str(), 16) {
            Ok(key_u8) => Ok(Some(key_u8)),
            _ => Err(Error::InvalidKey(key_str.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_input() {
        let input = Input::new();
        assert_eq!(input.get_key(), None);
    }

    #[test]
    fn test_set_key() {
        let mut input = Input::new();
        input.set_key("A");
        assert_eq!(input.get_key(), Some("A"));
    }

    #[test]
    fn test_clear_key() {
        let mut input = Input::new();
        input.set_key("A");
        input.clear_key();
        assert_eq!(input.get_key(), None);
    }
    #[test]
    fn test_get_key_u8_valid_key() {
        let mut input = Input::new();
        input.set_key("1");
        assert_eq!(input.get_key_u8(), Ok(Some(1)));
    }

    #[test]
    fn test_get_key_u8_invalid_key() {
        let mut input = Input::new();
        input.set_key("G");
        assert_eq!(input.get_key_u8(), Err(Error::InvalidKey("G".to_string())));
    }

    #[test]
    fn test_get_key_u8_no_key() {
        let input = Input::new();
        assert_eq!(input.get_key_u8(), Ok(None));
    }
}

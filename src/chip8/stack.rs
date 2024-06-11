#[derive(Debug, PartialEq)]
pub enum StackError {
    StackOverflow,
    StackUnderflow,
}

const STACK_SIZE: usize = 16;

pub struct Stack {
    stack: [u16; STACK_SIZE], // Array to hold 16 levels of the stack
    sp: usize,                // Stack pointer to track the current level (0-15)
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            stack: [0; STACK_SIZE],
            sp: 0,
        }
    }

    // Pushes a value onto the stack, if there's space
    pub fn push(&mut self, value: u16) -> Result<(), StackError> {
        if self.sp >= STACK_SIZE {
            return Err(StackError::StackOverflow);
        };
        self.stack[self.sp] = value;
        self.sp += 1;

        Ok(())
    }

    // Pops a value off the stack, if it's not empty
    pub fn pop(&mut self) -> Result<u16, StackError> {
        if self.sp == 0 {
            return Err(StackError::StackUnderflow);
        };
        self.sp -= 1;
        Ok(self.stack[self.sp])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stack() {
        let stack = Stack::new();

        assert_eq!(stack.sp, 0);
        assert_eq!(stack.stack, [0; 16]);
    }

    #[test]
    fn test_push_and_pop() {
        let mut stack = Stack::new();
        let values = [0x123, 0x456, 0x789, 0xABC];

        for &value in values.iter() {
            assert!(stack.push(value).is_ok());
        }

        for &value in values.iter().rev() {
            assert_eq!(stack.pop(), Ok(value));
        }
    }
    #[test]
    fn test_push_full_stack() {
        let mut stack = Stack::new();
        for _ in 0..STACK_SIZE {
            assert!(stack.push(0x123).is_ok());
        }

        assert_eq!(stack.push(0x456), Err(StackError::StackOverflow));
    }

    #[test]
    fn test_pop_empty_stack() {
        let mut stack = Stack::new();

        assert_eq!(stack.pop(), Err(StackError::StackUnderflow));
    }
}

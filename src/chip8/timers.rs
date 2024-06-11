pub struct Timers {
    delay_timer: u8,
    sound_timer: u8,
}

impl Timers {
    pub fn new() -> Timers {
        Timers {
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn get_delay_timer(&self) -> u8 {
        self.delay_timer
    }

    pub fn set_delay_timer(&mut self, value: u8) {
        self.delay_timer = value;
    }

    pub fn get_sound_timer(&self) -> u8 {
        self.sound_timer
    }

    pub fn set_sound_timer(&mut self, value: u8) {
        self.sound_timer = value;
    }

    pub fn decrement_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_timers() {
        let timers = Timers::new();
        assert_eq!(timers.get_delay_timer(), 0);
        assert_eq!(timers.get_sound_timer(), 0);
    }

    #[test]
    fn test_set_delay_timer() {
        let mut timers = Timers::new();
        timers.set_delay_timer(5);
        assert_eq!(timers.get_delay_timer(), 5);
    }

    #[test]
    fn test_set_sound_timer() {
        let mut timers = Timers::new();
        timers.set_sound_timer(3);
        assert_eq!(timers.get_sound_timer(), 3);
    }

    #[test]
    fn test_decrement_timers() {
        let mut timers = Timers::new();
        timers.set_delay_timer(5);
        timers.set_sound_timer(3);

        timers.decrement_timers();
        assert_eq!(timers.get_delay_timer(), 4);
        assert_eq!(timers.get_sound_timer(), 2);

        timers.decrement_timers();
        assert_eq!(timers.get_delay_timer(), 3);
        assert_eq!(timers.get_sound_timer(), 1);

        timers.decrement_timers();
        assert_eq!(timers.get_delay_timer(), 2);
        assert_eq!(timers.get_sound_timer(), 0);

        timers.decrement_timers();
        assert_eq!(timers.get_delay_timer(), 1);
        assert_eq!(timers.get_sound_timer(), 0);

        timers.decrement_timers();
        assert_eq!(timers.get_delay_timer(), 0);
        assert_eq!(timers.get_sound_timer(), 0);
    }
}

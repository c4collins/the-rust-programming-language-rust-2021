pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("The guess must be between 1 and 100 but was: {}", value);
        }

        Guess { value }
    }
    pub fn value(&self) -> u32 {
        self.value
    }
}

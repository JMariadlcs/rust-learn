pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn new_bug(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic]
    fn greater_than_100_bug() {
        Guess::new_bug(200);
    }

    // pass because received expected string
    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn greater_than_100_2() {
        Guess::new(200);
    }

    // failed because expected panic string is different 
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100_3() {
        Guess::new(200);
    }
}
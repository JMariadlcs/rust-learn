// Run: cargo test

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // bug
    fn can_hold_bug(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}

// user assert_eq!
pub fn add_two(a: i32) -> i32 {
    a + 2
}

// custom failed message
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    use super::*;
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    // bug
    #[test]
    fn smaller_cannot_hold_larger_2() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold_bug(&larger));
    }

    // assert_eq!
    mod tests {
        use super::*;
    
        #[test]
        fn it_adds_two() {
            assert_eq!(4, add_two(2));
        }
    }

    // custom failed message
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol2"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
}





use std::fmt::Display;

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl PartialEq for Rectangle {
    fn eq(&self, other: &Self) -> bool {
        self.area() == other.area()
    }

    fn ne(&self, other: &Self) -> bool {
        self.area() != other.area()
    }
}

impl PartialOrd for Rectangle {
    fn ge(&self, other: &Self) -> bool {
        self.area() >= other.area()
    }

    fn gt(&self, other: &Self) -> bool {
        self.area() > other.area()
    }

    fn le(&self, other: &Self) -> bool {
        self.area() <= other.area()
    }

    fn lt(&self, other: &Self) -> bool {
        self.area() < other.area()
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.area().partial_cmp(&other.area())
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rectangle (width: {}, height: {})",
            self.width, self.height
        )
    }
}
impl From<&str> for Rectangle {
    fn from(s: &str) -> Self {
        let v: Vec<&str> = s.split(',').collect();
        let width = v[0].parse().unwrap();
        let height = v[1].parse().unwrap();
        Rectangle { width, height }
    }
}

impl From<(u32, u32)> for Rectangle {
    fn from(t: (u32, u32)) -> Self {
        Rectangle {
            width: t.0,
            height: t.1,
        }
    }
}

impl Iterator for Rectangle {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.width > 0 {
            self.width -= 1;
            Some(self.width)
        } else {
            None
        }
    }
}
pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

// Checking for Panics with should_panic
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            height: 9,
            width: 7,
        };

        let smaller = Rectangle {
            height: 5,
            width: 4,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            height: 9,
            width: 7,
        };

        let smaller = Rectangle {
            height: 5,
            width: 4,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_displays() {
        let rect = Rectangle {
            height: 9,
            width: 7,
        };

        assert_eq!(format!("{}", rect), "Rectangle (width: 7, height: 9)");
    }

    #[test]
    fn it_converts_from_str() {
        let rect: Rectangle = "7,9".into();
        assert_eq!(rect.width, 7);
        assert_eq!(rect.height, 9);
    }

    #[test]
    fn it_converts_from_tuple() {
        let rect: Rectangle = (7, 9).into();
        assert_eq!(rect.width, 7);
        assert_eq!(rect.height, 9);
    }

    #[test]
    fn it_compares() {
        let rect1 = Rectangle {
            height: 9,
            width: 7,
        };

        let rect2 = Rectangle {
            height: 5,
            width: 4,
        };

        assert!(rect1 > rect2);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic] // if this test does not panic, the test fails
                    // we can include an expected error like so:
                    //#[should_panic(expected = "Guess value must be between 1 and 100, got 200.")]
    fn should_panic_if_outside_range() {
        Guess::new(101);
    }

    // Using Result<T, E> in tests:
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err("two plus two does not equal four".to_string())
        }
    }
}

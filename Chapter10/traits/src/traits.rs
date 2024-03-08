use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize(&self) -> String;

    fn summarize_author(&self) -> String;

    // Default implementation
    // If a type implements the Summary trait, but does not implement the summarize_author method,
    // the default implementation will be used.
    fn read_more(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
}

pub trait Test {
    fn display(&self) -> String;
}

// Will implement the trait for all types that implement Summary and Debug
impl<T: Summary + Debug> Test for T {
    fn display(&self) -> String {
        format!("Display: {:?}", self)
    }
}

pub trait ToNumber {
    fn to_number(&self) -> i32;
}

impl<T> ToNumber for T {
    // Will implement the trait for all types
    fn to_number(&self) -> i32 {
        0
    }
}

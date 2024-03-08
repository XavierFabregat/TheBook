#![allow(unused)]

mod news_article;
mod traits;
mod tweet;

pub use news_article::NewsArticle;
use std::fmt::Display;
pub use traits::{Summary, Test, ToNumber};
pub use tweet::Tweet;

// This requires that the item passed implements the Summary trait
pub fn notify(item: &impl Summary) {
    // That way we can call the summarize method on the item
    println!("Breaking news! {}", item.summarize());
}

// This is equivalent to the above
pub fn notify_generic<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// We can specify multiple trait bounds with the + syntax
pub fn notify_multiple(item: &(impl Summary + Display)) {
    // ...
}

// This is equivalent to the above
pub fn notify_multiple_generic<T: Summary + Display>(item: &T) {
    // ...
}

// We can also use where clauses to specify trait bounds
// This is useful when we have a lot of trait bounds
pub fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Display,
{
    // ...
    0
}

// We can also return a type that implements a trait
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// Using Trait Bounds to Conditionally Implement Methods

pub struct Pair<T> {
    pub x: T,
    pub y: T,
}

impl<T> Pair<T> {
    // We can call this method on any Pair regardless of the type
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}

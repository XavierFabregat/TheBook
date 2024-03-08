use traits::{NewsArticle, Pair, Summary, Test, ToNumber, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize_author());
    println!("Display: {}", article.display());

    let pair = Pair { x: 1, y: 2 };

    pair.cmp_display(); // we can only call this method if T implements Display and PartialOrd as i32 does

    println!("{}", pair.to_number()); // we can call this method on anything since the trait is implemented for all types
}

// We can also conditionally implement methods on a type depending on whether the type implements a trait
// This is useful when we want to provide additional functionality for types that implement a trait

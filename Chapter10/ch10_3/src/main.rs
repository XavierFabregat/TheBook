#![allow(unused)]
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    let i;

    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        i = ImportantExcerpt {
            part: first_sentence, // The lifetime of the reference to first_sentence is the same as the lifetime of the ImportantExcerpt instance
        };
    } // first_sentence goes out of scope here, but the ImportantExcerpt instance does not
      // println!("{}", i.part); <== This would not work because first_sentence is out of scope and the reference would be invalid
}

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.chars().count() >= str1.chars().count() {
        str1
    } else {
        str2
    }
}

// If the value returned in the function is generated inside the function
// this would not work because the value would be dropped when the function ends
// fn invalid_longest<'a>() -> &'a str {
//     let result = String::from("long string is long");
//     result.as_str() // Cannot return a reference to a value that is dropped
// }

// Lifetime annotations in struct definitions
struct ImportantExcerpt<'a> {
    part: &'a str, // We need to specify the lifetime of the reference bc the struct does not own the data
}

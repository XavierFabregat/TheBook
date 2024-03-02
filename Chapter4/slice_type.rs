fn main() {
    let mut phrase = String::from("Lorem ipsum dolor sit amet");
    // let first = first_word_slice(&phrase);
    // println!("The first word is: {}", first);
    let third = get_nth_word(&phrase, 3);
    println!("The third word is: {third}");

    phrase.clear();

    // println!("The third word is: {third}");

    // ARRAY SLICES
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // This is a slice of the array a
    println!("The slice is: {:?}", slice);
    assert_eq!(slice, &[2, 3]); // It will panic if the assertion is false
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         println!("{} and {}", i, item);
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len();
// }

// This presents a problem: we want to return a slice of the original string,
// but we can't return a reference to a part of the string because it would be dropped when the function ends.
// We can't return a String because we don't want to allocate memory.
// We can return a slice, which is a reference to a part of a string, but we need to change the function signature to do so.

// fn first_word_slice(s: &String) -> &str {
//     // &str signifies a string slice
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         println!("{i} and {}", item as char);
//         if item as char == ' ' {
//             return &s[0..i];
//         }
//     }

//     return &s[..];
// }

//This signature could be improved by using &str instead of &String bc that way it would accept both String and &str
// If we have a string slice, we can pass it to this function. If we have a String, we can pass a reference to the String to this function.
// Like so:
// let my_string = String::from("hello world");
// let word = first_word(&my_string[..]); == let word = first_word(&my_string);
// let my_string_literal = "hello world";
// let word = first_word(&my_string_literal[..]);
// Since string literals are string slices already, we can pass them directly to the function.
// let word = first_word(my_string_literal);
fn get_nth_word(s: &String, n: i8) -> &str {
    let bytes = s.as_bytes();
    let mut counter: i8 = 1; // we start at one since we are counting words not indexes
    let mut start_index: usize = 0;
    let mut end_index: usize = s.len();

    for (i, &item) in bytes.iter().enumerate() {
        if item as char == ' ' && counter != n {
            start_index = i;
            counter += 1;
        } else if item as char == ' ' && counter == n {
            end_index = i;
            break;
        }
    }
    return &s[start_index..end_index];
}

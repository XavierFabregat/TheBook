fn main() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    // if we try to use s here, we’ll get a compile error
    // This will result in a compile error:
    // value borrowed here after move, to avoid this error, we can use the clone method
    // println!("{}", s);

    let z = 5; // z comes into scope

    makes_copy(z); // z would move into the function,
                   // but i32 is Copy, so it’s okay to still
                   // use z afterward
    let b = String::from("World"); // b comes into scope
    let c = takes_and_gives_back(b); // b is moved into the function and returned
    println!("{}", c); // c is valid here

    let (string, length) = calculate_length(c); // c is moved into the function and returned
    println!("The length of '{}' is {}", string, length); // string is valid here
} // Here, z goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn takes_and_gives_back(a_string: String) -> String {
    println!("{}", a_string);
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len is the length of the string
    (s, length)
}

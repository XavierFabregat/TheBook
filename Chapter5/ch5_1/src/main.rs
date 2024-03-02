#![allow(dead_code)]
#![allow(unused_variables)]
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@rust.rs"),
        sign_in_count: 1,
    };
    println!("{} is active: {}\n", user1.username, user1.active);

    user1.active = false;

    println!("{} is active: {}\n", user1.username, user1.active);

    let mut user2 = build_user(String::from("user2@rust.rs"), String::from("user2"));

    println!("{} is active: {}\n", user2.username, user2.active);

    user2.active = false;

    println!("{} is active: {}\n", user2.username, user2.active);

    // Struct update syntax
    let user3 = User {
        username: String::from("user3"),
        ..user2
    };

    // since the ownership of user2.email is moved to user3, user2 is not accessible
    // This will result in a compilation error
    // println!("{:?}", user2);

    // Tuple Structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email,
        username,
        sign_in_count: 1,
    }
}

// Tuple Structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Condition must be a boolean
    // if number {
    //     println!("number was three");
    // }

    // this will result in a compile error:  expected `bool`, found integer

    // Multiple conditions

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let statement (ternary operator)
    let condition = true;
    let number = if condition { 5 } else { 6 }; // number will be 5

    // This will result in a compile error:  mismatched types
    // let number = if condition { 5 } else { "six" };

    //  $ cargo run
    //    Compiling branches v0.1.0 (file:///projects/branches)
    //    error[E0308]: `if` and `else` have incompatible types
    //    --> src/main.rs:4:44
    //   |
    // 4 |     let number = if condition { 5 } else { "six" };
    //   |                                 -          ^^^^^ expected integer, found `&str`
    //   |                                 |
    //   |                                 expected because of this

    // For more information about this error, try `rustc --explain E0308`.
    // error: could not compile `branches` due to previous error

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

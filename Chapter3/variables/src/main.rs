fn main() {
    let x = 5; // non mutable variable
    println!("The value of x is: {x}");
    x = 6; // error: cannot assign twice to immutable variable `x`
    println!("The value of x is: {x}");


    // FIX

    let mut x = 5; // mutable variable
    println!("The value of x is: {x}");
    x = 6; // no error
    println!("The value of x is: {x}");


    // Constants
    // Must be annotated with a type
    // Can be declared in any scope, including the global scope
    // Cannot be set to the result of a function call or any other value that is computed at runtime
    const MAX_POINTS: u32 = 100_000;
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Difference between mut and shadowing
    // Mut: We can change the value of a variable but not its type
    // Shadowing: We can change the type of a variable
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len(); // Allowed because we are changing the type of the variable

    let mut spaces = "   ";
    spaces = spaces.len(); // Error: expected `&str`, found `usize`
}
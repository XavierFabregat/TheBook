fn main() {
    println!("Hello, world!");
    another_function(2_000_000_000);
    another_function(-2_000_000_000);
    another_function(42);

    with_multiple_parameters(42, 'c');

    let x = five();
    println!("The value of x is: {x}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn with_multiple_parameters (value: i32, unit_label: char) {
    println!("The value is: {value} {unit_label}");
}

// function with a return value
fn five() -> i32 {
    return 5;
}


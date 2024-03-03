fn main() {
    let config_max = Some(3u8);

    match config_max {
        Some(max) => {
            println!("The maximum is configured to be: {}", max);
        }
        _ => {}
    }

    // We can write this more concisely using if let:
    if let Some(max) = config_max {
        println!("The maximum is configured to be: {}", max);
    } else {
        println!("No maximum configured");
    }
}

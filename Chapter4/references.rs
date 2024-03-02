fn main() {
    let s1 = String::from("Hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len); // s1 is valid since we haven't moved it
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

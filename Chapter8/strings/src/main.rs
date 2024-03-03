fn main() {
    let s = "Stringy thingy";
    let S = s.to_string();

    // When we concatenate two strings, we create a new string
    // the first string is moved into the new string and is no longer valid
    // This is because the + operator is defined to take ownership of the first string
    // and the rest of strings are borrowed
    let s1 = "Hello".to_string();
    let s2 = "World".to_string();
    let s3 = s1 + " " + &s2;
    println!("{}", s3);

    // When concatanating strings, we can use the format! macro
    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    let tic_tac_toe = format!("{tic}-{tac}-{toe}");
    println!("{tic_tac_toe}");
}

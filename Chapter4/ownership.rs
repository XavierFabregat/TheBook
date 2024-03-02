#![allow(unused)]
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
    // This will result in a compile error:  value borrowed here after move
}

#![allow(unused)]
fn main() {
    let home = IPAdressKind::V4(127, 0, 0, 1);
    let loopback = IPAdressKind::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
}

enum IPAdressKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("{:?}", self)
    }
}

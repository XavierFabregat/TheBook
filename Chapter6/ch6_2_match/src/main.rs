#![allow(unused)]

use std::slice::RSplit;
fn main() {
    let coin = Coin::Quarter(USState::Alabama);
    println!("Value in cents: {}", value_in_cents(coin));

    let five = Some(5);
    let five_plus_one = plus_one(five);

    #[derive(Debug)]
    enum Result {
        Number(i32),
        Text(String),
    }
    let result: Result = match five_plus_one {
        None => {
            println!("NONE");
            Result::Number(0)
        }
        Some(i) => {
            println!("Value is {i}");
            Result::Text(String::from("Some"))
        }
    };

    match result {
        Result::Number(i) => println!("Number: {}", i),
        Result::Text(s) => println!("Text: {}", s),
    }
}

#[derive(Debug)]
enum USState {
    Alabama,
    Alaska,
    // ... etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            match state {
                USState::Alabama => println!("Alabama State Quarter"),
                USState::Alaska => println!("Alaska State Quarter"),
                _ => (),
            }
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => {
            println!("No number provided!");
            None
        }
        Some(i) => Some(i + 1),
    }
}

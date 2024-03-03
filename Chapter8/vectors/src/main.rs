#![allow(dead_code)]

use std::vec::{self, Splice};
fn main() {
    // We annotate the type since we're not giving rust any initial values to infer the type from
    let v1: Vec<i32> = Vec::new();
    println!("v: {:?}", v1);

    // We can use the vec! macro to create a vector and populate it with values
    let mut v2 = vec![1, 2, 3];

    // to update a vector we can use the push method:
    v2.push(4);
    println!("v: {:?}", v2);

    // There are two ways to access elements in a vector, using the index or using the get method
    let third = &v2[2]; // We use the & to create a reference to the value, otherwise we would be moving the value out of the vector. For types that implement the Copy trait (like i32), this will not move the value out of the vector but instead create a copy. So in this case it is not strictly necessary to use the &. for mor info see: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
    println!("The third element is {}", third);

    // We can also use the get method to access elements in a vector
    // That returns an Option<&T>
    let third = v2.get(2);
    // We can see that third is of type Option<&i32>
    // Now we can use the match expression to handle the Option
    match third {
        Some(value) => println!("The third element is {}", value),
        None => println!("There is no third element"),
    }

    // If we created a vector of structs:
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let mut points = Vec::new();
    points.push(Point { x: 1, y: 2 });
    points.push(Point { x: 3, y: 4 });
    points.push(Point { x: 5, y: 6 });

    //Now if we wanted to grab an element of the vector and we didn't use the & to create a reference
    // we would be moving the value out of the vector, and that would create an invalid state for the vector
    // so it would not let us, instead we need to create a reference to the value
    let first = &points[0];
    println!("The first point is {:?}", first);

    // The reason we have the two ways to access the elements (get method and index)
    // is to choose how to handle invalid indexes

    let v3 = vec![1, 2, 3, 4, 5];
    // let _does_not_exist = &v3[100]; // This will panic
    let _does_not_exist = v3.get(100); // This will return None

    // another thing to have in mind is the immutable borrow and mutable borrow rule of rust:
    let mut v4 = vec![1, 2, 3, 4, 5];
    // immutable borrow
    let first = &v4[0];
    // v4.push(6); // mutable borrow <== this is an immutable borrow since the push method take the &mut self parameter
    println!("The first element is: {}", first); // This will not compile <== This will create an error, since we have both a mutable and immutable borrow at the same time
                                                 // This is a good thing, since the vectors are stored in the heap, if we have an immutable borrow pointing to the first element of the vector, then we update the vector it self, it might need to be reallocated in the heap if the current capacity is not enough, making the first immutable borrow invalid.

    // We can also iterate over the elements of a vector
    let v5 = vec![100, 32, 57];
    for i in &v5 {
        println!("{i}");
    }

    // we can also iterate over mutable references to each element
    // this does not break the mutable borrow rule since
    // each mutable reference is only valid within the current iteration
    let mut v6 = vec![100, 32, 57];
    for i in &mut v6 {
        *i += 50;
    }

    // We can also use an enum to store multiple types in a vector
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let mut row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // We can iterate over it and perform actions using the match expression
    for cell in &mut row {
        match cell {
            SpreadsheetCell::Int(number) => *number *= 2,
            SpreadsheetCell::Float(float) => *float *= 2.0,
            SpreadsheetCell::Text(string) => string.push_str(", red"),
        }
    }

    println!("row: {:?}", row);

    // Like any other struct, dropping a vector will also drop all its elements
    // and if the elements are themselves vectors, they will also be dropped
    // and so on, until all elements are dropped
    {
        let v7 = vec![1, 2, 3, 4, 5];
        // do stuff with v7
    } // <- v7 goes out of scope and is dropped here
}

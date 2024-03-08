#![allow(unused)]
fn main() {
    let v_num = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result_num = largest_i32(&v_num);

    println!("The largest number in {:?} is {}", &v_num, result_num);

    let v_char = vec!['y', 'm', 'a', 'q'];

    let result_char = largest_char(&v_char);

    println!("The largest char in {:?} is {}", &v_char, result_char);
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// We can use generics to make the above code more generic
// This will not compile because rust does not know if T can be compared
// For now to make it work we need to add the trait bound ==> T: PartialOrd
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// We can also use Generics in Structs and Enums
struct Point<T> {
    y: T,
    x: T,
}

fn make_point() {
    let p = Point { x: 5, y: 10 };
    let p2 = Point { x: 5.0, y: 10.0 };
}

enum Option<T> {
    Some(T),
    None,
}

fn make_option() {
    let some_number = Option::Some(5);
    let some_string = Option::Some("a string");
    let absent_number: Option<i32> = Option::None;
}

// We can also use Generics in Methods
struct Point2<T> {
    x: T,
    y: T,
}

impl<T> Point2<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn make_point2() {
    let p = Point2 { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}

// We can declare methods for specific types in a generic struct
impl Point2<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn make_point3() {
    let p = Point2 { x: 5.0, y: 10.0 };
    println!("Distance from origin: {}", p.distance_from_origin());

    let p2 = Point2 { x: 5, y: 10 };
    // This will not compile because the method distance_from_origin is only defined for Point2<f32>
    // println!("Distance from origin: {}", p2.distance_from_origin());
}

fn mystery<T>(x: T) -> T {
    x
}

fn make_mystery() {
    let x = mystery(3);
}

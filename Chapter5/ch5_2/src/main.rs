fn main() {
    let width = 30;
    let height = 50;
    let area = calc_area(width, height);
    println!("The area of the rectangle is {area} square pixels.");

    let rect_a = (30, 50);
    let area_a = calc_area_tuple(rect_a);
    println!("The area of the tuple duple supple rectangle is {area_a} square pixels.");

    let rect_b = Rectangle {
        width: 30,
        height: 50,
    };
    let area_b = calc_area_struct(&rect_b);
    println!("The area of the struct plucked rectangle is {area_b} square pixels.");
}

fn calc_area(width: u32, height: u32) -> u32 {
    width * height
}

// Refactor with tuples:

fn calc_area_tuple(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}

//Refactor to Struct

struct Rectangle {
    width: u32,
    height: u32,
}

fn calc_area_struct(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}

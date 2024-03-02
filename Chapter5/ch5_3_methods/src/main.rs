fn main() {
    let rect = Rectangle {
        height: 30,
        width: 50,
    };

    let rect_b = Rectangle {
        height: 20,
        width: 30,
    };

    let rect_c = Rectangle {
        height: 40,
        width: 60,
    };
    println!(
        "The area of a rectangle with height {} and width {} is {} square units.",
        rect.height,
        rect.width,
        rect.area()
    );

    println!("Rect B can fit in Rect? {}", rect.can_hold(&rect_b));
    println!("Rect C can fit in Rect? {}", rect.can_hold(&rect_c));

    let new_square = Rectangle::square(50); // This is how to call associated functions
    println!(
        "The area of a square with size 50 is {} square units.",
        new_square.area()
    )
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // &self is short for  self: &Self
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        let mut result = false;
        if self.width > rect.width && self.height > rect.height {
            result = true
        }
        result
    }

    fn _can_hold_better(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated functions
    // they don't take &self as first param, since they don't actually need it
    // In this functions Self is an Alias for the type that appears after the impl keyword, in this case Rectangle
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// Since the Breakfast struct has a private field
// we need to provide a public associated function to create an instance of Breakfast
// and optionally a public method to access the private field
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("Peaches"),
        }
    }

    pub fn seasonal_fruit(&self) -> &str {
        &self.seasonal_fruit
    }
}

// In contrast, if we make an enum public, all of its variants are then public
#[derive(Debug)]
pub enum Appetizer {
    Soup,
    Salad,
}

#![allow(unused)]
mod back_of_house;
mod front_of_house;

// We can use the use keyword to bring a module into scope
// This is similar to creating a symbolic link in the file system
// This allows us to use the functions in the module as if they were defined in the current scope
pub use front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_wait_list();
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // we can change the toast field:
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please.", meal.toast);

    //Since seasonal_fruit is private we cannot modify it or access it:
    // This throws the error ==> field `seasonal_fruit` of`Breakfast` is private
    // meal.seasonal_fruit = String::from("Papaya");
    // println!("The seasonal fruit is {}.", meal.seasonal_fruit);

    // We can access the seasonal_fruit field using the public get_seasonal_fruit method:
    println!("The seasonal fruit is {}.", meal.seasonal_fruit());

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    println!("What is this enums {:?} and {:?}", order1, order2);
}

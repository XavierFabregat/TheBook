use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Yellow"), 10);
    scores.insert(String::from("Blue"), 10);

    // Accessing values inside the HashMap using get method
    let team_name = String::from("Yellow");
    let score = scores.get(&team_name);

    // score wil be Option<&V> type so we can use match to get the value
    match score {
        Some(value) => println!("The score of {} is {}", team_name, value),
        None => println!("No score found for {}", team_name),
    }

    // Another option we have is to unwrap the value:
    let score = scores.get(&team_name).copied(); // This will convert Option<&V> to Option<V>
    let score = score.unwrap_or(0); // This will unwrap the value or return 0 if the value is None

    println!("The score of {} is {}", team_name, score);

    // unwrap_or(else) is a method that returns the value if it's Some or the value passed as argument if it's None
    // it is similar to doing:
    let score = match scores.get(&team_name) {
        Some(value) => *value,
        None => 0, // Here is the value of the else
    };

    println!("The score of {} is {}", team_name, score);

    // We can iterate over the HashMap using a for loop
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // HashMaps and ownership
    // For values that implement the Copy trait, the values are copied into the HashMap
    // For values that implement the Drop trait, the values are moved into the HashMap

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point
    // println!("{}: {}", field_name, field_value); // This will cause an error

    // If we insert references to values into the HashMap, the values won't be moved into the HashMap
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);
    // field_name and field_value are still valid at this point
    println!("{}: {}", field_name, field_value); // This will work
}

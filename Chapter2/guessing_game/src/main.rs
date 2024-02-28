use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // We add mut to make the variable mutable
        // (in JS we would just use let, in opposite to const, to make it mutable)
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim() == "quit" {
            println!("You quit the game, the number was: {secret_number}");
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You guessed to low!"),
            Ordering::Greater => println!("You guessed to high!"),
            Ordering::Equal => {
                println!("The double champ champ does whatever he wants, congrats!");
                break;
            }
        }
    }
}

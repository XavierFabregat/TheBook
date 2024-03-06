use std::fs::{self, File, OpenOptions};
use std::io::{self, ErrorKind, Read, Write};

fn main() {
    let greeting_file_result = OpenOptions::new()
        .write(true)
        .append(true)
        .read(true)
        .open("hello.txt");

    let mut greeting_file = match greeting_file_result {
        Ok(file) => {
            println!("The file is open: {:?}", file);
            file
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(e) => panic!("Problem creating the file {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    let str = "Xavier Fabregat.\n";

    match greeting_file.write(str.as_bytes()) {
        Ok(_) => println!("Write to file successfully"),
        Err(e) => println!("Failed to write to file: {:?}", e),
    }

    // We can also use the unwrap/expect method to handle the error

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .read(true)
        .open("bye.txt")
        .expect("No bye.txt file found."); // If the file does not exist, the program will panic

    let str = "Goodbye, World.\n";
    file.write(str.as_bytes()).expect("Failed to write to file");
    println!("Write to file successfully: {:?}", file);

    match read_username_from_file() {
        Ok(val) => println!("{val}"),
        Err(e) => println!("{:?}", e),
    }

    match read_username_from_file_shorthand() {
        Ok(val) => println!("{val}"),
        Err(e) => println!("{:?}", e),
    }

    match read_username_from_file_shortest() {
        Ok(val) => println!("{val}"),
        Err(e) => println!("{:?}", e),
    }
}

// Error propagation:
fn read_username_from_file() -> Result<String, io::Error> {
    let file_result = File::open("hello.txt");
    let mut file = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// This error propagation is so common that there is a short hand operator ==>  ?
fn read_username_from_file_shorthand() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

// The ? operator will return the Ok value if the operation is successful,
// and will return the Err value if the operation fails, but the error will be of the type
// specified in the return type of the function.

// We can even shorten the code further by using the read_to_string method of the fs module
fn read_username_from_file_shortest() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

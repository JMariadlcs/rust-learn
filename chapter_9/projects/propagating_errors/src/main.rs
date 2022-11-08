
use std::fs::{self, File};
use std::io::{self, Read};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, Propagating Errors!");
    Ok(())

}

fn read_username_from_file_longway() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e), // Return the error -> not panic
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// Same with shortcut
fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?; // '?'
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// shortest
fn read_username_from_file_shortest() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// even more shorter
fn read_username_from_file_more_shorter() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

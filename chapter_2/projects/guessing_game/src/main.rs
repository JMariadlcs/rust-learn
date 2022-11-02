use std::io; 

fn main() {
    println!("Welcome to The Guessing Game");

    println!("Please give an input");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)  
        .expect("Failed to read line");

    println!("Your guess was: {}", guess);

}

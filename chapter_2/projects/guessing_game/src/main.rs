use std::io; 
use rand::Rng;

fn main() {
    println!("Welcome to The Guessing Game");

    // Generate Secret Number
    // rand::tread_rng indicates that we are generating a random num in the actual thread
    // gen_range indicates range of the random number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The generated secret number is: {}", secret_number);
    println!("Please give an input");

    let mut guess = String::new(); // mut -> mutable variable 
                                   // variables are immutable by default

    io::stdin()
        .read_line(&mut guess) // variable used here must be "mut"
                               // '&' is a "reference" access to memory to data directly (like a pointer in C)
        .expect("Failed to read line");

    println!("Your guess was: {}", guess);

}

use std::io; 

fn main() {
    println!("Welcome to The Guessing Game");

    println!("Please give an input");

    let mut guess = String::new(); // mut -> mutable variable 
                                   // variables are immutable by default

    io::stdin()
        .read_line(&mut guess) // variable used here must be "mut"
                               // '&' is a "reference" access to memory to data directly (like a pointer in C)
        .expect("Failed to read line");

    println!("Your guess was: {}", guess);

}

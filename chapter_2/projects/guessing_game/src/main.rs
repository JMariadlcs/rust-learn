use std::io; 
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to The Guessing Game");

    // Generate Secret Number
    // rand::tread_rng indicates that we are generating a random num in the actual thread
    // gen_range indicates range of the random number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The generated secret number is: {}", secret_number);

    loop {
        println!("Please give an input");

        let mut guess = String::new(); // mut -> mutable variable 
                                    // variables are immutable by default

        io::stdin()
            .read_line(&mut guess) // variable used here must be "mut"
                                // '&' is a "reference" access to memory to data directly (like a pointer in C)
            .expect("Failed to read line");
        
        // SHADOWING -> redefine 'guess' another time to change its type (u32)
        // trim() eliminates blank spaces at the starting and end part of a String
        // parse() converts String to specified type (u32)
        let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please input a number.");
                    continue
                }
            };

        println!("Your guess was: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Guess is too small."),
            Ordering::Greater => println!("Guess is to high"),
            Ordering::Equal => { 
                println!("Correct guess!!");
                break 
            }
        }
    }

}

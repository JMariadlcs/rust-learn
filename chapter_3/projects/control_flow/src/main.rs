fn main() {
    println!("Hello, Control Flows!");

    // If expressions
    let returned: bool = grater_than_number(72);
    println!("Comparison returned {}", returned);
    // Match function
    // match_function(5);

    // If in an expression
    let number = -4;
    let is_positive = if number > 0 { true } else { false };
    println!("Value is positive: {}", is_positive)
}

// If expressions
fn grater_than_number(x: i32) -> bool {
    if x > 5 {
        true
    } else {
        false 
    }
}

// Using Match statement (another way of comparison)
// Not correct (I am not sure how to implement with 'match' yet.)
/* 
fn match_function(number: u32) {
    match number {
        number % 4 == 0 => println!("number is divisible by 4");
        number % 3 == 0 => println!("number is divisible by 3");
        number % 2 == 0 => println!("number is divisible by 2");
        _ => println!("number is not divisible by 4, 3, or 2");
    }
}
*/

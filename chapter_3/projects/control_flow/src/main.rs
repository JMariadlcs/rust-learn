fn main() {
    println!("Hello, Control Flows!");

    // IF EXPRESSIONS
    let returned: bool = grater_than_number(72);
    println!("Comparison returned {}", returned);
    // Match function
    // match_function(5);

    // If in an expression
    let number = -4;
    let is_positive = if number > 0 { true } else { false };
    println!("Value is positive: {}", is_positive);

    // LOOPS
    // simple
    simple_loop();
    // while
    while_loop();
    // for
    for_loop();
}

// IF EXPRESSIONS
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

// LOOPS
// simple loop
fn simple_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

// While loop
fn while_loop() {
    let mut number = 3; // number must be mut because it is changed inside while loop

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

// For loop
fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

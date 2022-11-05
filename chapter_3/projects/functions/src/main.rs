fn main() {
    println!("Hello, Functions project!");

    another_function();
    another_function2(5);
    print_labeled_measurement(5, 'h');
    statements();

    // functions with return values
    let x = five();
    println!("The value of RETURNED x is: {x}");
    let x = plus_one(5);
    println!("The value of input + 1 is: {x}");
}

fn another_function() {
    println!("Another function.");
}

// Functions with Parameters 
fn another_function2(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Statements and expressions
fn statements() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}

// Functions with Return Values
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
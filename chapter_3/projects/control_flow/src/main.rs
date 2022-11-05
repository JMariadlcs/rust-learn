fn main() {
    println!("Hello, Control Flows!");

    // If expressions
    let returned: bool = grater_than_number(72);
    println!("Comparison returned {}", returned);
}

// If expressions
fn grater_than_number(x: i32) -> bool {
    if x > 5 {
        true
    } else {
        false 
    }
}
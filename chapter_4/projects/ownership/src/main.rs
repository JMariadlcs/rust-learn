fn main() {
    println!("Hello, Ownership!");
    let mut s = String::from("hello"); // 'String' is now stored on the heap!
                                               // this means is it now not STATIC -> it can be increased
                                               // '::from' is requesting the needed memory
    println!("The string is {}", s);

    // If we want to modify the String on the heap
    s.push_str(", world!");
    println!("The string is {}", s);
    // IMPORTANT -> The allocated memory is automatically freed when the variable is out of scope!
}

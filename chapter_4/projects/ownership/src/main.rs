fn main() {
    println!("Hello, Ownership!");
    let mut s = String::from("hello"); // 'String' is now stored on the heap!
                                               // this means is it now not STATIC -> it can be increased
                                               // '::from' is requesting the needed memory
    println!("The string is {}", s);

    // If we want to modify the String on the heap
    s.push_str(", world!");
    println!("The string is {}", s);
    // IMPORTANT -> The allocated memory is AUTOMATICALLY freed when the variable is out of scope!

    // ----- MOVE ------

    // Integers are simple values stored on the STACK so they can be copied 
    let x = 5;
    let y = x;
    println!("The var are {}, {}", x, y); // Both are valid and accesible because are stored on the STACK
    
    // String types are stored on the HEAP not on the STACK
    let s1 = String::from("hello");
    let s2 = s1; // When s1 is copied on s2 -> s1 is NO LONGER VALID // This is called "MOVE"
    // println!("{}, world!", s1); -> This results on an error because s1 IS NOT LONGER VALID
    println!("The var is {}", s2);
    

    // ----- CLONE ------
    // By cloning, s1 would be still be accesible after scope
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
    
    
}

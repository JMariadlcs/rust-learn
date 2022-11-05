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

    // By cloning, s1 would be still be accesible after scope -> EXPENSIVE
    // Used when different things are going on
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // ----- COPY ------

    // In this case x is still valid altough 'clone()' was not used
    // This is because integers are stored on the STACK not on the HEAP
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    let tup: (i32, bool) = (7, true);
    let tup_clone = tup;
    println!("Tups first parameters are tup:{} and tup_clone:{}", tup.1, tup_clone.1);
    
    // More types that can be copied (stored on the stack)
    // u32, bool, f64, char, tuple with only copy types 

    // ----- OWNERSHIP AND FUNCTIONS ------

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                                    // ... and so is no longer valid here

    // println!("As s is strubg it can NOT be still used here, s is: {}", s);

    let x = 5;                 // x comes into scope

    makes_copy(x);     // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
    println!("As x is integer it can be still used here, x is: {}", x);
}   // Here, x goes out of scope, then s. But because s's value was moved, nothing
    // special happens.

// ----- OWNERSHIP AND FUNCTIONS ------

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

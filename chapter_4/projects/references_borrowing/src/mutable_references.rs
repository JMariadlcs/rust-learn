fn main() {

    // ----- MUTABLE REFERENCES -----
    let mut s = String::from("hello");
    println!("The value of s is: {}", s);

    change(&mut s); // Indicates that s will be modified
    println!("The value of s now is: {}", s);


    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; // Error: can NOT BORROW s 2 TIMES AT A TIME
    println!("{}", r1);

    // Multiple borrowing with NOT MUTABLE variables but not with a MUTABLE ONE
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    println!("{}, {}", r1, r2);

    // Combination of mutable and immutable with NO PROBLEMS :)
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

// ----- MUTABLE REFERENCES -----

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
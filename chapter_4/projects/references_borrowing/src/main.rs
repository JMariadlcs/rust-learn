fn main() {
    println!("Hello, references and borrowing!");

    // ------ REFERENCES ------
    let s1 = String::from("hello"); // Created on the HEAP

    let len = calculate_length(&s1); // We are not moving s1, we are referencing it
                                            // We are not taking ownership of s1 so it is still avaliable
    // s1 is still accesible because we have NOT MOVED IT
    println!("The length of '{}' is {}.", s1, len);
}

// ------ REFERENCES ------

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    // s.push_str(", world"); // s can not be modified becasue s is not passed, instead its reference '&' is passed here.
    s.len()
}


fn main() {
    println!("Hello, UTF-8!");

    // MUT STRING
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string(); // converts &str to String
    // the method also works on a literal directly:
    // EQUIVALENT -> let s = "initial contents".to_string();

    // Or directly create a String
    let s = String::from("initial contents");

    // STRING LENGTH CAN BE GROWN
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("String is {}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // PUSH SINGLE CHARACTER
    let mut s = String::from("lo");
    s.push('l');
    println!("String is {}", s);

    // CONCATENATION
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{}", s3);

    // CONCAT + 2
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3; // Always should be first normal and the added '&'
    println!("s final is {}", s);

    // FORMAT let s1 = String::from("tic");
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3); // Works as println! but instead of printing, returns a string
    println!("Returned string is {}", s);

    // SLICING STRINGS
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("string slice is {}", s);

    // ITERATING OVER A STRING
    for c in "Зд".chars() {
        println!("{}", c);
    }

    for b in "Зд".bytes() {
        println!("{}", b);
    }
}

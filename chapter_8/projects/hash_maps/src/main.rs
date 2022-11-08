use std::collections::HashMap;
fn main() {
    println!("Hello, Hash Maps!");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // ACCESSING HASH MAP
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // OWNERSHIP
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // UPDATING A HASH MAP
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // Overwritten
    println!("{:?}", scores);

    // ADDING KEY AND VALUE ONLY IF THEY WERE NOT DEFINED PREVIOUSLY
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // UPDATE A VALUE BASED ON ITS OLD VALUE
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}


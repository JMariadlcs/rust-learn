fn main() {
    println!("Hello, Slice Type!");
    let s = String::from("Testing this program");
    let word = first_word(&s);
    println!("The first word in {} is: {}", s, word);

    let second_word = second_word(&s);
    println!("The second word is {}", second_word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut first_index = 0;
    let mut found_first = false;

    for (i, &item) in bytes.iter().enumerate() { // Iteration over an array using 'iter()'      
                                                            // 'enumerates()' returns a tuple with 'i' and '&item'
        if item == b' ' && !found_first {
            first_index = i + 1; 
            found_first = true;
        } else if item == b' ' && found_first {
            return &s[first_index..i];
        }
    }
    ""
}

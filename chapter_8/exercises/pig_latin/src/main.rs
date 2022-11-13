

fn main() {
    println!("Hello, Pig Latin!");

    let word = String::from("apple");
    let converted_word = convert_to_piglatin(&word);
    println!("{} in piglatin is {}", word, converted_word);

    let word2 = String::from("fire");
    let converted_word2 = convert_to_piglatin(&word2);
    println!("{} in piglatin is {}", word2, converted_word2);
}

fn convert_to_piglatin (word: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let first_letter = word.chars().next();
    match first_letter {
        Some(letter) => { 
            let is_vowel = vowels.contains(&letter);
            if is_vowel { 
                format!("{}-{}", word, "hay")
            } else {
                let (first, last) = word.split_at(1);
                format!("{}-{}{}", last, first, "ay")
            }
        }
        None => { 
            panic!("Error!")
        }
    }
}

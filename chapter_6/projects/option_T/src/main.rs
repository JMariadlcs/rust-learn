fn main() {
    println!("Hello, OptionT!");

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("Results are Some: {:?} and None: {:?}", six, none);

    print_only_six(6);
    print_only_six(2);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x { // Should cover ALL the options
              // In case of Option<>, there are only 2 options
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn print_only_six (x: u8) {
    match x { // COVER ALL MATCHING CASES (use '_' for 'others')
        6 => println!("5"),
        _ => println!(""),
    }
}

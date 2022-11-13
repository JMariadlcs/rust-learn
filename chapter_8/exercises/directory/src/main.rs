use std::{collections::HashMap, io};

fn main() {
    println!("Hello, Directory!");
    directory();
}

fn directory() {
    let mut employee_directory = HashMap::new();

    loop {
        println!("Enter a command type: Add <Person> to <Department>");
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Error creating command");

        let command: &str = command.trim();
        let mut iter = command.split_whitespace();

        let person = match iter.nth(1) {
            Some(p) => p,
            None => {
                println!("Please enter valid command");
                continue;
            }
        };

        let department = match iter.nth(1) { // consumes out 0, 1, 2
            Some(d) => d,
            None => {
                println!("Please enter valid command");
                continue;
            }
        };

        let employees = employee_directory.entry(String::from(department)).or_insert(vec![String::from(person)]);
        employees.push(String::from(person));

        println!("Employee directory {:?}", employee_directory);
    }
}
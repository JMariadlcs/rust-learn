use std::fs::File;
use std::io::ErrorKind;

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    println!("Hello, Error Handling!");

    // ------- PANIC -------

    // To directly exit execution
    // panic!("crash and burn");

    // panic! BACKTRACE
    // execute: 'RUST_BACKTRADE=1 cargo run'
    let v = vec![1, 2, 3];
    // v[99];

    // ------- RESULT -------

    // Execute an action when an error occurs instead of terminating the proccess
    // If success -> greeting_file_result wont contain the file -> it will contain 'Ok()'
    // We need to compute a match for checking Ok and Err
    let greeting_file_result = File::open("hello.txt");
    
    // manage success and error cases
    /* let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    }; */

    let greeting_file_result = File::open("hello.txt");
    // Matching on Different Errors
    /* let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    }; */

    // Check exact type error using 'unwrap_or_else()'
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // ------- UNWRAP() -------
    // If Ok -> returns the value inside Ok
    // If Err -> calls the panic! macro
    // let greeting_file = File::open("hello2.txt").unwrap();

    // ------- EXPECT() -------
    // IF Err -> calls the panic! macro with customized message
    let greeting_file = File::open("hello3.txt")
        .expect("hello3.txt should be included in this project");
}

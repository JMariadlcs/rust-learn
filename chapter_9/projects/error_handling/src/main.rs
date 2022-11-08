fn main() {
    println!("Hello, Error Handling!");


    // PANIC
    // To directly exit execution
    // panic!("crash and burn");

    // panic! BACKTRACE
    // execute: 'RUST_BACKTRADE=1 cargo run'
    let v = vec![1, 2, 3];
    v[99];
}

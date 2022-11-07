

fn main() {
    println!("Hello, Vectors!");

    // DEF OF VECTOR
    let v: Vec<i32> = Vec::new();

    // DEF AND INITIALIZATION OF VECTOR -> Rust sets type
    let v = vec![1, 2, 3]; // all must be same value

    // UPDATE VECTORS
    let mut vec_updatable = Vec::new(); // must be 'mut'
    vec_updatable.push(5);
    println!("Vector is {:?}", vec_updatable);

    let mut v2 = vec![1, 2, 3]; 
    v2.push(4);
    println!("Vector is {:?}", v2);

    // READ VECTOR ELEMENTS
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third { // '.get' returns an Option<> so 'Some' and 'None' must be implemented
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

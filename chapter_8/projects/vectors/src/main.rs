

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

    // READ NON EXISTANT VALUES
    /*let sixth: &i32 = &v[6]; // Out of bounds, of course
    println!("The sixth element is {}", sixth);*/

    let sixth: Option<&i32> = v.get(6);
    match sixth { // '.get' returns an Option<> so 'Some' and 'None' must be implemented
        Some(sixth) => println!("The sixth element is {}", sixth),
        None => println!("There is no sixth element."),
    }

    // MUTABLE AND IMMUTABLE IN SAME SCOPE
    /* This does not work
     let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0]; // Borrowing IMMUTABLE 

    v.push(6); // Pushing a new value might end on having to copy all the values in a new vector 
               // with more memory allocation so, MUTABLE

    println!("The first element is: {}", first);
     */

    // ITERATING VECTORS
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // ITERAING ON MUT REFERENCES OF VECTOS
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; 
        // * derefences a pointer '&'
        println!("{}", i);
    }

    // STORING ENUMS IN VECTORS (to store diff types of elements)
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("Elements in row are {:?}, {:?}, {:?}", row[0], row[1], row[2]);
}

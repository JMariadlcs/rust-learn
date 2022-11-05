fn main() {

    // FLOATING NUMBERS
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("Your numbers are 'x': {} and 'y': {}", x, y);

    // NUMERIC OPERATIONS
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    // BOOLEAN
    let t = true;
    let f: bool = false; // with explicit type annotation

    // CHARACTER TYPE
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // ------- COMPOUND TYPES -------

    // TUPLE TYPE
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // ARRAY TYPE
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // type and num of elements

    let a = [3; 5]; // Array of 5 elements with ALL of them value 3!

    let a = [1, 2, 3, 4, 5];

    let first = a[0]; // access to its elements
    let second = a[1];
}

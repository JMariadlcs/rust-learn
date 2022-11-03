fn main() {
    // let x = 5; // by default variables are immutable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // SHADOWING

    let y = 5;

    let y = y + 1; // redefine same immutable variable shadows previous one

    {
        let y = y * 2;
        println!("The value of x in the inner scope is: {y}");
    }

    println!("The value of x is: {y}");
}

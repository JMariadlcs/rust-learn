// STRUCTS
#[derive(Debug)] // DERIVED TRAITS
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    println!("Hello, Rectangles!");
    
    // TUPLES 
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    // STRUCTS
    let rect_struct = Rectangle {
        width: 30,
        height: 50,
    };
    let area: u32 = area_struct(&rect_struct);  // With '&' becase we dont wannt to take ownership 
    println!( "The area of the rectangle is {} square pixels.", area);

    // DERIVED TRAITS
    println!("Rectangle struct is: {:?} ", rect_struct);
    dbg!(&rect_struct); // Debug function!

}

// TUPLES
fn area_tuple(dimesions: (u32, u32)) -> u32 {
    dimesions.0 * dimesions.1
}

// STRUCTS
fn area_struct(rectangle: &Rectangle) -> u32 { // With '&' becase we dont wannt to take ownership 
    rectangle.width * rectangle.height
}
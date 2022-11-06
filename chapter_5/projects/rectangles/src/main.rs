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

    // METHODS
    impl Rectangle {
        fn rect_area(&self) -> u32 {
            self.width * self.height
        }
    }
    println!( "The area of the rectangle using METHODS is {} square pixels.", rect_struct.rect_area());
    
    impl Rectangle {
        fn width(&self) -> bool {
            self.width > 0
        }
        
        fn can_hold(&self, other_rect: &Rectangle) -> bool {
            self.width > other_rect.width && self.height > other_rect.height
        }

        // ASSOCIATED FUNCTIONS
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }
    
    if rect_struct.width() {
        println!("The rectangle has a nonzero width; it is {}", rect_struct.width);
    }

    // METHODS WITH MORE PARAMETERS
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // ASSOCIATED FUNCTIONS
    let hip_to_be_squared = Rectangle::square(5);
    println!("hip_to_be_squared {:?}", hip_to_be_squared)



}

// TUPLES
fn area_tuple(dimesions: (u32, u32)) -> u32 {
    dimesions.0 * dimesions.1
}

// STRUCTS
fn area_struct(rectangle: &Rectangle) -> u32 { // With '&' becase we dont wannt to take ownership 
    rectangle.width * rectangle.height
}
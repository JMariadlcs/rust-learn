// ---- STRUCTS -----

struct Point<T> {
    x: T,
    y: T,
}

// Different types
struct Point2<T, U> {
    x: T,
    y: U,
}

struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

// ---- ENUMS -----
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// ---- IN METHODS DEF -----
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<X1, Y1> Point3<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {

    // ---- STRUCTS -----

    println!("Hello, Data Types in Structs and Enums!");

    // Generic Point<T>
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    // Wont work be are using i32 and f64
    // let wont_work = Point { x: 5, y: 4.0 };

    // 2 data types in same struct
    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };

    // ---- IN METHODS DEF -----
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

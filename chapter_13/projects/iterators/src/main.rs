fn main() {
    println!("Hello, Iterators!");

    // Simple Iteration
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
    
    println!("{:?}", v1_iter);
    for val in v1_iter {
        println!("Got: {}", val);
    }

    // Iterators that produce another iterations
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); // collect consume the new iterator and create a new vector

    assert_eq!(v2, vec![2, 3, 4]);

}

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s // As s is going out of scope after the function finished
       // It can not be returned a reference to it -> ITS OUT OF SCOPE!
}
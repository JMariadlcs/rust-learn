fn main() {
    let reference_to_nothing = dangle();
    let reference_to_something = no_dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s // As s is going out of scope after the function finished
       // It can not be returned a reference to it -> ITS OUT OF SCOPE!
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s // We are returning s not a reference to s. So it works!
}
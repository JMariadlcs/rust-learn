use std::io;

fn main() {
    println!("Hello, temp converter!");
    println!("Plase input farenheit.");

    let mut farenheit = String::new();
    
    io::stdin().read_line(&mut farenheit).expect("Failed to read line");

    let farenheit: f64 = match farenheit.trim().parse() {
        Ok(num) => num,
        Err(_) => {panic!("Please input a number");}
    };

    let cs: f64 = farenheit_to_cs(farenheit);
    println!("The temp for {} farenheit is {} cs", farenheit, cs);
    let farenheit_2: f64 = cs_to_farenheit(cs);
    println!("The temp for {} cs is {} farenheit", cs, farenheit_2);
}

fn farenheit_to_cs(f: f64) -> f64{
    (f - 32.0 ) * (5.0/9.0)
}

fn cs_to_farenheit(c: f64) -> f64 {
    ((c * 9.0)/5.0) + 32.0

}
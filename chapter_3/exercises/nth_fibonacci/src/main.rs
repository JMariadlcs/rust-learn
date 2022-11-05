use std::io;

fn main() {
    println!("Hello, nth fibonacci!");

    println!("Introduce a fibonacci number.");

    let mut n = String::new();
    
    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {panic!("Please input a valid positive number");}
    };

    let fibo_num = nth_fibonacci(n);
    print!("The fibo number of {} is {}", n, fibo_num)
}

fn nth_fibonacci( n:u32 ) -> u32 {
    // Fibonacci of n is (n-2 + n-1)
    if n == 1 {
        return 1;
    } else if n == 0 {
        return 0;
    }
    (nth_fibonacci(n - 1)) + nth_fibonacci(n - 2)
}

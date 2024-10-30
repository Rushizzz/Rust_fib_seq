use std::io;

fn fib_seq( n : u8) -> u8 {
    if n < 2 { n }
    else {
        return fib_seq( n - 1) + fib_seq( n - 2)
    }
}

fn main() {
    let mut input_line = String::new();

    println!("Enter your number");
    let _ = io::stdin().read_line(&mut input_line).unwrap();

    let number : u8 = input_line.trim().parse().unwrap();
    println!("Fibonacci_seq({})={}", number, fib_seq(number));
}

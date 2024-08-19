use std::io;

fn main() {
    let mut input = String::new();
    println!("Please enter n: ");
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    let n: u32 = input.trim().parse().expect("Invalid input.");

    println!("The {}th Fibonacci number is {}!", n, fibonacci(n));

}

fn fibonacci(n: u32) -> u32 {
    if n <= 1{
        return n;
    }
    let (mut a, mut b) = (0, 1);
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

use std::io;

fn main() {
    println!("Enter the n-th number of fibonacci you want to calculate");
    let mut nth_number = String::new();
    
    io::stdin()
    .read_line(&mut nth_number)
    .expect("Failed to read line"); 

    let nth_number : u64 = nth_number.trim().parse().expect("Please type a positive number!");
    let fibonacci_number = fib(nth_number);
    println!("The {nth_number} of fibonacci sequence is {fibonacci_number}")
}

fn fib(nth_number: u64) -> u64 {
    if nth_number <= 0 {
        return 0;
    } else if nth_number == 1{
        return 1;
    }
    fib(nth_number-1) + fib(nth_number-2)
}
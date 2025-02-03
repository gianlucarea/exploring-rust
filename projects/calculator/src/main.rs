use std::io;

fn main() {
    println!("Enter your choice below - only number");
    println!("Your Options are:");
    println!("1. add");
    println!("2. subtract");
    println!("3. multiply");
    println!("4. divide");
    println!("5. remainder");
    let mut input: String = String::new();

    //Parse operation choice;    
    let _ = io::stdin().read_line(&mut input);
    let operation : i32 = input.trim().parse().unwrap();
    
    //Get numbers
    println!("Enter the first number: ");
    let mut digit_1  = String::new();
    std::io::stdin().read_line(&mut digit_1).unwrap();
    let number_1: i32 = digit_1.trim().parse().unwrap();

    println!("Enter the second number: ");
    let mut digit_2  = String::new();
    std::io::stdin().read_line(&mut digit_2).unwrap();
    let number_2: i32 = digit_2.trim().parse().unwrap();

    //Do Op
   let result = match operation {
        1 => add(number_1,number_2),
        2 => subtract(number_1,number_2),
        3 => multiply(number_1,number_2),
        4 => division(number_1,number_2),
        5 => remainder(number_1,number_2),
        _ => {panic!("Wrong choice of operation")},
    };

    println!("The result is {result}");
}

fn add(x: i32, y:i32) -> i32 {
    x + y
}

fn subtract(x: i32, y:i32) -> i32 {
    x - y
}

fn multiply(x: i32, y:i32) -> i32 {
    x * y
}

fn division(x: i32, y:i32) -> i32 {
    x / y
}

fn remainder(x: i32, y:i32) -> i32 {
    x % y
}
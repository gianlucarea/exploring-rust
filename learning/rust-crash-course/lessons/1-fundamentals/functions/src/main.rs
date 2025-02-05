fn main() {
    let x = do_something(10, 12);
    println!("Value is {}", x);
}

fn do_something(x: u32, y: u32) -> u32 {
    x * y
}
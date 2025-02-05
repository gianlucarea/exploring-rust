struct RedFox{}

impl RedFox{
    fn new() -> Self{
        Self {}
    }
}

//Traits defined required behavior
trait Noisy {
    fn get_noise(&self) -> &str;
}

//Implementing Noisy trait for RedFox class
impl Noisy for RedFox {
    fn get_noise(&self) -> &str {"Meow?"}
}

//Implementing Noisy trait for u8
impl Noisy for u8 {
    fn get_noise(&self) -> &str {"BYTE!"}
}

// General function that works for anyone who impl the trait Noisy
fn println_noise<T: Noisy>(item: T){
    println!("{}", item.get_noise());
}

fn main() {
    let fox = RedFox::new();
    let value : u8 = 10;
    println_noise(fox);
    println_noise(value);
}

use module::greeting; // importing 
use rand::Rng;

fn main() {
    
    // module::greet(); Not working because greet is private
    module::greeting(); //Works because greeting is public - Absolute path
    greeting(); // Relative path using use

    //Added crate 
    let x = rand::thread_rng().gen_range(1..=100);
    println!("{}", x);
}

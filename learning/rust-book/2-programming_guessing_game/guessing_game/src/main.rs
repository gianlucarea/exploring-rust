use std::io; //Input output library
use std::cmp::Ordering; //Ordering gives us Less Greater Equal
use rand::Rng; // Random library 

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100); //Generate a number between 1 and 100

    loop{
        println!("Please input your guess!");
        //Create a variable mutable
        let mut guess = String::new(); 
        
        //Read line and put the value into guess mutable variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); 

        //Transform guess into a unsigned 32 bit integer, 
        //If parse return Ok assign number else continue with next guess
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        }; 
        
        println!("You guessed: {guess}");
        
        //Pattern match the result of cmp between guess and secret_number
        //If equal print and break the loop
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

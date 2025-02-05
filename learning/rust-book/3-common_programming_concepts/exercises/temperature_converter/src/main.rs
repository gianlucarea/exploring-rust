use std::io; //Input output library

fn main() {
    println!("Enter the temperature unit: Fahrenheit(Enter 0), Celsius(Enter 1)");
    let mut temperature_unit = String::new();
    
    io::stdin()
    .read_line(&mut temperature_unit)
    .expect("Failed to read line"); 

    let temperature_unit : i8 = temperature_unit.trim().parse().expect("Please type a number!");

    if temperature_unit == 0 {
        convert_to_celsius();
    } else if temperature_unit == 1 {
        convert_to_fahrenheit();
    } else {
        println!("You have to choose between 0 and 1");
    }
}

fn convert_to_celsius(){
    println!("Enter the temperature in fahrenheit unit");
    let mut temperature_f = String::new() ;
    
    io::stdin()
    .read_line(&mut temperature_f)
    .expect("Failed to read line"); 

    let temperature_f: f64 = temperature_f.trim().parse().expect("Please type a number!");
    let temperature_c: f64 = {((temperature_f - 32.0) * 5.0)/ 9.0};
    println!("{temperature_c}")
}

fn convert_to_fahrenheit(){
    println!("Enter the temperature in celsius unit");
    let mut temperature_c = String::new() ;

    io::stdin()
    .read_line(&mut temperature_c)
    .expect("Failed to read line"); 

    let temperature_c: f64 = temperature_c.trim().parse().expect("Please type a number!");
    let temperature_f: f64 = {((temperature_c  * 9.0) / 5.0) + 32.0};
    println!("{temperature_f}")
}

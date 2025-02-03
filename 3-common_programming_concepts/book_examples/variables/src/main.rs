fn main() {

    let _x : u8 = 5; 
    let x : u8 = 6; // Unsigned 8 bit 
    {
        let x : i32 = -1_000; //Signed 32 bit with _ to make it more readable
        println!("{x}"); //Prints 99999
    }
    println!("{x}"); //Prints 6

    //Basic Math Operations
    //addition 
    let sum = 5 + 10;
    println!("Sum of 5 and 10 is {sum}"); //Prints 15
    //subtraction
    let difference = 10 - 5;
    println!("Diff between 10 and 5 is {difference}"); //Prints 5
    //multiplication
    let product = 10 * 5;
    println!("Product of 10 and 5 is {product}"); //Prints 50
    //division
    let quotient = 56.7 / 32.2;
    println!("Quotient between 56.7 and 32.2 is {quotient}"); //Prints 1.760...
    let truncated = -5 / 3;
    println!("Truncated between -5 and 3 is {truncated}"); //Prints -1
    //remainder
    let remainder = 43 % 5; 
    println!("The remainder between 43 and 5 is {remainder}") //Prints 3

    //BOOLEANS
    let t = true; //Simple
    let f : bool = false; //Inferring type
    
    //CHAR
    let letter_t = 't';
    let letter_z : char = 'z' 
}

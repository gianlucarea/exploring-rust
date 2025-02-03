fn main() {
    
    //TUPLES
    let tup: (i8,u8,f32,char) = (-10,255,-1.4,'z');
    // Option 1 - Pattern matching to associate value of the tuple to single variable
    let (x,y,_z,_k) = tup; 
    println!("The first value of the tuple is {x}"); //Print -10
    println!("The second value of the tuple is {y}"); // Print 255
    // Option 2 - Assign with . operator
    let z = tup.2;
    let k = tup.3;
    println!("The third value of the tuple is {z}"); // Print -1.4
    println!("The fourth value of the tuple is {k}"); //Print k

    //ARRAYS
    let array_a = [1,2,3,4,5]; // In Rust the arrays are static
    let _array_b : [i32;5] = [1,2,3,4,-5]; // Initialize an array of five elements of i32 type 
    let _array_c = [100; 5]; //Initialize an array of five elements all with value 100
    let first_value = array_a[0];
    println!("First element of array_a is {first_value}");

}

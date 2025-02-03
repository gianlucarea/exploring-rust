fn main() {
    let number = 3;
    let condition = true;

    // IF 
    if number < 5 {
        println!("Less than five");
    }else if number == 5{
        println!("The number is five");
    }else {
        println!("Greater than five");
    }

    let number = if condition {10} else {0};
    println!("{number}");
    
    // LOOP
    let mut counter = 0;
    let result = loop {
        counter +=1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result after the loop is {result}");

    // WHILE
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("While has finished");

    // FOR
    let a = [10,20,30,40,50];
    for element in a {
        println!("the value is: {element}");
    }
}

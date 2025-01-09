fn main() {
    println!("Hello, world!");
    another_function();
    measurement_function(100 , 'm');
    let x = function_with_return_value(5);
    println!("x is {x}")
}

fn another_function(){
    println!("Another Function!");
}

fn measurement_function(value: i8, unit: char){
    println!("The measurements is: {value}{unit}");
}

fn function_with_return_value(x: i8) -> i8 {
    x + 1
}
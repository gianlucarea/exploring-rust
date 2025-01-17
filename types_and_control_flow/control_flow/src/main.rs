fn main() {
    let num = 4;
    let msg = if num == 5 {
        "five"
    } else if num == 4 {
        "four"
    } else {
        "other"
    };
    println!("{}", msg);

    // Ternary x = a ? b : c
    let a = true;
    let x = if a {10} else {12};
    println!("{x}");

    //Unconditional loop
    loop {
        break; //Exit the loop
    }

    //Nested unconditional loop 
    'bob: loop{
        loop{
            loop{
                break 'bob;
            }
        }
    }

    //Continue is similar to break 
    let mut x = 9;
    while x < 10 {
        println!("{x}");
        x += 1;
    }

    //A do while 
    let mut y = 0;
    loop{
        y = y+1;
        println!("{y}");
        if y == 1{ break }
    }

    //FOR loop
    println!("SIMPLE ARRAY FOR LOOP");
    for num in [7,8,9].iter(){
        println!("{num}");
    }

    let array = [(1,2),(3,4)];
    println!("ARRAY OF TUPLES FOR LOOP");
    for (x,y) in array.iter(){
        println!("{x} and {y}");
    }
    // Counts to 0 to 4 
    // if we put 0..=5 counts to 5 included
    println!("ARRAY RANGE FOR LOOP");
    for num in 0..5{
        println!("{num}");
    }
}

fn main() {
    //Closures are anonymous functions
    let add = |x, y| { x + y };
    let z = add(1,2);
    println!("{:?}",z);

    //Empty closure
    // ||{}
    
    // Closure will borrow a reference to values in the enclosing scope
    let burger = "🍔".to_string();
    let f = || {println!("{}",burger);};
    f();

    // force closure move
    // let f = move || {println!("{}",burger);};

    //An example of using a closures 
    let mut v = vec![2,4,6];
    let solution  = v.iter().map(|x| x * 3).filter(|x| *x > 10).fold(0, |acc,x| acc + x);
    
    println!("{:?}", solution);
}

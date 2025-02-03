fn main() {
    let x = 5;
    let shadow = 10;
    {
        let y = 99;
        let shadow = 99; 
        println!("{},{},{}",x,y,shadow);
    }
    println!("{},{}",x,shadow);

    //Transform mutable to immutable
    let mut z = 5;
    let z = z;
    println!("{}",z);
    let k = 10;
    let mut k = k; 
    k = 100;
    println!("{}",k);
}

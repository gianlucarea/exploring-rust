fn main() {
    //TUPLE - Max arity is 12!!!!
    let info = (1,3.3,999);
    let info1 : (i32,i32) = (2,3);
    let first_info = info1.1;
    let (jets,fuel,ammo) = info;
    println!("{},{},{},{}",first_info,jets,fuel,ammo);
    
    //ARRAY - Max size 32
    let buf1 = [30,1,2];
    let buf2 = [0; 3]; // [0,0,0]
    let buf3: [u8;3] = [1,2,3];
    let x = buf2[0];
    println!("{},{}",x,buf1[0]);
}

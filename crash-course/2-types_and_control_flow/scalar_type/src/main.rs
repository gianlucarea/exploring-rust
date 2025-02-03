fn main() {
    let value1 : u32 = 1000000;
    let value2 : u32 = 1_000_000;
    let value3 = 1000000_u32;
    println!("{}", if(value1 == value2){true} else {false});
    println!("{}", if(value1 == value3){true} else {false});
    println!("{}", if(value2 == value3){true} else {false});

    let value4 : f32 = 1.34;
    let value5 = 1.34_f32;
    let value6 = 1.34f32;
    println!("{}", if(value4 == value5){true} else {false});
    println!("{}", if(value4 == value6){true} else {false});
    println!("{}", if(value5 == value6){true} else {false});

    let letter : char = 'a';
    let rocket : char = '🚀';
    println!("{}", letter);
    println!("{}", rocket);
}

#[derive(Debug)]
struct Rectangle {
    width : u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    //Getters
    fn width(&self) -> u32 {
        self.width 
    }
    fn height(&self) -> u32 {
        self.height
    }

    //Kind of Constructor 
    fn square(size: u32) -> Self{
        Self {
            width : size,
            height: size,
        }
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    let rectangle = (30,40);
    let rectangle_from_struct = Rectangle{
        width: 30,
        height: 60,
    };
    println!("The area of the rectangle simple is {0}", area(width1,height1));
    println!("The area of the rectangle using tuple is {0}", area_tuple(rectangle));
    println!("The area of the rectangle using struct is {0}", area_struct(&rectangle_from_struct));
    println!("The area of the rectangle using struct and impl {0}", rectangle_from_struct.area());
    println!("Printing a struct using :? {:?}", rectangle_from_struct);
    println!("Printing a struct using :#? {:#?}", rectangle_from_struct);
    println!("The width of the rectangle using struct and impl using getters {0}", rectangle_from_struct.width());
    println!("The height of the rectangle using struct and impl using getters {0}", rectangle_from_struct.height());

    //Check if a rectangle can hold another one
    let rectangle1 = Rectangle{
        width: 30,
        height: 60,
    };
    let rectangle2 = Rectangle{
        width: 10,
        height: 20,
    };
    println!("Can rectangle1 hold rectangle2? {}", rectangle1.can_hold(&rectangle2));
    println!("Can rectangle2 hold rectangle1? {}", rectangle2.can_hold(&rectangle1));
    
    let square = Rectangle::square(10);
    println!("Declaring a square {:#?}", square);
}

//Simple
fn area(width : u32, height: u32) -> u32 {
    width * height
}

//With tuple but not clear which is width and which is height 
fn area_tuple(dimensions: (u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}

//With struct
fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
} 

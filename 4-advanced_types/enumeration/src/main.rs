enum Color {
    Red,
    Green,
    Blue,
}

enum Rgb {
    White(u8,u8,u8),
    Black(u8,u8,u8),
}

fn main() {
   let color = Color::Red;
   let rgb = Rgb::White(0,0,0);
}

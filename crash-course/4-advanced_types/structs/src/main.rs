struct RedFox{
    enemy: bool,
    life: u8,
}

//Impl block
impl RedFox {
    //Constructor example - associated function * no param* 
    //Self is the same as using RedFox
    fn new() -> Self{
        Self {
            enemy: true,
            life: 70,
        }
    }

    //methods
    fn add(&mut self){
        self.life = self.life + 1;
    }
    
}
fn main() {
    //Basic instantiate 
    let fox = RedFox {
        enemy: true,
        life: 70,
    };
    //Using a Constructor 
   let mut fox1 = RedFox::new();
   //Method call
   RedFox::add(&mut fox1);
   println!("{}",fox1.life);
}

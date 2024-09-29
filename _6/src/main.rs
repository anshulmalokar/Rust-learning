struct Rect{
    width: u32,
    height: u32
}

impl Rect{
    // object level function
    fn area(&self) -> u32{
        return self.width * self.height;
    }
    // object level function
    fn perimeter(&self) -> u32 {
        let perimeter: u32 = 2 * (self.width * self.height);
        return perimeter;
    }
    // struct level function
    // similar to static function in java/javascript
    fn debug(){
        println!("This is a debug message");
    }
}


fn main() {
    let rect: Rect = Rect{
        width: 10,
        height: 10
    };
    println!("{}",rect.area());
    println!("{}",rect.perimeter());
    Rect::debug();

}



// structs are more closer to classes
// To implement classes we will be using structs


trait Summary{
    fn summarize(&self){
        println!("Summarizing:");
    }
}

trait Fix{
    fn fix(&self){
        println!("Fixing:");
    }
}

struct User{
    name: String,
    age: u32
}

impl Summary for User{}
impl Fix for User{}


fn main() {
    let user = User{
        name: String::from("Anshul Malokar"),
        age: 32
    };

    print_summary(&user);
}

fn print_summary<T: Summary + Fix>(user: &T){
    println!("{:?}", user.summarize());
    println!("{:?}",user.fix());
}
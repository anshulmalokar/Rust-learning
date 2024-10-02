
trait Summary{
    fn summarize(&self){
        println!("Summarizing");
    }
}

struct User{
    name: String,
    age: u32
}

impl Summary for User{
    fn summarize(&self) {
        println!("{}",format!("Name is {} and age is {}", self.name, self.age));
    }
}

impl User{
    fn print_user(&self) {
        println!("{}",format!("Name is {} and age is {}", self.name, self.age));
    }
}

fn main() {
    let user = User{
        name: String::from("Hello"),
        age: 32
    };
    user.summarize();
    user.print_user();
}



// Traits in rust are similar to interfaces in java
// A product can implement this traits
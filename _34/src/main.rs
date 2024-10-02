
// Missing Life Time Specifier
// struct User  {
//     name: &str
// }

// Now whenever the name goes out of scope so does the User
struct User<'a>{
    name: &'a str
}

fn main(){
    let user;
    {
        let name = String::from("Proful");
        user = User {
            name: &name
        };
    }
    println!("{}", user.name);
}
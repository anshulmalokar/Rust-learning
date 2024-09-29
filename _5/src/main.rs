struct User{
    active: bool,
    user_name: String,
    email: String,
    sign_in_count: String
}

fn main() {
    let user1: User = User{
        active: true,
        user_name: String::from("Anshul"),
        email: String::from("anshulmalokar@gmail.com"),
        sign_in_count: String::from("1")
    };
    println!("{} is the name",user1.user_name);
}
// Learning Structs
// Similar to types in typeScript
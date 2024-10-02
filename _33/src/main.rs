fn main() {
    let str1 = String::from("hell");
    let longest_string;
    {
        let str2 = String::from("world");
        longest_string = longest(&str1,&str2);
    }
    println!("{}",longest_string);
    println!("{}","longest_string");
}

// Missing lifetime specifier
fn longest<'a>(str1: &'a str,str2: &'a str) -> &'a str{
    if str1.len() > str2.len() {
        return str1;
    }
    return str2;
}
// fn longest(str1:String,str2:String) -> String{
//     if str1.len() > str2.len() {
//         return str1;
//     }
//     return str2;
// }

// Lifetimes in Rust
// Rust compilier will ask for the lefetime of the paramaters
// If the compilier was not complaining then there would be a dangaling pointer

// Generic lifetime annotation

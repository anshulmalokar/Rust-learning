fn main(){
    let my_string: String = String::from("Hello");
    let len:usize = calculate_length(&my_string);
    println!("{}",len);
}

fn calculate_length(str: &str) -> usize{
    return str.chars().count();
}


// Learnt passing by address
// calculating the number of characters for a function
// str.chars().count();
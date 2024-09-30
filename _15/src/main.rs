fn main() {
    let mut s1: String = String::from("Hello"); 
    print_str(&s1);
}

fn print_str(s1: &String){
    println!("{}",s1);
}

// Borrowing and References
// take the address of s1 and do the read operations and whatever you want to do with it
// This is cleaner than passing on the ownership

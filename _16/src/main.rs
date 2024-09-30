fn main() {
    let mut s1: String = String::from("Hello");
    
    // immutable references
    let s2 = &s1;
    let s3 = &s1;
    
    // giving mutable reference
    do_something(&mut s1);
    println!("{}",s1);
}

fn do_something(str:&mut String){
    str.push_str("a");
}

// Ownership borrowing rule
// At any given time have one mutable reference
// Or any number of immutable references

// To ensure memory saftey
    // Multupe immutable references possible.
    // No mutable reference can be there when any immutable references can be there.
fn main() {
    let s1: String = String::from("Hello");
    {
        let s2 = s1;
        println!("{}", s2);
    }
    // This will be a dangaling pointer
    // That's why only one owner is there in Rust
    println!("{}", s1);
}

// Ownership 
// This means that only one owner will be there at a time.
// Owner in the line 3 becomes s2 and s1 will be invalid

// Rust invalidates the first variable
// a1 
// a1 moved to a2
// a1 will be no longer valid

// Moving
// The error says we are trying to borrow a moved value
// Now the new owner is s2 not s1


// You can clone the data 
// Have two data in the heap
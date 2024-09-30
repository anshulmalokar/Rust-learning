fn main() {
    create_string();
}

fn create_string(){
    let s: String = String::from("Hello world");
    println!("{}", s);
}

// Each value on the heap have a owner
// Whenever this variable goes out of scope this variable will be cleaned
// Only one owner of the heap data present in the stack
// So no dangaling pointers and no memory leak will be here
// Very similar to garbage collection
    // Garbage collector will clear the memory in time intervals
// Rust will not allow to have a garbage collector.
// Rust we're cleaning the corresponding data
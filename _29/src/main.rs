fn main() {
    println!("Hello, world!");
    let name = String::from("James Bond 007");
    // This is stored in memory
    // name in stack
    // references contigious segments of collection J a m e s

    // These literals are literally stored in the final binary
    // as 0's and 1's
    // In the final build file
    // This str is pointing to somewhere in the binray
    let str = "Hello World";
    println!("Hello, {}!", str);

    for val in name.chars(){
        if val == ' '{
            break;
        }
        print!("{}",val);
    }
    println!("");

    let slice = &name[0..6];
    println!("{}", slice);
}
// String vs Slices.
// String is growable,mutable,owned UTF-8 encoded.
// &str.
// &str is normally reference and doesnot pass the ownership.
// slices let you reference contigious set of elements in a collection.
// String slice is into the view on the contigious set of elements
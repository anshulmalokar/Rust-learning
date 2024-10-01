fn main() {
    let vec = vec![1,2,3];
    let iter = vec.iter();

    for val in iter{
        println!("{}", val);
    }

    println!("{:?}",vec);
}

// Iterators in rust
// Collections of your own then you will need to know more about this 
// iterator
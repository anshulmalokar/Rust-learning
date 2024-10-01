fn main() {
    let mut vec = vec![1,2,3];
    let itr = vec.iter_mut();
    for val in itr{
        *val = *val + 1;
    }
    println!("{:?}", vec);
}


// Mutable iterato